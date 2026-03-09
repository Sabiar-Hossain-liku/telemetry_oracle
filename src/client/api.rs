// ============================================================
// client/api.rs — reqwest Implementation (The Radio Tower)
// Responsible for making async HTTP calls to the exchange rate API.
// ============================================================

use crate::engine::model::RateResponse;
use crate::errors::OracleResult;
use anyhow::Context;

/// Fetch live exchange rates from the ExchangeRate-API.
/// `base` is the source currency (e.g. "USD").
pub async fn fetch_exchange_rates(base: &str) -> OracleResult<RateResponse> {
    // 1. Read API credentials from the .env file
    let api_key = std::env::var("EXCHANGE_RATE_API_KEY")
        .context("EXCHANGE_RATE_API_KEY not set — add it to your .env file")?;

    let base_url = std::env::var("EXCHANGE_RATE_API_URL")
        .context("EXCHANGE_RATE_API_URL not set — add it to your .env file")?;

    // 2. Build the full URL:  https://v6.exchangerate-api.com/v6/<KEY>/latest/USD
    let url = format!("{}/{}/latest/{}", base_url, api_key, base.to_uppercase());

    // 3. Send the HTTP GET request
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to reach the Exchange Rate API — check your internet connection")?;

    // 4. Fail fast on HTTP 4xx / 5xx status codes
    let response = response
        .error_for_status()
        .context("Exchange Rate API returned an error — check your API key")?;

    // 5. Deserialise JSON → RateResponse struct (serde does the heavy lifting)
    let rates = response
        .json::<RateResponse>()
        .await
        .context("Failed to parse the API response as JSON")?;

    Ok(rates)
}
