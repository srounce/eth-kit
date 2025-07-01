#![allow(missing_docs)]

use futures_util::stream;
use influxdb2::models::WriteDataPoint;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;
use std::{
    future::Future,
    sync::{Arc, Mutex},
};
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const INITIAL_RETRY_DELAY: Duration = Duration::from_millis(500);

#[derive(Debug)]
pub struct Output {
    client: Arc<Mutex<Client>>,
    bucket: String,
    url: String,
    token: String,
    org: String,
}

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "blockspeed_mgas"]
pub struct Mgas {
    #[influxdb(tag)]
    uri: String,
    #[influxdb(tag)]
    block_hash: String,
    #[influxdb(field)]
    value: f64,
    #[influxdb(timestamp)]
    timestamp: u64,
}

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "blockspeed_tps"]
pub struct Tps {
    #[influxdb(tag)]
    uri: String,
    #[influxdb(tag)]
    block_hash: String,
    #[influxdb(field)]
    value: f64,
    #[influxdb(timestamp)]
    timestamp: u64,
}

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "blockspeed_gas_used"]
pub struct GasUsed {
    #[influxdb(tag)]
    uri: String,
    #[influxdb(tag)]
    block_hash: String,
    #[influxdb(field)]
    value: f64,
    #[influxdb(timestamp)]
    timestamp: u64,
}

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "blockspeed_txs"]
pub struct Txs {
    #[influxdb(tag)]
    uri: String,
    #[influxdb(tag)]
    block_hash: String,
    #[influxdb(field)]
    value: f64,
    #[influxdb(timestamp)]
    timestamp: u64,
}

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "blockspeed_block_per_sec"]
pub struct BlockPerSec {
    #[influxdb(tag)]
    uri: String,
    #[influxdb(field)]
    value: f64,
    #[influxdb(timestamp)]
    timestamp: u64,
}

impl Output {
    pub fn new(url: &str, token: &str, org: &str, bucket: &str) -> Self {
        let client = Client::new(url, org, token);

        Self {
            client: Arc::new(Mutex::new(client)),
            bucket: bucket.to_string(),
            url: url.to_string(),
            token: token.to_string(),
            org: org.to_string(),
        }
    }

    async fn write_with_retry<T, F, Fut>(
        &self,
        operation_name: &str,
        create_points: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Vec<T>>,
        T: WriteDataPoint + Send + Sync + 'static,
    {
        let mut retry_count = 0;
        let mut retry_delay = INITIAL_RETRY_DELAY;

        loop {
            let points = create_points().await;
            let stream = stream::iter(points);

            // clone the client reference for this attempt
            let client = {
                let client_guard = self.client.lock().unwrap();
                client_guard.clone()
            };

            match client.write(&self.bucket, stream).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    let err_str = e.to_string();
                    let is_connection_error = err_str.contains("Connection refused")
                        || err_str.contains("Connection error")
                        || err_str.contains("Network unreachable")
                        || err_str.contains("timeout")
                        || err_str.contains("Connection reset")
                        || err_str.contains("Broken pipe");

                    if !is_connection_error || retry_count >= MAX_RECONNECT_ATTEMPTS {
                        error!(
                            "failed to write {} to InfluxDB after {} attempts: {}",
                            operation_name,
                            retry_count + 1,
                            e
                        );
                        return Err(e.into());
                    }

                    retry_count += 1;
                    warn!(
                        "failed to write {} to InfluxDB (attempt {}): {}, retrying in {:?}",
                        operation_name, retry_count, e, retry_delay
                    );

                    // Recreate client on connection errors
                    if retry_count == 1 {
                        info!("recreating InfluxDB client due to connection error");
                        let new_client = Client::new(&self.url, &self.org, &self.token);
                        let mut client_guard = self.client.lock().unwrap();
                        *client_guard = new_client;
                    }

                    sleep(retry_delay).await;

                    // Exponential backoff with jitter
                    retry_delay = std::cmp::min(retry_delay * 2, Duration::from_secs(30));
                }
            }
        }
    }

    pub async fn write_mgas(
        &self,
        uri: &str,
        block_hash: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write_with_retry("MGAS", || async {
            vec![Mgas {
                uri: uri.to_string(),
                block_hash: block_hash.to_string(),
                value,
                timestamp,
            }]
        })
        .await
    }

    pub async fn write_tps(
        &self,
        uri: &str,
        block_hash: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write_with_retry("TPS", || async {
            vec![Tps {
                uri: uri.to_string(),
                block_hash: block_hash.to_string(),
                value,
                timestamp,
            }]
        })
        .await
    }

    pub async fn write_gas_used(
        &self,
        uri: &str,
        block_hash: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write_with_retry("GAS_USED", || async {
            vec![GasUsed {
                uri: uri.to_string(),
                block_hash: block_hash.to_string(),
                value,
                timestamp,
            }]
        })
        .await
    }

    pub async fn write_txs(
        &self,
        uri: &str,
        block_hash: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write_with_retry("TXS", || async {
            vec![Txs {
                uri: uri.to_string(),
                block_hash: block_hash.to_string(),
                value,
                timestamp,
            }]
        })
        .await
    }

    pub async fn write_block_per_sec(
        &self,
        uri: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write_with_retry("BLOCK_PER_SEC", || async {
            vec![BlockPerSec {
                uri: uri.to_string(),
                value,
                timestamp,
            }]
        })
        .await
    }
}
