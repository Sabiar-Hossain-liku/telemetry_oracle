// ============================================================
// engine/storage.rs — Fault-Tolerant Persistence (Cache)
// Reads and writes the last known exchange rates to cache.db.
// ============================================================

// TODO: Define `save_rates(rates: &RateResponse) -> OracleResult<()>`
//   1. Serialize the RateResponse to JSON string (serde_json::to_string_pretty)
//   2. Write to "cache.db" using std::fs::write
//   3. Return Ok(())

// TODO: Define `load_rates() -> OracleResult<RateResponse>`
//   1. Read "cache.db" using std::fs::read_to_string
//   2. Deserialize JSON string back into RateResponse (serde_json::from_str)
//   3. Return the deserialized struct
//   4. Handle the case where cache.db doesn't exist yet (return a helpful error)
