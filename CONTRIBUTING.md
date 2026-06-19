# Contributing to tronz/examples

Thanks for your interest in contributing! All contributions are welcome.

## Adding an Example

1. Pick the appropriate category directory under `examples/` (queries, signers, transfers, staking, contracts, trc10, trc20, accounts).
2. Add a new `.rs` file under `examples/<category>/examples/`.
3. Update the checkbox index in `README.md`.

### Conventions

- All examples target the **Nile testnet** by default.
- Read-only examples must run without any environment variables.
- Examples that send transactions require `TRON_PRIVATE_KEY=<hex>`.
- Use `TRON_API_KEY` for optional TronGrid API key support.
- Use `anyhow::Result<()>` as the return type of `main`.
- Keep examples focused — one concept per file.

### Running Examples

```sh
# Read-only (no key needed)
cargo run -p examples-queries --example query

# With private key
TRON_PRIVATE_KEY=<hex> cargo run -p examples-staking --example stake
```

### Before Submitting

```sh
cargo +nightly fmt --all
cargo +nightly clippy --examples --all-features -- -D warnings
cargo build --examples --all-features
```

## License

By contributing, you agree that your contributions will be licensed under the same
[MIT OR Apache-2.0](LICENSE-MIT) terms as the rest of the project.
