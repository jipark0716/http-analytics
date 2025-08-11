use crate::text::text;
use crate::text::text::{Prompt, Text};
use ai_client::text::text::AiClient;
use async_trait::async_trait;
use config::Ai;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Role {
    User,
    Model,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    parts: Vec<Part>,
    role: Role,
}

#[derive(Serialize)]
struct GenerateContentRequest {
    system_instruction: Content,
    contents: Vec<Content>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
    content: Content,
}

pub struct GeminiClient {
    key: &'static str,
    engine: &'static str,
    client: Client,
}

impl GeminiClient {
    pub fn new(config: &Ai<'static>) -> Arc<Self> {
        Arc::new(Self {
            key: config.api_key,
            engine: config.engine,
            client: Client::new(),
        })
    }
}

#[async_trait]
impl AiClient for GeminiClient {
    async fn generate_text(&self, prompt: Vec<Prompt>) -> Result<Text, Box<dyn Error>> {
        let (system_parts, user_parts): (Vec<_>, Vec<_>) = prompt
            .into_iter()
            .partition(|o| o.role == text::Role::System);

        let systems: Vec<Part> = system_parts
            .into_iter()
            .map(|o| Part { text: o.text })
            .collect();

        let user: Vec<Content> = user_parts
            .into_iter()
            .map(|o| Content {
                role: Role::User,
                parts: vec![Part { text: o.text }],
            })
            .collect();

        let req = GenerateContentRequest {
            system_instruction: Content {
                role: Role::User,
                parts: systems,
            },
            contents: user,
        };

        let response = self
            .client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                &self.engine
            ))
            .header("x-goog-api-key", self.key)
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await?;

        let body: GeminiResponse = response.json().await.unwrap();

        Ok(Text {
            text: body
                .candidates
                .first()
                .unwrap()
                .content
                .parts
                .first()
                .unwrap()
                .text
                .clone(),
        })
    }
}
