// ============================================================
// commands/add.rs — Add Asset Command
// Accepts user input and persists a new asset with a currency tag.
// ============================================================

// TODO: Define `run(name: String, value_usd: f64, currency: String) -> OracleResult<()>`
//   1. Create an Asset struct from the provided arguments
//   2. Load existing assets from storage (or start with an empty Vec)
//   3. Push the new asset into the Vec
//   4. Save the updated Vec back to storage
//   5. Print a confirmation message to the terminal
