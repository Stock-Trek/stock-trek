//! Networks within a blockchain.

use crate::dex::blockchain::BlockchainId;
use crate::errors::{StockTrekError, ValueError};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;

/// A network on a blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    network_id: NetworkId,
    chain_id: Option<ChainId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    blockchain_id: Option<BlockchainId>,
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

impl From<String> for NetworkId {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mainnet" => NetworkId::Mainnet,
            "testnet" => NetworkId::Testnet,
            _ => NetworkId::Other(value),
        }
    }
}

impl FromStr for NetworkId {
    type Err = StockTrekError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(StockTrekError::Value(ValueError::NotFound {
                name: "NetworkId".into(),
                key: value.into(),
            }));
        }
        Ok(Self::from(value))
    }
}

#[allow(missing_docs)]
impl NetworkId {
    pub fn new(id: &str) -> Self {
        Self::from(id)
    }

    pub fn as_str(&self) -> &str {
        match self {
            NetworkId::Mainnet => "mainnet",
            NetworkId::Testnet => "testnet",
            NetworkId::Other(name) => name,
        }
    }

    pub fn into_inner(self) -> String {
        match self {
            NetworkId::Mainnet => "mainnet".to_string(),
            NetworkId::Testnet => "testnet".to_string(),
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

impl Deref for NetworkId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl From<NetworkId> for String {
    fn from(value: NetworkId) -> Self {
        value.into_inner()
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

#[allow(missing_docs)]
impl ChainId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ChainId {
    type Err = StockTrekError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse::<u64>().map(Self::new).map_err(|_| {
            StockTrekError::Value(ValueError::IncorrectType {
                expected: "u64".into(),
                found: value.into(),
            })
        })
    }
}

impl AsRef<u64> for ChainId {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl PartialEq<u64> for ChainId {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ChainId> for u64 {
    fn eq(&self, other: &ChainId) -> bool {
        *self == other.0
    }
}

impl From<u64> for ChainId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ChainId> for u64 {
    fn from(value: ChainId) -> Self {
        value.into_inner()
    }
}

#[allow(missing_docs)]
impl Network {
    pub fn new(network_id: NetworkId, chain_id: Option<ChainId>) -> Self {
        Self {
            network_id,
            chain_id,
            blockchain_id: None,
        }
    }

    /// Creates a network that is explicitly associated with a blockchain.
    pub fn for_blockchain(
        blockchain_id: impl Into<BlockchainId>,
        network_id: NetworkId,
        chain_id: Option<ChainId>,
    ) -> Self {
        Self {
            network_id,
            chain_id,
            blockchain_id: Some(blockchain_id.into()),
        }
    }

    pub fn network_id(&self) -> &NetworkId {
        &self.network_id
    }

    pub fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }

    /// Returns the blockchain this network is explicitly associated with, if any.
    pub fn blockchain_id(&self) -> Option<&BlockchainId> {
        self.blockchain_id.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn empty_network_ids_and_invalid_chain_ids_are_rejected_when_parsed() {
        assert!("".parse::<NetworkId>().is_err());
        assert!("not-a-number".parse::<ChainId>().is_err());
    }
}
