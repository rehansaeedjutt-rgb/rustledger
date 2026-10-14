//! rustledger — a high-performance Ethereum block indexer.
//!
//! Purpose: given a block range, fetch blocks over JSON-RPC, parse them
//! into typed Rust structs, and report throughput. This is the same shape
//! of code that blockchain indexers, explorers, and MEV searchers need.

pub mod indexer;
pub mod models;
pub mod rpc;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
