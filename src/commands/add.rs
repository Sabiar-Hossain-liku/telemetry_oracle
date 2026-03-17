
use crate::engine::{model::Asset, storage};
use crate::errors::OracleResult;

/// Add a new asset to the portfolio and save it to disk.
pub fn run(name: String, value_usd: f64, currency: String) -> OracleResult<()> {
    // 1. Load existing assets (empty Vec on first run)
    let mut assets = storage::load_assets()?;

    // 2. Generate a sequential ID: "1", "2", "3"...
    let id = (assets.len() + 1).to_string();

    // 3. Build the new Asset struct
    let asset = Asset {
        id,
        name: name.clone(),
        value_usd,
        currency_tag: currency.clone(),
    };

    // 4. Add to the list and persist
    assets.push(asset);
    storage::save_assets(&assets)?;

    // 5. Confirm to the user
    println!(
        " Asset added → \"{}\" | ${:.2} USD | reported in {}",
        name, value_usd, currency
    );

    Ok(())
}
