# Distributed Ledger Rust

High-performance, immutable blockchain ledger implemented in Rust.

## Features

- **Proof-of-Work**: Mining consensus mechanism
- **SHA-256 Hashing**: Cryptographic integrity
- **Block Validation**: Transaction verification
- **Chain Linking**: Immutable block connections
- **Mining Difficulty**: Configurable proof-of-work

## Usage

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

## Project Structure

```
distributed-ledger-rust/
├── src/
│   ├── blockchain/     # Block and chain implementation
│   └── utils/         # Utility functions
├── Cargo.toml         # Dependencies
├── README.md          # This file
└── .github/
    └── workflows/
        └── ci.yml    # CI/CD pipeline
```

## Architecture

- **Block**: Contains transactions, hash, previous hash
- **Chain**: Manages block validation and linking
- **Mining**: Proof-of-work consensus

## Requirements

- Rust 1.70+
- Cargo

## License

MIT
