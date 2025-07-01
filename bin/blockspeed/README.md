# Blockspeed

Real-time Ethereum blockchain performance monitoring tool that tracks transaction throughput, gas usage, and block processing metrics.

## Overview

Blockspeed connects to Ethereum RPC endpoints via WebSocket to monitor new blocks in real-time and calculates performance metrics including:

- **TPS (Transactions Per Second)** - Transaction processing rate
- **MGAS/s (Million Gas Per Second)** - Gas consumption rate
- **Block Processing Rate** - Blocks processed per second
- **Gas Usage** - Total gas used per block
- **Transaction Count** - Number of transactions per block

All metrics are stored in InfluxDB and visualized through Grafana dashboards.

## Architecture

```
Ethereum RPC (WebSocket) → Blockspeed → InfluxDB → Grafana
```

## Features

- **Real-time monitoring** via WebSocket subscriptions
- **Automatic reconnection** with exponential backoff
- **Graceful shutdown** handling (SIGINT/SIGTERM)
- **Race condition handling** for block data availability
- **Configurable retry logic** with timeouts
- **Multi-endpoint support** for monitoring multiple chains

## Quick Start

### Using Docker Compose

```bash
# Start the monitoring stack
docker-compose up -d

# Access Grafana dashboard
open http://localhost:3000
```

### Manual Installation

```bash
# Build the application
cargo build --release

./target/release/blockspeed --ws-rpc-url wss://ethereum-rpc.publicnode.com --influxdb-bucket blockspeed --influxdb-host http://localhost:8086/blockspeed --influxdb-org blockspeed-org --influxdb-token token
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ADDR` | Server bind address | `0.0.0.0` |
| `PORT` | Server port | `3031` |
| `METRICS_ADDR` | Metrics server bind address | `0.0.0.0` |
| `METRICS_PORT` | Metrics server port | `3001` |
| `WS_RPC_URL` | WebSocket URL for Ethereum RPC endpoint | *Required* |
| `INFLUXDB_HOST` | InfluxDB server host/URL | *Required* |
| `INFLUXDB_ORG` | InfluxDB organization | *Required* |
| `INFLUXDB_TOKEN` | InfluxDB authentication token | *Required* |
| `INFLUXDB_BUCKET` | InfluxDB bucket name | *Required* |

### Command Line Arguments

```bash
blockspeed [OPTIONS]

Options:
  --addr <ADDR>                      Server bind address [env: ADDR] [default: 0.0.0.0]
  --port <PORT>                      Server port [env: PORT] [default: 3031]
	--metrics-addr <METRICS_ADDR>      Metrics server bind address [env: METRICS_ADDR] [default: 0.0.0.0]
	--metrics-port <METRICS_PORT>      Metrics server port [env: METRICS_PORT] [default: 3001]
	--ws-rpc-url <WS_RPC_URL>          WebSocket URL for Ethereum RPC endpoint [env: WS_RPC_URL]
	--influxdb-host <INFLUXDB_HOST>    InfluxDB server host/URL [env: INFLUXDB_HOST]
	--influxdb-org <INFLUXDB_ORG>      InfluxDB organization [env: INFLUXDB_ORG]
	--influxdb-token <INFLUXDB_TOKEN>  InfluxDB authentication token [env: INFLUXDB_TOKEN]
	--influxdb-bucket <INFLUXDB_BUCKET> InfluxDB bucket name [env: INFLUXDB_BUCKET]
  -h, --help                             Print help information
  -V, --version                          Print version information
```