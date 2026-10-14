# rustledger

> High-performance Ethereum block indexer, written in Rust.

[![CI](https://github.com/rehansaeedjutt-rgb/rustledger/actions/workflows/ci.yml/badge.svg)](https://github.com/rehansaeedjutt-rgb/rustledger/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust 1.99+](https://img.shields.io/badge/rust-1.99%2B-orange.svg)](https://www.rust-lang.org/)

## What this is

`rustledger` fetches Ethereum blocks over JSON-RPC, parses them into typed
Rust structs, and reports throughput. It is the same shape of code that
blockchain explorers, indexers, and MEV searchers need on day one.

This is my first serious Rust project. I picked a blockchain indexer because
it exercises the language's strengths — async I/O, zero-copy parsing, strong
typing — against real mainnet data, not a toy example.

## Live output

Ran against public Ethereum mainnet (block range: last 20 blocks):

    $ rustledger --count 20
    rustledger v0.1.0
    Fetching 20 recent blocks...

    === Results ===
    Blocks fetched:   20
    Failures:         0
    Transactions:     5684
    Total gas used:   619032183
    Elapsed:          10.08s

    === Throughput ===
    Blocks/sec:       1.98
    Transactions/sec: 563.92
    Avg tx/block:     284.2

Numbers depend on the public RPC endpoint and network latency. On a local
node, blocks/sec is limited by the node, not by this program.

## What is in here

- **Async RPC client** — `reqwest` with `rustls`, no OpenSSL dependency.
- **Typed models** — hex strings parsed into `u64` / `u128` at the boundary.
- **Range indexer** — fetches a sliding window of recent blocks.
- **Throughput stats** — blocks/sec, transactions/sec, gas utilization.
- **CLI** — `clap` with a clean interface.
- **2 unit tests** — hex parsing, plus room to grow.

## What is not here yet (deliberately)

- **No persistence.** Blocks are fetched, reported, and dropped. Same
  pattern as `tickstream` before I added SQLite.
- **No contract log parsing.** Only top-level transactions. ERC-20 transfer
  detection needs `eth_getLogs` and ABI decoding.
- **No reorg handling.** A real indexer needs to detect and unwind reorgs.
- **No tests against a real RPC in CI.** The CI runs offline; live tests
  would hammer public endpoints on every push.
- **No backpressure.** Fetches run sequentially. A production indexer
  would pipeline with bounded channels.

## Why Rust for this

Python and TypeScript are fine for research. But a production indexer is
the kind of thing that runs 24/7, holds gigabytes of state, and competes
for microseconds. Rust gives me all of that without a garbage collector
pause, and the type system catches most of the bugs I would otherwise
find in production.

I am not claiming rustledger is production-ready. I am claiming it is a
real Rust program against real data, and the code is honest about what it
does and does not do.

## Install and run

### Prerequisites

- Rust 1.99+ (`rustup` recommended)
- A network connection (uses public Ethereum RPC)

On Windows, you will need a MinGW-w64 toolchain if you use the GNU Rust
target. See the "Windows setup notes" section below.

### Build

    git clone https://github.com/rehansaeedjutt-rgb/rustledger.git
    cd rustledger
    cargo build --release

### Run

    ./target/release/rustledger --count 20

    # Or point at your own endpoint
    ./target/release/rustledger --count 100 --endpoint https://your-node.example.com

### Test and lint

    cargo test
    cargo clippy -- -D warnings

## Project structure

    rustledger/
    ├── src/
    │   ├── main.rs            # CLI entry point
    │   ├── lib.rs             # Library root
    │   ├── models/
    │   │   ├── mod.rs
    │   │   └── block.rs       # Block and Transaction models
    │   ├── rpc/
    │   │   ├── mod.rs
    │   │   └── client.rs      # Async JSON-RPC client
    │   └── indexer/
    │       ├── mod.rs
    │       └── runner.rs      # Range fetcher + throughput stats
    ├── tests/
    ├── Cargo.toml
    └── README.md

## Design notes

**Parse at the boundary.** The RPC returns every field as a hex string.
I parse to `u64`/`u128` immediately in the model's accessor methods, so
downstream code never touches a hex string. This is the same discipline
I use in Python with Pydantic.

**Failures are counted, not fatal.** If one block fails to fetch, the run
continues. I want to see throughput with failures, not fail on the first
hiccup. A production system would add retry with jitter, not just count.

**`rustls` over OpenSSL.** Not just because it makes Windows builds easier
(no OpenSSL DLL to ship), but because `rustls` is a Rust-native TLS stack
and does not need to be audited separately as a C library.

**Sequential fetches for now.** A parallel fetcher with bounded channels
would be faster, but the added complexity is not justified at 20 blocks.
The `tokio` runtime is already there; `join_all` is a five-line change.

## Windows setup notes

Rust on Windows supports two toolchains: MSVC (default) and GNU. I use GNU
because it does not require Visual Studio Build Tools (3 GB) just to link
a binary.

To build on Windows:

1. Install Rust via rustup: `rustup toolchain install stable-x86_64-pc-windows-gnu`
2. Install a MinGW-w64 toolchain (I use [WinLibs](https://winlibs.com/))
3. Add the `bin/` folder to `PATH`
4. `rustup default stable-x86_64-pc-windows-gnu`
5. `cargo build --release`

Details are in `docs/windows-setup.md` (to be written).

## Roadmap

- [ ] Parallel block fetching with bounded concurrency
- [ ] SQLite persistence (mirrors the pattern from `tickstream`)
- [ ] ERC-20 transfer log parsing via `eth_getLogs`
- [ ] Reorg detection and rollback
- [ ] Prometheus-compatible metrics endpoint
- [ ] Benchmarks using `criterion.rs`

## Disclaimer

Educational project. Not audited. Not financial advice.

## License

MIT — see [LICENSE](LICENSE).

## Author

**Muhammad Rehan Saeed** — [GitHub](https://github.com/rehansaeedjutt-rgb)