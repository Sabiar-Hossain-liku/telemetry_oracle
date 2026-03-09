# Telemetry Oracle

An off-chain CLI oracle for real-world asset (RWA) valuation on Solana.

## What it does

Fetches live exchange rates from a Central Bank API and converts the USD value of your real-world assets (e.g., a Porsche collection) into any target currency — in real time.

## The Value Journey

```
User Input → reqwest HTTP Call → JSON Parsed by Serde → Calculator Module → Terminal Report
```

## Usage

```bash
# Add an asset
cargo run -- add "Porsche 911" 150000 EUR

# Refresh exchange rates from the API
cargo run -- refresh

# List all assets with live conversion
cargo run -- list
```

## Setup

```bash
cp .env.example .env
# Fill in your EXCHANGE_RATE_API_KEY in .env
cargo run -- refresh
cargo run -- list
```

## Project Structure

```
├── Cargo.toml            # Features: tokio (full), reqwest (json), serde, dotenv
├── .env                  # API Keys & Secrets (The Fuel Grade)
├── src/
│   ├── main.rs           # The Ignition: Async entry point
│   ├── lib.rs            # The Chassis: Module registry
│   ├── errors.rs         # Check Engine Light: Network & Parse errors
│   ├── client/           # The Radio Tower: External API Logic
│   │   ├── mod.rs
│   │   └── api.rs        # reqwest implementation
│   ├── engine/           # THE POWERTRAIN: Core Business Logic
│   │   ├── mod.rs
│   │   ├── model.rs      # Asset & Rate Data Structures
│   │   ├── storage.rs    # Persistence (Local Cache)
│   │   └── calculator.rs # Cross-currency valuation logic
│   └── commands/         # THE DASHBOARD: User Interface
│       ├── mod.rs        # Command Dispatcher
│       ├── add.rs        # Add asset with currency tag
│       ├── list.rs       # List assets with real-time conversion
│       └── refresh.rs    # Manually pull new Oracle data
└── cache.db              # Local storage for the last known rates
```

## Why this matters for Solana RWA

This CLI is the **off-chain worker** that bridges the physical world and the digital ledger. It prepares real-world price data that can be fed into an on-chain Solana program as oracle data.
