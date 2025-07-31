use ai_client::text::gemini;
use ai_client::text::text::{AiClient, Prompt, Role};
use config::analyze::HttpAnalyzeConfig;
use config::import;

#[cfg(feature = "development")]
static CONFIG_BIN: &[u8] = include_bytes!("../config/development.bin");

#[cfg(feature = "development")]
static PROMPT: &str = include_str!("../prompt/development.txt");

static SCHEMA: &str = include_str!("../prompt/schema.json");

#[tokio::main]
async fn main() {
    let config = import::<HttpAnalyzeConfig>(CONFIG_BIN);

    let client = gemini::GeminiClient::new(config.ai.clone());

    let response = client.generate_text(vec![
        Prompt {
            role: Role::System,
            text: format!("{PROMPT}{SCHEMA}"),
        },
        Prompt {
            role: Role::System,
            text: "이 사용자의 client_id 는 1 이고, 오늘 날짜는 2025-07-30 타임존은 +9 날짜 조건은 로컬 자정 기준으로만 작성해".to_string(),
        },
        Prompt {
            role: Role::User,
            text: "최근 2일전부터 오늘까지 1234 상품을 본 사람을 알고 싶어".to_string(),
            // text: "오늘 주문서는 작성했지만 주문은 하지 않은 사람을 알고 싶어".to_string(),
        },
    ])
        .await
        .map_err(|e| format!("잼민이가 죽었다 {e}"))
        .unwrap();

    println!("{:?}", response);
}
