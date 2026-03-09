// ============================================================
// main.rs — The Ignition
// Async entry point. Parses CLI args and dispatches commands.
// ============================================================

use clap::{Parser, Subcommand};
use telemetry_oracle::commands;
use telemetry_oracle::errors::OracleResult;

// ── CLI Struct ─────────────────────────────────────────────────
/// Telemetry Oracle — An off-chain oracle CLI for real-world asset valuation on Solana
#[derive(Parser)]
#[command(name = "telemetry_oracle", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// ── Subcommands Enum ───────────────────────────────────────────
#[derive(Subcommand)]
enum Commands {
    /// Add a new real-world asset to your portfolio
    Add {
        /// Name of the asset, e.g. "Gold 1oz"
        #[arg(short, long)]
        name: String,

        /// Base value in USD, e.g. 1850.00
        #[arg(short = 'v', long)]
        value_usd: f64,

        /// Target currency code to report in, e.g. EUR
        #[arg(short, long)]
        currency: String,
    },

    /// List all assets with their current converted values
    List,

    /// Fetch the latest exchange rates from the API
    Refresh,
}

// ── Entry Point ────────────────────────────────────────────────
#[tokio::main]
async fn main() -> OracleResult<()> {
    // Load .env variables (silently ignores missing .env file)
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            name,
            value_usd,
            currency,
        } => {
            commands::add::run(name, value_usd, currency)?;
        }
        Commands::List => {
            commands::list::run()?;
        }
        Commands::Refresh => {
            commands::refresh::run().await?;
        }
    }

    Ok(())
}
