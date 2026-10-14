//! CLI entry point.

use clap::Parser;
use rustledger::indexer::run_indexer;

#[derive(Parser, Debug)]
#[command(
    name = "rustledger",
    version,
    about = "High-performance Ethereum block indexer"
)]
struct Cli {
    /// Number of recent blocks to fetch.
    #[arg(short, long, default_value_t = 20)]
    count: u64,

    /// Custom JSON-RPC endpoint. Defaults to a public one.
    #[arg(short, long)]
    endpoint: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    println!("rustledger v{}", env!("CARGO_PKG_VERSION"));
    println!("Fetching {} recent blocks...", cli.count);
    println!();

    let stats = run_indexer(cli.endpoint, cli.count).await?;

    println!("=== Results ===");
    println!("Blocks fetched:   {}", stats.blocks_fetched);
    println!("Failures:         {}", stats.failures);
    println!("Transactions:     {}", stats.transactions_seen);
    println!("Total gas used:   {}", stats.total_gas_used);
    println!("Elapsed:          {:.2?}", stats.elapsed);
    println!();
    println!("=== Throughput ===");
    println!("Blocks/sec:       {:.2}", stats.blocks_per_second());
    println!("Transactions/sec: {:.2}", stats.transactions_per_second());
    println!("Avg tx/block:     {:.1}", stats.avg_tx_per_block());

    Ok(())
}
