# Execution Probe

The execution probe monitors the health and synchronization status of ethereum execution clients(geth, reth, ...) by continuesly checking key network and sync metrics.

## Features

- **Multi-client support** - Works with all major Ethereum execution clients
- **Configurable thresholds** - Set minimum peer counts and sync tolerances
- **Health scoring** - Comprehensive node health assessment

## Quick Start

### Manual Installation

```bash
# Build the application
cargo build --release

./target/release/execution-probe --node-uri http://localhost:8545 --min-peers 3
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ADDR` | Server bind address | `0.0.0.0` |
| `PORT` | Server port | `3031` |
| `METRICS_ADDR` | Metrics server bind address | `0.0.0.0` |
| `METRICS_PORT` | Metrics server port | `3001` |
| `NODE_URI` | Ethereum execution client URL | *Required* |
| `MIN_PEERS` | Minimum number of peers required for healthy status | `2` |

### Command Line Arguments

```bash
execution-probe [OPTIONS] --node-uri <NODE_URI>

Options:
  --addr <ADDR>                      Server bind address [env: ADDR] [default: 0.0.0.0]
  --port <PORT>                      Server port [env: PORT] [default: 3031]
	--metrics-addr <METRICS_ADDR>      Metrics server bind address [env: METRICS_ADDR] [default: 0.0.0.0]
	--metrics-port <METRICS_PORT>      Metrics server port [env: METRICS_PORT] [default: 3001]
	--node-uri <NODE_URI>              Ethereum execution client URL [env: NODE_URI]
	--min-peers <MIN_PEERS>            Minimum number of peers required [env: MIN_PEERS] [default: 2]
  -h, --help                             Print help information
  -V, --version                          Print version information
```
