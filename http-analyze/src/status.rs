use config::analyze::HttpAnalyzeConfig;
use service_analyze::create_query::{CreateQueryService, CreateQueryServiceImpl};
use std::sync::Arc;
use ai_client::text::gemini::GeminiClient;

#[cfg(feature = "development")]
static PROMPT: &str = include_str!("../prompt/development.txt");

static SCHEMA: &str = include_str!("../prompt/schema.json");

pub struct AppStatus {
    pub create_query_service: Arc<dyn CreateQueryService>,
}

impl AppStatus {
    pub fn new(config: &'static HttpAnalyzeConfig<'static>) -> Self {
        let ai_client = GeminiClient::new(&config.ai);
        let create_query_service = CreateQueryServiceImpl::new(format!("{PROMPT}{SCHEMA}").to_string(), ai_client);
        Self {
            create_query_service,
        }
    }
}
