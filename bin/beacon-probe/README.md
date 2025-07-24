# Beacon Probe

The beacon probe monitors the health and synchronization status of ethereum consensus clients(lighthouse, prysm,...) by continuesly checking key network and sync metrics.

## Features

- **Multi-client support** - Works with all major Ethereum consensus clients
- **Configurable thresholds** - Set minimum peer counts and sync tolerances
- **Health scoring** - Comprehensive node health assessment

## Quick Start

### Manual Installation

```bash
# Build the application
cargo build --release

./target/release/beacon-probe --node-uri http://localhost:3500
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ADDR` | Server bind address | `0.0.0.0` |
| `PORT` | Server port | `3031` |
| `METRICS_ADDR` | Metrics server bind address | `0.0.0.0` |
| `METRICS_PORT` | Metrics server port | `3001` |
| `NODE_URI` | Ethereum consensus client URL | *Required* |

### Command Line Arguments

```bash
beacon-probe [OPTIONS] --node-uri <NODE_URI>

Options:
  --addr <ADDR>                      Server bind address [env: ADDR] [default: 0.0.0.0]
  --port <PORT>                      Server port [env: PORT] [default: 3031]
	--metrics-addr <METRICS_ADDR>      Metrics server bind address [env: METRICS_ADDR] [default: 0.0.0.0]
	--metrics-port <METRICS_PORT>      Metrics server port [env: METRICS_PORT] [default: 3001]
	--node-uri <NODE_URI>              Ethereum consensus client URL [env: NODE_URI]
  -h, --help                             Print help information
  -V, --version                          Print version information
```
