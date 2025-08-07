use std::error::Error;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Text {
    pub text: String,
}

pub struct Prompt {
    pub role: Role,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    User,
    System,
}

#[async_trait]
pub trait AiClient : Send + Sync {
    async fn generate_text(&self, prompt: Vec<Prompt>) -> Result<Text, Box<dyn Error>>;
}
