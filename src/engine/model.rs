// ============================================================
// engine/model.rs — Asset & Rate Data Structures
// The schema-contract: Rust structs that mirror the JSON API.
// ============================================================

// TODO: Define `Asset` struct
//   Fields: id (String), name (String), value_usd (f64), currency_tag (String)
//   Derive: Debug, Clone, Serialize, Deserialize

// TODO: Define `RateResponse` struct
//   Mirrors the JSON returned by the exchange rate API.
//   Fields: base (String), date (String), rates (HashMap<String, f64>)
//   Derive: Debug, Deserialize

// TODO: Define `ExchangeRate` struct (optional helper)
//   Fields: from (String), to (String), rate (f64)
