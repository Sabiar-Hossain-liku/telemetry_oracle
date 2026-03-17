# Telemetry Oracle

Off-chain CLI tool that fetches live exchange rates and converts RWA prices into any currency.

## Usage

```bash
cargo run -- add "Porsche 911" 150000 EUR
cargo run -- refresh
cargo run -- list
```

## Setup

```bash
cp .env.example .env
# Add your EXCHANGE_RATE_API_KEY
cargo run -- refresh
```

## Structure

```
src/
├── main.rs           # Async entry point
├── lib.rs            # Module registry
├── errors.rs         # Error types
├── client/api.rs     # External API (reqwest)
├── engine/
│   ├── model.rs      # Asset & rate structs
│   ├── storage.rs    # Local cache
│   └── calculator.rs # Currency conversion
└── commands/
    ├── add.rs        # Add asset
    ├── list.rs       # List with live rates
    └── refresh.rs    # Pull new oracle data
```

Data flow: `CLI args → HTTP fetch → JSON parse → calculate → print`
