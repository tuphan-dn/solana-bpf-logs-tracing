# Solana BPF Logs Tracing

A procedural macro for tracing logs in Solana programs.

## Features

- Compatible with Anchor framework.
- Trace errors with filename and line number.
- On-chain tracing is disabled in production, so release builds are unaffected.

## Getting Started

### Installation

```bash
cargo add solana-bpf-logs-tracing
```

### Usage

Add the `tracing` feature to your program's features (i.e. `programs/<your_program>/Cargo.toml`):

```toml
[features]
# ...
tracing = ["solana-bpf-logs-tracing/tracing"]
```

Enable tracing when building for development:

```bash
anchor build -- --features tracing
```

Apply the `#[trace]` macro to any function you want to debug:

```rs
#[trace]
pub fn your_function() {
  // ...
}
```

**Note:** For release builds, omit the `--features tracing` flag to disable all `#[trace]` macros.

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss your ideas.

## License

MIT
