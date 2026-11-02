//! Networks within a blockchain.

use crate::errors::{StockTrekError, ValueError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
        match value {
            "mainnet" => NetworkId::Mainnet,
            "testnet" => NetworkId::Testnet,
            name => NetworkId::Other(name.to_string()),
        }
    }
}

impl FromStr for NetworkId {
    type Err = StockTrekError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "mainnet" => Ok(NetworkId::Mainnet),
            "testnet" => Ok(NetworkId::Testnet),
            "" => Err(StockTrekError::Value(ValueError::NotFound {
                name: "NetworkId".into(),
                key: value.into(),
            })),
            name => Ok(NetworkId::Other(name.to_string())),
        }
    }
}

impl NetworkId {
    /// Returns the network identifier as a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            NetworkId::Mainnet => "mainnet",
            NetworkId::Testnet => "testnet",
            NetworkId::Other(name) => name,
        }
    }
}

impl std::fmt::Display for NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for NetworkId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for NetworkId {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for NetworkId {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
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

    /// Returns the numeric chain identifier.
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Consumes the chain identifier and returns its numeric value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl From<ChainId> for u64 {
    fn from(value: ChainId) -> Self {
        value.into_inner()
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

    /// Returns the network identifier.
    pub fn network_id(&self) -> &NetworkId {
        &self.network_id
    }

    /// Returns the network's numeric chain identifier, if it has one.
    pub fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_network_fields() {
        let network = Network::new(NetworkId::Mainnet, Some(ChainId::new(1)));

        assert_eq!(network.network_id(), &NetworkId::Mainnet);
        assert_eq!(network.chain_id(), Some(ChainId::new(1)));
        assert_eq!(network.chain_id().unwrap().value(), 1);
    }

    #[test]
    fn network_id_round_trips_through_strings() {
        for network_id in [
            NetworkId::Mainnet,
            NetworkId::Testnet,
            NetworkId::Other("devnet".to_string()),
        ] {
            assert_eq!(
                network_id.as_str().parse::<NetworkId>().unwrap(),
                network_id
            );
            assert_eq!(NetworkId::from(network_id.as_str()), network_id);
        }
    }
}
