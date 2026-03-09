// ============================================================
// engine/model.rs — Asset & Rate Data Structures
// The schema-contract: Rust structs that mirror the JSON API.
// ============================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A real-world asset in the portfolio (e.g. Gold 1oz priced in EUR)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Sequential ID, e.g. "1", "2"
    pub id: String,
    /// Human label, e.g. "Gold 1oz"
    pub name: String,
    /// Base value in US dollars
    pub value_usd: f64,
    /// Target currency code, e.g. "EUR"
    pub currency_tag: String,
}

/// Mirrors the JSON shape returned by the ExchangeRate-API:
/// {
///   "base_code": "USD",
///   "time_last_update_utc": "Fri, 28 Feb 2026 00:00:01 +0000",
///   "conversion_rates": { "EUR": 0.92, "GBP": 0.79, ... }
/// }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateResponse {
    /// Base currency, always "USD" in our case
    pub base_code: String,
    /// Human-readable timestamp of the last rate update
    pub time_last_update_utc: String,
    /// Map of currency code → exchange rate
    pub conversion_rates: HashMap<String, f64>,
}
