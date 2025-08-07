use ai_client::text::text::{AiClient, Prompt, Role};
use async_trait::async_trait;
use chrono::Local;
use std::error::Error;
use std::sync::Arc;

pub enum QueryType {
    None,
    List,
    Group,
}

impl From<char> for QueryType {
    fn from(c: char) -> Self {
        match c {
            'a' | 'A' => Self::List,
            'b' | 'B' => Self::Group,
            _ => Self::None,
        }
    }
}

pub struct Query {
    query_type: QueryType,
    query: String,
}

#[async_trait]
pub trait CreateQueryService : Send + Sync {
    async fn create_query(self, client_id: String, prompt: String) -> anyhow::Result<Query, Box<dyn Error>>;
}

pub struct CreateQueryServiceImpl {
    system_prompt: String,
    ai_client: Arc<dyn AiClient>,
}

impl CreateQueryServiceImpl {
    pub fn new(system_prompt: String, ai_client: Arc<dyn AiClient>) -> Arc<Self> {
        Arc::new(Self {
            system_prompt,
            ai_client,
        })
    }
}

// 회원가입 시작은했는데 완료는 안한사람 비율은 얼만큼이야?
#[async_trait]
impl CreateQueryService for CreateQueryServiceImpl {
    async fn create_query(self, client_id: String, prompt: String) -> anyhow::Result<Query, Box<dyn Error>>
    {
        let response = self.ai_client.generate_text(vec![
            Prompt {
                role: Role::System,
                text: self.system_prompt,
            },
            Prompt {
                role: Role::System,
                text: format!(
                    "이 사용자의 client_id 는 {} 이고, 오늘 날짜는 {} 타임존은 +9 날짜 조건은 로컬 자정 기준으로만 작성해",
                    client_id,
                    Local::now().format("%Y-%m-%d").to_string(),
                ),
            },
            Prompt {
                role: Role::User,
                text: prompt,
                // text: "오늘 주문서는 작성했지만 주문은 하지 않은 사람을 알고 싶어".to_string(),
            },
        ])
            .await?;

        let mut chars = response.text.chars();

        let query_type: QueryType = chars.next().unwrap().into();

        Ok(Query {
            query_type,
            query: chars.collect(),
        })
    }
}