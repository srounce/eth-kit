#![allow(missing_docs)]

use futures_util::stream;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;

#[derive(Debug)]
pub struct Output {
    client: Client,
    bucket: String,
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
            client,
            bucket: bucket.to_string(),
        }
    }

    pub async fn write_mgas(
        &self,
        uri: &str,
        block_hash: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let points = vec![Mgas {
            uri: uri.to_string(),
            block_hash: block_hash.to_string(),
            value,
            timestamp,
        }];
        let stream = stream::iter(points);
        self.client.write(&self.bucket, stream).await?;

        Ok(())
    }

    pub async fn write_tps(
        &self,
        uri: &str,
        block_hash: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let points = vec![Tps {
            uri: uri.to_string(),
            block_hash: block_hash.to_string(),
            value,
            timestamp,
        }];
        let stream = stream::iter(points);
        self.client.write(&self.bucket, stream).await?;

        Ok(())
    }

    pub async fn write_gas_used(
        &self,
        uri: &str,
        block_hash: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let points = vec![GasUsed {
            uri: uri.to_string(),
            block_hash: block_hash.to_string(),
            value,
            timestamp,
        }];
        let stream = stream::iter(points);
        self.client.write(&self.bucket, stream).await?;

        Ok(())
    }

    pub async fn write_txs(
        &self,
        uri: &str,
        block_hash: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let points = vec![Txs {
            uri: uri.to_string(),
            block_hash: block_hash.to_string(),
            value,
            timestamp,
        }];
        let stream = stream::iter(points);
        self.client.write(&self.bucket, stream).await?;

        Ok(())
    }

    pub async fn write_block_per_sec(
        &self,
        uri: &str,
        value: f64,
        timestamp: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let points = vec![BlockPerSec {
            uri: uri.to_string(),
            value,
            timestamp,
        }];
        let stream = stream::iter(points);
        self.client.write(&self.bucket, stream).await?;

        Ok(())
    }
}
