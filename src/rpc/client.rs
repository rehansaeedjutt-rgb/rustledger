//! Minimal async JSON-RPC client for Ethereum.
//!
//! Uses reqwest with rustls (not OpenSSL — GNU toolchain friendly).
//! Public endpoint only; no API key. Not production-grade — no rate-limit
//! awareness, no retry jitter.

use crate::models::Block;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("RPC returned error: {0}")]
    Rpc(String),
    #[error("block not found")]
    BlockNotFound,
    #[error("unexpected response shape")]
    BadResponse,
}

/// Async Ethereum JSON-RPC client.
pub struct RpcClient {
    url: String,
    http: reqwest::Client,
    next_id: AtomicU64,
}

impl RpcClient {
    /// Build a client against a single endpoint.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(20))
                .build()
                .expect("failed to build HTTP client"),
            next_id: AtomicU64::new(1),
        }
    }

    /// Default client against a free public endpoint.
    pub fn public() -> Self {
        Self::new("https://ethereum.publicnode.com")
    }

    async fn call(&self, method: &str, params: Value) -> Result<Value, RpcError> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let body = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let resp: Value = self
            .http
            .post(&self.url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        if let Some(err) = resp.get("error") {
            return Err(RpcError::Rpc(err.to_string()));
        }
        resp.get("result").cloned().ok_or(RpcError::BadResponse)
    }

    /// Latest block number.
    pub async fn block_number(&self) -> Result<u64, RpcError> {
        let result = self.call("eth_blockNumber", json!([])).await?;
        let s = result.as_str().ok_or(RpcError::BadResponse)?;
        let trimmed = s.strip_prefix("0x").unwrap_or(s);
        u64::from_str_radix(trimmed, 16).map_err(|_| RpcError::BadResponse)
    }

    /// Fetch a full block by number.
    pub async fn get_block(&self, number: u64) -> Result<Block, RpcError> {
        let hex = format!("0x{:x}", number);
        let result = self
            .call("eth_getBlockByNumber", json!([hex, true]))
            .await?;
        if result.is_null() {
            return Err(RpcError::BlockNotFound);
        }
        serde_json::from_value(result).map_err(|_| RpcError::BadResponse)
    }
}
