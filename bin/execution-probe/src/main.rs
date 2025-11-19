#![allow(missing_docs)]

use clap::Parser;
use std::net::SocketAddr;

use tracing::{error, info};

use eth_kit_metrics::{recorder::install_prometheus_recorder, server::MetricsServer};
use execution_probe_cmd::cli::Cli;
use execution_probe_echo::server::serve_echo;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let addr = cli.resolve_addr().unwrap();
    let metrics_addr = cli.resolve_metrics_addr().unwrap();
    let node_uri = cli.resolve_node_uri().unwrap();
    let max_block_delay_seconds = cli.resolve_max_block_delay_seconds().unwrap();
    let min_peers = cli.resolve_min_peers().unwrap();

    if let Err(e) = tokio::try_join!(
        serve_app(
            addr,
            node_uri.to_string(),
            max_block_delay_seconds,
            min_peers
        ),
        serve_metrics(metrics_addr)
    ) {
        error!("error: {:?}", e);
    } else {
        info!("shutting down");
    }
}

pub async fn serve_app(
    addr: SocketAddr,
    node_uri: String,
    max_block_delay_seconds: u64,
    min_peers: u16,
) -> eyre::Result<()> {
    serve_echo(addr, node_uri, max_block_delay_seconds, min_peers).await?;
    Ok(())
}

pub async fn serve_metrics(addr: SocketAddr) -> eyre::Result<()> {
    install_prometheus_recorder().spawn_upkeep();
    MetricsServer::new(addr).run().await?;
    Ok(())
}
