// ============================================================
// errors.rs — The Check Engine Light
// Custom error types for network failures and parse errors.
// ============================================================

// TODO: Define an OracleError enum using `thiserror` or `anyhow`
// Variants to consider:
//   - NetworkError(String)   — reqwest / HTTP failures
//   - ParseError(String)     — serde_json deserialization failures
//   - StorageError(String)   — file I/O or cache failures
//   - NotFound(String)       — asset or rate not found

pub type OracleResult<T> = anyhow::Result<T>;
