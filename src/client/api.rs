// ============================================================
// client/api.rs — reqwest Implementation
// Responsible for making async HTTP calls to the exchange rate API.
// ============================================================

// TODO: Create an async function `fetch_exchange_rates(base: &str) -> OracleResult<RateResponse>`
//   1. Read API key from environment (std::env::var)
//   2. Build the request URL using the base currency
//   3. Use reqwest::Client to send a GET request (await)
//   4. Check for HTTP errors (.error_for_status()?)
//   5. Deserialize the JSON body into RateResponse (.json::<RateResponse>().await?)
//   6. Return the deserialized struct
