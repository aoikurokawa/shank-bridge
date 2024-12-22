# Whitelist Program

## Whitelist Program

### Test

```bash
cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run --all-features
```

### Generate Clients

```bash
cargo b -p shank-cli && ./target/debug/shank-cli && yarn generate-clients && cargo b
```

## CLI

### HELP!

```bash
cargo r -p cli ncn-portal whitelist --help
```

### Initialize Whitelist

```bash
cargo r -p cli ncn-portal whitelist initialize --keypair "KEYPAIR" --rpc-url "https://api.devnet.solana.com" --ncn-portal-program-id "DwyMNTQ5aSduQhx3Pjra9kXeySxjD5YUkC1bDXmvEPFZ"
```

### Add to Whitelist

```bash
cargo r -p cli ncn-portal whitelist add-to-whitelist "DyEKpfGg6sBL2Dg6rnHcsdAHJdCoe7Ur5VWzDzdHkQY6" 100 --keypair "KEYPAIR" --rpc-url "https://api.devnet.solana.com" --ncn-portal-program-id "DwyMNTQ5aSduQhx3Pjra9kXeySxjD5YUkC1bDXmvEPFZ"
```

## Resources
- https://rig.rs/index.html
- https://docs.anthropic.com/en/home

