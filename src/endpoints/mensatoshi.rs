use crate::endpoints::{ApiError, ApiResponse};
use rocket::State;
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct SatoshiPriceCache {
    pub price: f64,
    pub time: Instant,
}

pub type Cache = RwLock<Option<SatoshiPriceCache>>;

#[derive(Deserialize)]
struct CoinGeckoResponse {
    bitcoin: Bitcoin,
}

#[derive(Deserialize)]
struct Bitcoin {
    eur: f64,
}

#[derive(Serialize)]
pub struct MensaSatoshiData {
    satoshi: f64,
    message: String,
}

async fn fetch_price() -> Result<f64, ApiError> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=eur";
    let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .map_err(|_| ApiError {
            message: "Error creating HTTP client".to_string(),
        })?;

    let response = client.get(url).send().await.map_err(|_| ApiError {
        message: "Error fetching data from CoinGecko".to_string(),
    })?;

    let data = response
        .json::<CoinGeckoResponse>()
        .await
        .map_err(|_| ApiError {
            message: "Error deserializing CoinGecko response. Probably rate limited.".to_string(),
        })?;

    let eur_per_btc = data.bitcoin.eur;
    let satoshi_per_btc: f64 = 100_000_000.0;
    Ok(satoshi_per_btc / eur_per_btc)
}

async fn get_price(cache_state: &State<Cache>) -> Result<f64, ApiError> {
    // First, try to get a read lock.
    {
        let cache = cache_state.read().await;
        if let Some(price_cache) = *cache {
            if price_cache.time.elapsed() < Duration::from_secs(10) {
                return Ok(price_cache.price);
            }
        }
    }

    // If the cache is stale or empty, get a write lock to update it.
    let mut cache = cache_state.write().await;

    // We need to check again in case another request has already updated the cache
    // while we were waiting for the write lock.
    if let Some(price_cache) = *cache {
        if price_cache.time.elapsed() < Duration::from_secs(10) {
            return Ok(price_cache.price);
        }
    }

    let price = fetch_price().await?;

    *cache = Some(SatoshiPriceCache {
        price,
        time: Instant::now(),
    });

    Ok(price)
}

#[get("/mensatoshi?<format>")]
pub async fn mensatoshi(
    cache_state: &State<Cache>,
    format: Option<String>,
) -> ApiResponse<MensaSatoshiData> {
    let satoshi_per_eur = match get_price(cache_state).await {
        Ok(price) => price,
        Err(e) => return ApiResponse::Error(e),
    };

    let mensa_price_eur = 1.20; // TOOO: fetch dynamically
    let mensasatoshi = mensa_price_eur * satoshi_per_eur;
    let rounded_satoshi = mensasatoshi.round();
    let message = format!(
        "Der Mensa-Eintopf kostet aktuell {} Satoshi.",
        rounded_satoshi
    );

    match format.as_deref() {
        Some("json") => ApiResponse::Json(MensaSatoshiData {
            satoshi: rounded_satoshi,
            message,
        }),
        _ => ApiResponse::Plain(message),
    }
}
