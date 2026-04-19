# Stellar Smart Contract

This directory contains the Soroban smart contract for the Legacy System Connector project. It provides the necessary structures and logic to facilitate on-chain interactions with the Stellar network.

## Project Structure

- `src/lib.rs`: Contains the Soroban smart contract logic.
- `Cargo.toml`: Rust dependency configuration.

## Prerequisites

To build and deploy the smart contract, you will need:

1. **Rust toolchain:** Install it from [rustup.rs](https://rustup.rs/).
2. **Target:** Add the WebAssembly target for Rust:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. **Stellar CLI (Soroban CLI):** Install the CLI to interact with Stellar testnet and deploy contracts:
   ```bash
   cargo install --locked stellar-cli@20.0.0-rc.4 --features opt
   ```

## Building

To compile the smart contract into a WebAssembly (.wasm) file, run the following command from this directory:

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled file will be located at `target/wasm32-unknown-unknown/release/stellar_contract.wasm`.

## Testing

To run the contract tests locally:

```bash
cargo test
```

## Deployment

Deploy the compiled WebAssembly to the Stellar testnet using the Stellar CLI:

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_contract.wasm \
  --source <your-secret-key> \
  --network testnet
```
