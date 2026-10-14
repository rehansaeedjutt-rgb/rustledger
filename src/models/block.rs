//! Block and Transaction models.
//!
//! The RPC returns everything as hex strings. We parse into typed values
//! at the boundary so the rest of the code never touches a hex string.

use serde::Deserialize;

/// A single Ethereum block, parsed from the JSON-RPC payload.
#[derive(Debug, Clone, Deserialize)]
pub struct Block {
    pub number: String,
    pub hash: String,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    pub timestamp: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(default)]
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Block number as u64. Panics on malformed hex — this should never
    /// happen against a compliant node, and failing loud is better than
    /// silently returning 0.
    pub fn number_u64(&self) -> u64 {
        parse_hex_u64(&self.number)
    }

    /// Timestamp as seconds since epoch.
    pub fn timestamp_u64(&self) -> u64 {
        parse_hex_u64(&self.timestamp)
    }

    /// Gas used as u64.
    pub fn gas_used_u64(&self) -> u64 {
        parse_hex_u64(&self.gas_used)
    }

    /// Gas limit as u64.
    pub fn gas_limit_u64(&self) -> u64 {
        parse_hex_u64(&self.gas_limit)
    }

    /// Fraction of gas limit consumed (0.0 to 1.0).
    pub fn gas_utilization(&self) -> f64 {
        let limit = self.gas_limit_u64();
        if limit == 0 {
            return 0.0;
        }
        self.gas_used_u64() as f64 / limit as f64
    }

    /// Number of transactions in this block.
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }
}

/// A single transaction within a block.
#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    #[serde(default)]
    pub to: Option<String>,
    pub value: String,
    pub gas: String,
    #[serde(rename = "gasPrice", default)]
    pub gas_price: Option<String>,
    pub nonce: String,
}

impl Transaction {
    /// Value transferred in wei as u128. u64 is not enough — some transfers
    /// exceed 2^64 wei.
    pub fn value_wei(&self) -> u128 {
        parse_hex_u128(&self.value)
    }

    /// True if this is a contract-creation transaction (no `to` field).
    pub fn is_contract_creation(&self) -> bool {
        self.to.is_none()
    }
}

fn parse_hex_u64(s: &str) -> u64 {
    let trimmed = s.strip_prefix("0x").unwrap_or(s);
    u64::from_str_radix(trimmed, 16).unwrap_or(0)
}

fn parse_hex_u128(s: &str) -> u128 {
    let trimmed = s.strip_prefix("0x").unwrap_or(s);
    u128::from_str_radix(trimmed, 16).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex_u64() {
        assert_eq!(parse_hex_u64("0x10"), 16);
        assert_eq!(parse_hex_u64("0x0"), 0);
        assert_eq!(parse_hex_u64("ff"), 255); // missing prefix tolerated
    }

    #[test]
    fn parses_hex_u128() {
        assert_eq!(
            parse_hex_u128("0xde0b6b3a7640000"),
            1_000_000_000_000_000_000
        );
    }
}
