use crate::event::Event;
use crate::session::Session;
use clickhouse::{Client, Row};
use config::DatabaseConfig;
use serde::Serialize;
use std::mem;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct DbContext {
    pub insert_sessions: Arc<InsertBuffer<Session>>,
    pub insert_event: Arc<InsertBuffer<Event>>,
}

impl DbContext {
    pub fn new(config: &DatabaseConfig) -> Arc<Self> {
        let client = Arc::new(
            Client::default()
                .with_url(config.host)
                .with_user(config.user)
                .with_password(config.password)
                .with_database(config.database),
        );

        Arc::new(Self {
            insert_sessions: InsertBuffer::<Session>::new(client.clone(), "session", config.batch_size),
            insert_event: InsertBuffer::<Event>::new(client.clone(), "event", config.batch_size),
        })
    }
}

pub struct InsertBuffer<T> {
    client: Arc<Client>,
    table: &'static str,
    buffer: Mutex<Vec<T>>,
    batch_size: usize,
    last_flush: Mutex<Instant>,
}

impl<T> InsertBuffer<T>
where
    T: Serialize + Send + Sync + Row + 'static,
{
    pub fn new(client: Arc<Client>, table: &'static str, batch_size: usize) -> Arc<Self> {
        let this = Arc::new(Self {
            client,
            table,
            buffer: Mutex::new(Vec::with_capacity(batch_size)),
            batch_size,
            last_flush: Mutex::new(Instant::now()),
        });

        Self::spawn_flusher(this.clone());

        this
    }

    pub async fn push(&self, row: T) -> anyhow::Result<()> {
        let rows_otp: Option<Vec<T>> = {
            let mut locked_buffer = self.buffer.lock().await;

            locked_buffer.push(row);

            if locked_buffer.len() >= self.batch_size {
                Some(mem::take(&mut locked_buffer))
            } else {
                None
            }
        };

        if let Some(rows) = rows_otp {
            println!("flush");
            self.flush(rows).await?;
        }

        Ok(())
    }

    async fn flush_locked(&self) -> anyhow::Result<()> {
        let rows_otp: Option<Vec<T>> = {
            let mut locked_buffer = self.buffer.lock().await;

            if locked_buffer.len() > 0 {
                Some(mem::take(&mut locked_buffer))
            } else {
                None
            }
        };

        if let Some(rows) = rows_otp {
            self.flush(rows).await?;
        }

        Ok(())
    }

    async fn flush(&self, rows: Vec<T>) -> anyhow::Result<()> {
        let mut insert = self.client.insert(self.table)?;

        for row in rows {
            insert.write(&row).await?;
        }

        insert.end().await?;

        {
            *self.last_flush.lock().await = Instant::now();
        }

        Ok(())
    }

    fn spawn_flusher(this: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                let should_flush = {
                    this.last_flush.lock().await.elapsed() >= Duration::from_secs(60)
                };

                if should_flush {
                    println!("batch flush");
                    let _ = this.flush_locked().await; // 내부에서 락 짧게만 잡히도록 설계
                }
            }
        });
    }
}
