// ============================================================
// commands/list.rs — List Assets Command
// Displays all assets with real-time currency conversion.
// ============================================================

// TODO: Define `run() -> OracleResult<()>`
//   1. Load all assets from storage
//   2. Load cached exchange rates from storage::load_rates()
//   3. For each asset:
//      a. Get the rate for its currency_tag using calculator::get_rate_for()
//      b. Compute the converted value using calculator::convert()
//      c. Print: "Asset: {name} | USD: {value_usd} | {currency}: {converted_value}"
//   4. Handle the case where no assets exist yet
