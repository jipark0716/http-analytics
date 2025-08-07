pub mod response;

pub async fn not_found() -> response::SimpleResponse {
    response::NOT_FOUND_RESPONSE
}