//! Networks within a blockchain.

use serde::{Deserialize, Serialize};

/// A network on a blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    network_id: NetworkId,
    chain_id: Option<ChainId>,
}

/// Well-known network identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum NetworkId {
    /// The production network.
    Mainnet,
    /// A test network.
    Testnet,
    /// Any other named network.
    Other(String),
}

impl From<&str> for NetworkId {
    fn from(value: &str) -> Self {
        NetworkId::Other(value.to_string())
    }
}

impl std::fmt::Display for NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkId::Mainnet => write!(f, "mainnet"),
            NetworkId::Testnet => write!(f, "testnet"),
            NetworkId::Other(name) => write!(f, "{}", name),
        }
    }
}

/// Numeric chain identifier, such as an EVM chain ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainId(u64);

impl ChainId {
    /// Creates a chain identifier.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Network {
    /// Creates a network on a blockchain.
    pub fn new(network_id: NetworkId, chain_id: Option<ChainId>) -> Self {
        Self {
            network_id,
            chain_id,
        }
    }
}
