#![allow(missing_docs)]

use crate::output::Output;
use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use serde_json::Value;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::signal::unix::{signal, SignalKind};
use tracing::{error, info};

const MAX_RECONNECT_ATTEMPTS: u32 = 10;

#[derive(Debug)]
pub struct WsConnection {
    uri: String,
}

impl WsConnection {
    pub async fn new(uri: &str) -> eyre::Result<Self> {
        Ok(Self {
            uri: uri.to_string(),
        })
    }

    pub async fn subscribe_to_head(&self, output: Arc<Output>) -> eyre::Result<()> {
        let mut reconnect_attempts = 0;
        let mut backoff_delay = tokio::time::Duration::from_secs(1);

        let mut sigint = signal(SignalKind::interrupt()).unwrap();
        let mut sigterm = signal(SignalKind::terminate()).unwrap();

        let mut timestamp_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos() as u64;

        loop {
            tokio::select! {
                client = WsClientBuilder::default().build(&self.uri) => {
                    match client {
                        Ok(client) => {
                            info!("connected to WebSocket at {}", self.uri);
                            reconnect_attempts = 0; // reset on successful connection
                            backoff_delay = tokio::time::Duration::from_secs(1); // reset on successful connection

                            match client.subscribe(
                                "eth_subscribe",
                                jsonrpsee::core::rpc_params!["newHeads"],
                                "eth_unsubscribe"
                            ).await {
                                Ok(mut subscription) => {
                                    info!("subscribed to newHeads");

                                    loop {
                                        match subscription.next().await {
                                            Some(Ok(header_value)) => {
                                                if let Err(e) = self.process_block_header(
                                                    &client,
                                                    header_value,
                                                    &mut timestamp_nanos,
                                                    &output
                                                ).await {
                                                    error!("failed to process block header: {}", e);
                                                    break; // break to trigger reconnection
                                                }
                                            },
                                            Some(Err(e)) => {
                                                error!("subscription error: {:?}", e);
                                                break; // break to trigger reconnection
                                            }
                                            None => {
                                                info!("subscription closed");
                                                break; // break to trigger reconnection
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("failed to subscribe to newHeads: {}", e);
                                    // falls through to trigger reconnection
                                }
                            }
                        }
                        Err(e) => {
                            error!("failed to connect to WebSocket: {}", e);
                            if reconnect_attempts >= MAX_RECONNECT_ATTEMPTS {
                                return Err(eyre::eyre!("maximum reconnect attempts reached"));
                            }
                            tokio::time::sleep(backoff_delay).await;
                            backoff_delay = std::cmp::min(backoff_delay * 2, tokio::time::Duration::from_secs(30));
                            reconnect_attempts += 1;
                            info!("reconnecting in {} seconds...", backoff_delay.as_secs());
                        }
                    }
                }

                _ = sigint.recv() => {
                    info!("received sigint, shutting down");
                    break;
                }

                _ = sigterm.recv() => {
                    info!("received sigterm, shutting down");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn process_block_header(
        &self,
        client: &WsClient,
        header_value: Value,
        timestamp_nanos: &mut u64,
        output: &Output,
    ) -> eyre::Result<()> {
        let prev_timestamp_nanos = *timestamp_nanos;
        *timestamp_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos() as u64;

        let diff_timestamp_nanos = *timestamp_nanos - prev_timestamp_nanos;

        let header: Value = header_value;
        let block_hash = header["hash"].as_str().unwrap_or_default();

        let block_tx_count = loop {
            match tokio::time::timeout(
                tokio::time::Duration::from_secs(1),
                client.request::<Option<String>, _>(
                    "eth_getBlockTransactionCountByHash",
                    jsonrpsee::core::rpc_params![block_hash],
                ),
            )
            .await
            {
                Ok(Ok(Some(hex_str))) => {
                    break u64::from_str_radix(hex_str.trim_start_matches("0x"), 16).map_err(
                        |e| eyre::eyre!("failed to parse transaction count '{}': {}", hex_str, e),
                    )? as usize;
                }
                Ok(Ok(None)) => {
                    // block not available yet, retry after delay
                    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                    continue;
                }
                Ok(Err(e)) => {
                    // RPC error, retry after delay
                    error!(
                        "RPC error getting block transaction count: {}, retrying in 150ms",
                        e
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                    continue;
                }
                Err(_) => {
                    // timeout, retry after delay
                    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                    continue;
                }
            }
        };

        let block_gas_used_hex = header["gasUsed"].as_str().unwrap_or_default();
        let block_gas_used = u64::from_str_radix(block_gas_used_hex.trim_start_matches("0x"), 16)
            .map_err(|e| eyre::eyre!("failed to parse gas used: {}", e))?;

        let mgas =
            block_gas_used as f64 / (diff_timestamp_nanos as f64 / 1_000_000_000.0) / 1_000_000.0;

        let tps = if diff_timestamp_nanos >= 1_000_000_000 {
            block_tx_count as f64 / (diff_timestamp_nanos as f64 / 1_000_000_000.0)
        } else {
            block_tx_count as f64
        };

        if let Err(e) = output
            .write_mgas(&self.uri, block_hash, mgas, *timestamp_nanos)
            .await
        {
            error!("failed to write mgas: {}", e);
        }

        if let Err(e) = output
            .write_tps(&self.uri, block_hash, tps, *timestamp_nanos)
            .await
        {
            error!("failed to write tps: {}", e);
        }

        if let Err(e) = output
            .write_gas_used(
                &self.uri,
                block_hash,
                block_gas_used as f64,
                *timestamp_nanos,
            )
            .await
        {
            error!("failed to write gas used: {}", e);
        }

        if let Err(e) = output
            .write_txs(
                &self.uri,
                block_hash,
                block_tx_count as f64,
                *timestamp_nanos,
            )
            .await
        {
            error!("failed to write txs: {}", e);
        }

        if let Err(e) = output
            .write_block_per_sec(&self.uri, 1.0, *timestamp_nanos)
            .await
        {
            error!("failed to write block per sec: {}", e);
        }

        Ok(())
    }
}
