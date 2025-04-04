use crate::models::{ApiResponse, Coin};
use log::{debug, warn};
use reqwest::Client;
use std::time::Duration;
use serde_json::json;
use std::env;
use log::error;

const API_BASE_URL: &str = "https://api-sdk.zora.engineering/explore";



pub async fn create_zora_token(name: &str, symbol: &str, metadata: &str) -> Result<String, String> {
    let api_key = env::var("ZORA_API_KEY").map_err(|_| "ZORA_API_KEY not set".to_string())?;
    let client = Client::new();
    let url = "https://api.zora.co/coins/sdk/create-coin";

    let payload = json!({
        "name": name,
        "symbol": symbol,
        "metadata": metadata,
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|e| format!("Failed to read response: {}", e))?;
        Ok(body)
    } else {
        let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("API error: {}", error_message))
    }
}

pub async fn fetch_zora_coins(list_type: &str) -> Result<Vec<Coin>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}?listType={}&count=10", API_BASE_URL, list_type);
    debug!("Fetching from URL: {}", url);

    let res = client
        .get(&url)
        .header("accept", "application/json")
        .send()
        .await?;

    debug!("Response status: {}", res.status());
    let api_res = res.json::<ApiResponse>().await?;
    let coins = api_res
        .explore_list
        .edges
        .into_iter()
        .map(|e| e.node)
        .collect::<Vec<_>>();

    debug!("Fetched {} coins for {}", coins.len(), list_type);
    Ok(coins)
}

// Simple rate limiting with exponential backoff
pub async fn fetch_with_backoff(list_type: &str, max_retries: u32) -> Result<Vec<Coin>, reqwest::Error> {
    let mut retries = 0;
    loop {
        match fetch_zora_coins(list_type).await {
            Ok(coins) => return Ok(coins),
            Err(e) if retries < max_retries => {
                let delay = Duration::from_secs(2u64.pow(retries));
                warn!("API error: {}. Retrying in {:?} (attempt {}/{})", e, delay, retries + 1, max_retries);
                tokio::time::sleep(delay).await;
                retries += 1;
            }
            Err(e) => return Err(e),
        }
    }
}