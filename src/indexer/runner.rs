//! Range fetcher and stats collector.

use crate::models::Block;
use crate::rpc::{RpcClient, RpcError};
use std::time::{Duration, Instant};

/// Stats from a completed indexer run.
#[derive(Debug, Clone)]
pub struct IndexerStats {
    pub blocks_fetched: u64,
    pub transactions_seen: u64,
    pub total_gas_used: u64,
    pub elapsed: Duration,
    pub failures: u64,
}

impl IndexerStats {
    /// Blocks per second across the whole run.
    pub fn blocks_per_second(&self) -> f64 {
        let secs = self.elapsed.as_secs_f64();
        if secs == 0.0 {
            return 0.0;
        }
        self.blocks_fetched as f64 / secs
    }

    /// Transactions per second.
    pub fn transactions_per_second(&self) -> f64 {
        let secs = self.elapsed.as_secs_f64();
        if secs == 0.0 {
            return 0.0;
        }
        self.transactions_seen as f64 / secs
    }

    /// Average transactions per block.
    pub fn avg_tx_per_block(&self) -> f64 {
        if self.blocks_fetched == 0 {
            return 0.0;
        }
        self.transactions_seen as f64 / self.blocks_fetched as f64
    }
}

/// Fetch `count` blocks ending at the current head and collect stats.
///
/// Failures are counted but do not stop the run — I would rather see
/// throughput with 2 failures out of 20 than fail on the first hiccup.
pub async fn run_indexer(endpoint: Option<String>, count: u64) -> Result<IndexerStats, RpcError> {
    let client = match endpoint {
        Some(url) => RpcClient::new(url),
        None => RpcClient::public(),
    };

    let head = client.block_number().await?;
    let start = head.saturating_sub(count);

    let mut stats = IndexerStats {
        blocks_fetched: 0,
        transactions_seen: 0,
        total_gas_used: 0,
        elapsed: Duration::ZERO,
        failures: 0,
    };

    let begin = Instant::now();
    for n in (start + 1)..=head {
        match client.get_block(n).await {
            Ok(block) => {
                stats.blocks_fetched += 1;
                stats.transactions_seen += block.transaction_count() as u64;
                stats.total_gas_used += block.gas_used_u64();
                let _ = block; // silence unused warning for now
            }
            Err(_) => {
                stats.failures += 1;
            }
        }
    }
    stats.elapsed = begin.elapsed();
    Ok(stats)
}

/// Used by the CLI to print a compact per-block line.
pub fn format_block_summary(b: &Block) -> String {
    format!(
        "block {:>10}  txs={:>4}  gas={:>10}  util={:.1}%",
        b.number_u64(),
        b.transaction_count(),
        b.gas_used_u64(),
        b.gas_utilization() * 100.0
    )
}
