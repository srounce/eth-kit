# eth-kit

`eth-kit` is a comprehensive toolkit for Ethereum. This repository contains various tools and utilities designed to facilitate the development, monitoring, and maintenance of applications built on the Ethereum blockchain.

We will be adding through time more tools and utilities to this repository. If you have any suggestions or requests, please feel free to open an issue or a pull request.

## Build the project from source code

```
cargo build --release
```
## ETH Tools

All tools and more info can be found under the `bin` directory.

- [beacon-probe](./bin/beacon-probe/README.md) (monitor the health of Ethereum consensus clients)
- [execution-probe](./bin/execution-probe/README.md) (monitor the health of Ethereum execution clients)
- [blockspeed](./bin/blockspeed/README.md) (real-time Ethereum blockchain performance monitoring)

## Development Tools

| Tool           | Purpose                                 |
|----------------|-----------------------------------------|
| `rustfmt`      | Auto-format code                        |
| `clippy`       | Lint for common mistakes                |
| `cargo-audit`  | Scan for vulnerable dependencies        |
| `cargo-geiger` | Detect `unsafe` code in deps            |
| `proptest`     | Property-based testing                  |
| `criterion`    | Benchmarking with statistical analysis  |

```
cargo fmt -- --check
cargo clippy -- -D warnings
cargo audit
cargo geiger
```

## License

All other files within this repository are licensed under the MIT License unless stated otherwise.

## Support

This project is supported by [CuteTarantula](https://cutetarantula.com).

We are a UK-based consultancy specializing in Ethereum and blockchain solutions. Whether you have an exciting project idea or need expert guidance on any of our supported tools, we’re here to help. Don’t hesitate to reach out, we’d love to collaborate with you!
