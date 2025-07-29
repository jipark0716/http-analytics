use gemini_client_api::gemini::ask::Gemini;
use gemini_client_api::gemini::types::sessions::Session;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut session = Session::new(10);
    let api = Gemini::new(std::env::var("GEMINI_API_KEY")?, "gemini-1.5-flash", None);

    let user_input = "최근 로그인한 사람들의 전화번호를 쿼리로 알려줘";
    let response = api.ask(session.ask_string(user_input)).await?;
    println!("LLM 응답:\n{}", response.get_text(""));

    Ok(())
}