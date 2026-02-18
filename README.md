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
src/
├── main.rs           # Async entry point + CLI dispatcher
├── lib.rs            # Module registry
├── errors.rs         # Custom error types
├── client/api.rs     # reqwest HTTP client
├── engine/model.rs   # Serde data structures
├── engine/storage.rs # Local cache (cache.db)
├── engine/calculator.rs # Currency conversion logic
└── commands/         # add / list / refresh handlers
```

## Why this matters for Solana RWA

This CLI is the **off-chain worker** that bridges the physical world and the digital ledger. It prepares real-world price data that can be fed into an on-chain Solana program as oracle data.
