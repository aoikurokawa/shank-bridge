# NCN Portal

## Getting Started

### Set Environment Variable

- `ANTHROPIC_API_KEY`

### Build

```bash
cargo b
```

### Run

```bash
cargo r --bin server
```

## Endpoints

```bash
curl http://localhost:8080/prompt
```

## Whitelist Program

### Test

```bash
cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run --all-features
```

### Generate Clients

```bash
cargo b -p shank-cli && ./target/debug/shank-cli && yarn generate-clients && cargo b
```

## Resources
- https://rig.rs/index.html
- https://docs.anthropic.com/en/home

