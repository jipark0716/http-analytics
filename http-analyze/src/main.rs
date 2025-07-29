use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    User,
}

#[derive(Serialize)]
struct Content {
    role: Role,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct GenerateContentRequest {
    system_instruction: Content,
    contents: Vec<Content>,
}

#[tokio::main]
async fn main() {
    // let api_key = std::env::var("GEMINI_API_KEY").expect("miss env: GEMINI_API_KEY");
    let api_key = "ㅁㄴㅇ".to_string();
    let model = "gemini-2.5-flash-lite".to_string();

    let req = GenerateContentRequest {
        system_instruction: Content {
            role: Role::User,
            parts: vec![Part {
                text: "안녕이라하면 벅벅이라고 해줘".to_string(),
            }],
        },
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: r#"안녕"#.to_string(),
            }],
        }],
    };

    let client = Client::new();
    let res = client
        .post(format!("https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"))
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&req)
        .send()
        .await
        .unwrap();

    let text = res.text().await.unwrap();
    println!("{}", text);
}