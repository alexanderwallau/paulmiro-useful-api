use crate::endpoints::ApiResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct CongressBeerData {
    congressbeers: i64,
    message: String,
}

#[get("/congressbeer?<satoshi>&<format>")]
pub fn congressbeer(satoshi: f64, format: Option<String>) -> ApiResponse<CongressBeerData> {
    const CONGRESSBEER_SATOSHI: f64 = 69.0;
    let congressbeers = (satoshi / CONGRESSBEER_SATOSHI).floor() as i64;
    let message = format!(
        "{} Satoshi entspricht {} Congressbeers.",
        satoshi, congressbeers
    );

    match format.as_deref() {
        Some("json") => ApiResponse::Json(CongressBeerData {
            congressbeers,
            message,
        }),
        _ => ApiResponse::Plain(message),
    }
}
