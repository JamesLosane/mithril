# Mithril Aggregator

**This is a work in progress** :hammer_and_wrench:

This is a first version of the Mithril Aggregagator

---

## Pre-requisites

**Install Rust**

- Install a [correctly configured](https://www.rust-lang.org/learn/get-started) Rust toolchain (version 1.62.0+).
- Install OpenSSL development libraries, for example on Ubuntu/Debian/Mint run `apt install libssl-dev`

## Download source code

```bash
# Download sources from github
git clone https://github.com/input-output-hk/mithril

# Go to sources directory
cd mithril-aggregator
```

## Development test and build

```bash
# Test
make test

# Help
make help

# Doc
make doc

# Run in debug with default configuration
make debug
```

## Release build and run binary

```bash
# Build and run in release with default configuration
make run

# Or
# Build
make build

# Help
./mithril-aggregator --help

# Run
./mithril-aggregator

# Run in a specific mode
./mithril-aggregator -r testnet

# Run with custom configuration with env vars
RUN_INTERVAL=60000 NETWORK=testnet URL_SNAPSHOT_MANIFEST=https://storage.googleapis.com/cardano-testnet/snapshots.json ./mithril-aggregator
```

## Build and run Docker container

```bash
# Build Docker image
make docker-build

# Run Docker container
make docker-run
```

## Interact with the Mithril Aggregator

```bash
# Interact with the aggregator through the OpenAPI UI
open -u https://input-output-hk.github.io/mithril/openapi-ui/
```
