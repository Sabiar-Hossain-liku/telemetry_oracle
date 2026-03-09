// ============================================================
// commands/refresh.rs — Refresh Oracle Data Command
// Pulls fresh exchange rate data from the API and caches it.
// ============================================================

use crate::client::api;
use crate::engine::storage;
use crate::errors::OracleResult;

/// Fetch live USD exchange rates from the API and save them to disk.
pub async fn run() -> OracleResult<()> {
    println!("🔄  Fetching live exchange rates from ExchangeRate-API...");

    match api::fetch_exchange_rates("USD").await {
        Ok(rates) => {
            // Capture display info before `rates` is moved into save_rates()
            let base = rates.base_code.clone();
            let updated = rates.time_last_update_utc.clone();
            let count = rates.conversion_rates.len();

            // Persist the fresh data to disk
            storage::save_rates(&rates)?;

            println!("✅  Rates updated successfully!");
            println!("    Base currency : {}", base);
            println!("    Last updated  : {}", updated);
            println!("    Currencies    : {} supported", count);
        }
        Err(e) => {
            // Print a friendly error — don't propagate; let the user retry
            eprintln!("❌  Failed to fetch rates: {:#}", e);
            eprintln!("💡  Check: Is EXCHANGE_RATE_API_KEY set correctly in .env?");
        }
    }

    Ok(())
}
