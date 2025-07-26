use crate::session::Session;
use clickhouse::{Client, Row};
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct DbContext {
    client: Arc<Client>,
    pub insert_sessions: Arc<Mutex<InsertBuffer<Session>>>,
}

impl DbContext {
    pub fn new() -> Arc<Self> {
        let client = Arc::new(
            Client::default()
                .with_url("http://localhost:8123")
                .with_user("admin")
                .with_password("password1234")
                .with_database("event"),
        );

        Arc::new(Self {
            client: client.clone(),
            insert_sessions: InsertBuffer::<Session>::new(client.clone(), "session", 2),
        })
    }
}

pub struct InsertBuffer<T> {
    client: Arc<Client>,
    table: &'static str,
    buffer: Vec<T>,
    batch_size: usize,
    last_flush: Instant,
}

impl<T> InsertBuffer<T>
where
    T: Serialize + Send + Sync + Row + 'static,
{
    pub fn new(client: Arc<Client>, table: &'static str, batch_size: usize) -> Arc<Mutex<Self>> {
        let this = Arc::new(Mutex::new(Self {
            client,
            table,
            buffer: Vec::with_capacity(batch_size),
            batch_size,
            last_flush: Instant::now(),
        }));

        Self::spawn_flusher(this.clone());

        this
    }

    pub async fn push(buffer: Arc<Mutex<Self>>, row: T) -> anyhow::Result<(), Box<dyn Error>> {
        let mut this = buffer.lock().await;
        this.buffer.push(row);

        if this.buffer.len() >= this.batch_size {
            this.flush_locked()
                .await
                .map_err(|e| format!("error lock {e}"))?;
        }
        Ok(())
    }

    async fn flush_locked(&mut self) -> anyhow::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let mut insert = self.client.insert(self.table)?;
        for row in self.buffer.drain(..) {
            insert.write(&row).await?;
        }
        insert.end().await?;
        self.last_flush = Instant::now();
        Ok(())
    }

    fn spawn_flusher(buffer: Arc<Mutex<Self>>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                let mut this = buffer.lock().await;
                if this.last_flush.elapsed() >= Duration::from_secs(60) {
                    let _ = this.flush_locked().await;
                }
            }
        });
    }
}
