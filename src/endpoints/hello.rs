use crate::endpoints::ApiResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloData {
    message: String,
}

#[get("/?<format>")]
pub fn hello(format: Option<String>) -> ApiResponse<HelloData> {
    let msg = "Hello, World!";
    match format.as_deref() {
        Some("json") => ApiResponse::Json(HelloData {
            message: msg.to_string(),
        }),
        _ => ApiResponse::Plain(msg.to_string()),
    }
}
