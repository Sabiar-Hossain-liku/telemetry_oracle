// ============================================================
// commands/refresh.rs — Refresh Oracle Data Command
// Manually pulls fresh exchange rate data and updates the cache.
// ============================================================

// TODO: Define `run() -> OracleResult<()>`
//   1. Call client::api::fetch_exchange_rates("USD").await
//   2. Save the returned RateResponse to cache using storage::save_rates()
//   3. Print a success message with the date of the fetched rates
//   4. Handle network errors gracefully (print error, don't panic)
