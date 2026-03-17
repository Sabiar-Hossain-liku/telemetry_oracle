// A type alias so every function in the project returns
// OracleResult<T> instead of Result<T, SomeSpecificError>.
// anyhow::Result lets you use `?` on ANY error type automatically.
pub type OracleResult<T> = anyhow::Result<T>;
