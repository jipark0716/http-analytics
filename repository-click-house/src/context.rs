use crate::event::Event;
use crate::session::Session;
use clickhouse::{Client, Row};
use serde::Serialize;
use std::mem;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct DbContext {
    client: Arc<Client>,
    pub insert_sessions: Arc<Mutex<InsertBuffer<Session>>>,
    pub insert_event: Arc<Mutex<InsertBuffer<Event>>>,
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
            insert_sessions: InsertBuffer::<Session>::new(client.clone(), "session", 10000),
            insert_event: InsertBuffer::<Event>::new(client.clone(), "event", 10000),
        })
    }
}

pub struct InsertBuffer<T> {
    client: Arc<Client>,
    table: &'static str,
    buffer1: Vec<T>,
    buffer2: Vec<T>,
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
            buffer1: Vec::with_capacity(batch_size),
            buffer2: Vec::with_capacity(batch_size),
            batch_size,
            last_flush: Instant::now(),
        }));

        Self::spawn_flusher(this.clone());

        this
    }

    pub async fn push(buffer: Arc<Mutex<Self>>, row: T) -> anyhow::Result<()> {
        let mut this = buffer.lock().await;
        this.buffer1.push(row);

        if this.buffer1.len() >= this.batch_size {
            if let Err(e) = this.flush_locked().await {
                eprintln!("flush error: {e}");
            }
        }
        Ok(())
    }

    async fn flush_locked(&mut self) -> anyhow::Result<()> {
        if self.buffer1.is_empty() {
            return Ok(());
        }

        mem::swap(&mut self.buffer1, &mut self.buffer2);

        let mut insert = self.client.insert(self.table)?;
        for row in mem::take(&mut self.buffer2) {
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
                let should_flush = {
                    let this = buffer.lock().await;
                    this.last_flush.elapsed() >= Duration::from_secs(60)
                };

                if should_flush {
                    let mut this = buffer.lock().await;
                    let _ = this.flush_locked().await; // 내부에서 락 짧게만 잡히도록 설계
                }
            }
        });
    }
}
