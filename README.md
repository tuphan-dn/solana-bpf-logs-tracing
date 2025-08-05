# Solana BPF Logs Tracing

A procedural macro to trace logs in Solana programs.

## Features

- Trace errors with filename and line number.
- On-chain tracing is disabled in production, so your programs are unaffected when released.

## Getting Started

### Installation

Add the crate to your project:

```bash
cargo add solana-bpf-logs-tracing
```

### Usage

Enable the `tracing` feature when building your program:

```bash
anchor build -- --features tracing
```

Apply the `[trace]` macro to any function you want to debug:

```rs
#[trace]
pub fn your_function() {
  // ...
}
```

**NOTE:** For release builds, omit the `--features tracing` flag to disable all `[trace]` macros in your program.

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss your ideas.

## License

MIT
