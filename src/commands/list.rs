
use crate::engine::{calculator, storage};
use crate::errors::OracleResult;

/// Display all assets, converting their USD value into each asset's target currency.
pub fn run() -> OracleResult<()> {
    // 1. Load the portfolio
    let assets = storage::load_assets()?;

    // 2. Handle the empty case gracefully
    if assets.is_empty() {
        println!("📭  No assets found. Use `add` to create one.");
        return Ok(());
    }

    // 3. Load the cached exchange rates (user must run `refresh` first)
    let rates = storage::load_rates()?;

    // 4. Print table header
    println!(
        "\n{:<4} {:<22} {:>12} {:>6} {:>14}",
        "ID", "Name", "USD Value", "CCY", "Converted"
    );
    println!("{}", "─".repeat(62));

    // 5. Print each asset row, converting the USD value
    for asset in &assets {
        match calculator::get_rate_for(&rates, &asset.currency_tag) {
            Ok(rate) => {
                let converted = calculator::convert(asset.value_usd, rate);
                println!(
                    "{:<4} {:<22} {:>12.2} {:>6} {:>14.2}",
                    asset.id, asset.name, asset.value_usd, asset.currency_tag, converted
                );
            }
            Err(e) => {
                // Don't crash the whole list — show a warning for this row only
                println!(
                    "{:<4} {:<22} {:>12.2} {:>6} {:>14}",
                    asset.id,
                    asset.name,
                    asset.value_usd,
                    asset.currency_tag,
                    format!("⚠ {}", e)
                );
            }
        }
    }

    // 6. Footer with the timestamp of the last rate update
    println!("{}", "─".repeat(62));
    println!("📅  Rates last updated: {}", rates.time_last_update_utc);

    Ok(())
}
