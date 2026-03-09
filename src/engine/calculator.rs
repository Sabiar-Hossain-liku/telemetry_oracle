use crate::engine::model::RateResponse;
use crate::errors::OracleResult;
use anyhow::anyhow;

/// Multiply a USD value by an exchange rate to get the target currency value.
/// Example: convert(1000.0, 0.92) → 920.0  (USD → EUR)
pub fn convert(value_usd: f64, rate: f64) -> f64 {
    value_usd * rate
}

/// Look up the exchange rate for `target_currency` from a RateResponse.
/// Returns an error if the currency code isn't in the map.
pub fn get_rate_for(rates: &RateResponse, target_currency: &str) -> OracleResult<f64> {
    rates
        .conversion_rates
        .get(target_currency) // HashMap.get() → Option<&f64>
        .copied() // Option<&f64> → Option<f64>
        .ok_or_else(|| {
            anyhow!(
                "Currency '{}' not found in rate data. Check the currency code.",
                target_currency
            )
        })
}
