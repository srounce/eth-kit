#![allow(missing_docs)]

use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;

use tracing::{error, info};
use tracing_subscriber;

use blockspeed_cmd::cli::Cli;
use blockspeed_echo::server::serve_echo;
use blockspeed_trace::output::Output;
use blockspeed_trace::ws::WsConnection;
use eth_kit_metrics::{recorder::install_prometheus_recorder, server::MetricsServer};

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let addr = cli.resolve_addr().unwrap();
    let metrics_addr = cli.resolve_metrics_addr().unwrap();
    // let http_rpc_url = cli.resolve_http_rpc_url().unwrap();
    let ws_rpc_url = cli.resolve_ws_rpc_url().unwrap();
    let influxdb_host = cli.resolve_influxdb_host().unwrap();
    let influxdb_org = cli.resolve_influxdb_org().unwrap();
    let influxdb_token = cli.resolve_influxdb_token().unwrap();
    let influxdb_bucket = cli.resolve_influxdb_bucket().unwrap();

    let output = Arc::new(connect_influxdb(
        &influxdb_host,
        &influxdb_token,
        &influxdb_org,
        &influxdb_bucket,
    )?);

    let ws_connection = WsConnection::new(&ws_rpc_url.as_str())
        .await
        .expect("failed to connect to WebSocket");

    if let Err(e) = tokio::try_join!(
        serve_app(addr),
        serve_metrics(metrics_addr),
        subscribe_to_head(&ws_connection, Arc::clone(&output))
    ) {
        error!("error: {:?}", e);
    } else {
        info!("shutting down");
    }

    Ok(())
}

pub async fn serve_app(addr: SocketAddr) -> eyre::Result<()> {
    serve_echo(addr).await?;
    Ok(())
}

pub async fn serve_metrics(addr: SocketAddr) -> eyre::Result<()> {
    install_prometheus_recorder().spawn_upkeep();
    MetricsServer::new(addr).run().await?;
    Ok(())
}

pub fn connect_influxdb(host: &str, token: &str, org: &str, bucket: &str) -> eyre::Result<Output> {
    let client = Output::new(host, token, org, bucket);
    Ok(client)
}

pub async fn subscribe_to_head(
    ws_connection: &WsConnection,
    output: Arc<Output>,
) -> eyre::Result<()> {
    ws_connection
        .subscribe_to_head(output)
        .await
        .map_err(|e| eyre::eyre!("failed to subscribe to head: {}", e))?;
    Ok(())
}
