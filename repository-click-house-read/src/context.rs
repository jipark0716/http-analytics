use std::sync::Arc;
use config::DatabaseConfig;
use reqwest::Client;

pub struct DbContext {
    client: Client,
    config: Arc<DatabaseConfig>,
}

impl DbContext {
    pub fn new(config: Arc<DatabaseConfig>) -> Arc<Self> {
        Arc::new(Self {
            client: Client::new(),
            config,
        })
    }

    pub async fn query(&self, query: String) -> Result<String, reqwest::Error> {
        let response = self.client.post(format!("{}?database={}", &self.config.host, &self.config.database))
            .basic_auth(&self.config.user, Some(&self.config.password))
            .body(query)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}