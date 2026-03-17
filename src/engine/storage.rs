use crate::engine::model::{Asset, RateResponse};
use crate::errors::OracleResult;
use anyhow::Context;
use std::path::Path;

/// File where exchange rates are cached between API calls
const RATES_FILE: &str = "cache.db";
/// File where the asset portfolio is persisted
const ASSETS_FILE: &str = "assets.db";

// ── Exchange Rate Functions ────────────────────────────────────

/// Serialise a RateResponse to pretty JSON and write it to disk.
pub fn save_rates(rates: &RateResponse) -> OracleResult<()> {
    let json = serde_json::to_string_pretty(rates).context("Failed to serialise rate data")?;
    std::fs::write(RATES_FILE, json).context("Failed to write rate cache to disk")?;
    Ok(())
}

/// Load a previously cached RateResponse from disk.
/// Returns an error if the cache file does not exist yet.
pub fn load_rates() -> OracleResult<RateResponse> {
    if !Path::new(RATES_FILE).exists() {
        anyhow::bail!("No rate cache found. Run `refresh` first to fetch live rates.");
    }
    let raw = std::fs::read_to_string(RATES_FILE).context("Failed to read rate cache from disk")?;
    let rates = serde_json::from_str(&raw)
        .context("Rate cache file is corrupted. Run `refresh` to rebuild it.")?;
    Ok(rates)
}

// ── Asset Functions ────────────────────────────────────────────

/// Serialise the asset list to pretty JSON and write it to disk.
pub fn save_assets(assets: &[Asset]) -> OracleResult<()> {
    let json = serde_json::to_string_pretty(assets).context("Failed to serialise asset list")?;
    std::fs::write(ASSETS_FILE, json).context("Failed to write assets to disk")?;
    Ok(())
}

/// Load the asset list from disk.
/// Returns an empty Vec if no assets have been added yet (not an error).
pub fn load_assets() -> OracleResult<Vec<Asset>> {
    if !Path::new(ASSETS_FILE).exists() {
        return Ok(Vec::new()); // first-time user — no assets yet
    }
    let raw = std::fs::read_to_string(ASSETS_FILE).context("Failed to read assets from disk")?;
    let assets = serde_json::from_str(&raw).context("Asset file is corrupted.")?;
    Ok(assets)
}
