#![allow(missing_docs)]

use clap::Parser;
use std::net::{SocketAddr, ToSocketAddrs};
use url::Url;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "ADDR",
        env = "ADDR",
        default_value = "0.0.0.0"
    )]
    addr: String,

    #[arg(short, long, value_name = "PORT", env = "PORT", default_value = "3031")]
    port: u16,

    #[arg(
        long,
        value_name = "METRICS_ADDR",
        env = "METRICS_ADDR",
        default_value = "0.0.0.0"
    )]
    metrics_addr: String,

    #[arg(
        long,
        value_name = "METRICS_PORT",
        env = "METRICS_PORT",
        default_value = "3001"
    )]
    metrics_port: u16,

    #[arg(long, value_name = "WS_RPC_URL", env = "WS_RPC_URL")]
    ws_rpc_url: Option<String>,

    #[arg(long, value_name = "INFLUXDB_HOST", env = "INFLUXDB_HOST")]
    influxdb_host: String,

    #[arg(long, value_name = "INFLUXDB_ORG", env = "INFLUXDB_ORG")]
    influxdb_org: String,

    #[arg(long, value_name = "INFLUXDB_TOKEN", env = "INFLUXDB_TOKEN")]
    influxdb_token: String,

    #[arg(long, value_name = "INFLUXDB_BUCKET", env = "INFLUXDB_BUCKET")]
    influxdb_bucket: String,
}

impl Cli {
    pub fn resolve_addr(&self) -> eyre::Result<SocketAddr> {
        let addr_str = format!("{}:{}", self.addr, self.port);
        addr_str
            .to_socket_addrs()
            .map_err(|e| eyre::eyre!("failed to parse address {}: {}", addr_str, e))?
            .next()
            .ok_or_else(|| eyre::eyre!("unable to resolve address: {}", addr_str))
    }

    pub fn resolve_metrics_addr(&self) -> eyre::Result<SocketAddr> {
        let addr_str = format!("{}:{}", self.metrics_addr, self.metrics_port);
        addr_str
            .to_socket_addrs()
            .map_err(|e| eyre::eyre!("failed to parse address {}: {}", addr_str, e))?
            .next()
            .ok_or_else(|| eyre::eyre!("unable to resolve address: {}", addr_str))
    }

    pub fn resolve_ws_rpc_url(&self) -> eyre::Result<Url> {
        match self.ws_rpc_url.as_deref() {
            Some(url_str) => match Url::parse(url_str) {
                Ok(url) => {
                    if url.scheme() == "ws" || url.scheme() == "wss" {
                        Ok(url)
                    } else {
                        Err(eyre::eyre!("invalid WS RPC URL scheme: {}", url))
                    }
                }
                Err(e) => Err(eyre::eyre!("failed to parse WS RPC URL: {}", e)),
            },
            None => Err(eyre::eyre!("WS RPC URL not provided")),
        }
    }

    pub fn resolve_influxdb_host(&self) -> eyre::Result<String> {
        if self.influxdb_host.is_empty() {
            Err(eyre::eyre!("INFLUXDB_HOST not provided"))
        } else {
            Ok(self.influxdb_host.clone())
        }
    }
    pub fn resolve_influxdb_org(&self) -> eyre::Result<String> {
        if self.influxdb_org.is_empty() {
            Err(eyre::eyre!("INFLUXDB_ORG not provided"))
        } else {
            Ok(self.influxdb_org.clone())
        }
    }

    pub fn resolve_influxdb_token(&self) -> eyre::Result<String> {
        if self.influxdb_token.is_empty() {
            Err(eyre::eyre!("INFLUXDB_TOKEN not provided"))
        } else {
            Ok(self.influxdb_token.clone())
        }
    }

    pub fn resolve_influxdb_bucket(&self) -> eyre::Result<String> {
        if self.influxdb_bucket.is_empty() {
            Err(eyre::eyre!("INFLUXDB_BUCKET not provided"))
        } else {
            Ok(self.influxdb_bucket.clone())
        }
    }
}
