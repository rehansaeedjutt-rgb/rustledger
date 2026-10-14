//! Block indexer: fetch a range and report throughput.

pub mod runner;

pub use runner::{IndexerStats, run_indexer};
