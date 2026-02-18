// ============================================================
// engine/calculator.rs — Cross-Currency Valuation Logic
// Multiplies an asset's USD value by the live exchange rate.
// ============================================================

// TODO: Define `convert(value_usd: f64, rate: f64) -> f64`
//   Formula: value_usd * rate
//   This is the core computation: USD cost × EUR rate = EUR value

// TODO: Define `get_rate_for(rates: &RateResponse, target_currency: &str) -> OracleResult<f64>`
//   1. Look up target_currency in rates.rates HashMap
//   2. Return the f64 rate if found
//   3. Return an error if the currency code is not in the map
