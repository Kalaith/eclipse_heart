# Eclipse Heart Tech Stack

## Client

- Rust 2021
- Macroquad
- `macroquad-toolkit`

Rationale:

- matches repository standards
- supports native and WASM from one codebase
- keeps rendering/input thin while rules live in plain Rust modules

## Data

- `serde`
- `serde_json`

Use JSON under `assets/data/` for:

- Magical Girl definitions
- Baddie definitions
- story card definitions
- rules and thresholds
- starter loadouts
- UI text

## Persistence

Local JSON save files first:

- collection
- decks
- roster presets
- settings
- profile progression

## Future online stack

Not required for the first playable, but the architecture should leave room for:

- separate Rust service crate
- HTTP API
- authoritative ownership and trade validation
- online PvP authority

## Recommended module layout

```text
src/
├── main.rs
├── game.rs
├── data/
├── engine/
├── state/
├── screens/
└── ui/
```

## Build commands

```bash
cargo fmt
cargo clippy -p eclipse_heart --all-targets --all-features
cargo test -p eclipse_heart
cargo build --release -p eclipse_heart
cargo build --release --target wasm32-unknown-unknown -p eclipse_heart
```

## Design rules

- UI returns intents, engine applies rules
- gameplay values come from JSON, not hardcoded constants
- use explicit state machines for screens and match phases
- keep battle resolution deterministic for testing and future networking

## Revision

- 2026-03-21: replaced generic template with Eclipse Heart Rust stack
