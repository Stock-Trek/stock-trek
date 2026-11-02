//! Networks within a blockchain.

use crate::error::{result::StockTrekError, value::ValueError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::string_id::define_string_id;

/// A network on a blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    network_id: NetworkId,
    chain_id: Option<ChainId>,
}

define_string_id! {
    /// Identifier for a network on a blockchain.
    NetworkId {
        /// The main network of a blockchain.
        #[allow(non_upper_case_globals)]
        pub const Mainnet: Self = Self::from_static("mainnet");

        /// A test network of a blockchain.
        #[allow(non_upper_case_globals)]
        pub const Testnet: Self = Self::from_static("testnet");

        /// Creates a network identifier from a static string.
        ///
        /// # Panics
        ///
        /// Panics if `id` is empty.
        pub const fn from_static(id: &'static str) -> Self {
            assert!(!id.is_empty(), "network id must not be empty");
            Self(std::borrow::Cow::Borrowed(id))
        }

        /// Creates a network identifier for an arbitrary network name.
        ///
        /// # Panics
        ///
        /// Panics if `name` is empty or whitespace-only.
        #[allow(non_snake_case)]
        pub fn Other(name: impl Into<String>) -> Self {
            let name = name.into();
            Self::try_from(name).expect("network id must not be empty or whitespace-only")
        }
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
        }
    }

    pub fn network_id(&self) -> &NetworkId {
        &self.network_id
    }

    pub fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
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
            assert_eq!(NetworkId::new(network_id.as_str()).unwrap(), network_id);
        }
    }

    #[test]
    fn mainnet_and_testnet_have_a_single_representation() {
        assert_eq!(NetworkId::Other("mainnet"), NetworkId::Mainnet);
        assert_eq!(NetworkId::Other("testnet"), NetworkId::Testnet);
        assert_eq!(
            serde_json::to_string(&NetworkId::Other("mainnet")).unwrap(),
            serde_json::to_string(&NetworkId::Mainnet).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&NetworkId::Other("testnet")).unwrap(),
            serde_json::to_string(&NetworkId::Testnet).unwrap()
        );
    }

    #[test]
    fn network_ids_serialize_as_plain_strings() {
        assert_eq!(
            serde_json::to_string(&NetworkId::Other("sepolia")).unwrap(),
            "\"sepolia\""
        );
        assert_eq!(
            serde_json::from_str::<NetworkId>("\"sepolia\"").unwrap(),
            NetworkId::Other("sepolia")
        );
    }

    #[test]
    fn empty_and_whitespace_only_network_ids_are_rejected() {
        for value in ["", " ", "\t\n"] {
            assert!(NetworkId::new(value).is_err());
            assert!(NetworkId::try_from(value).is_err());
            assert!(value.parse::<NetworkId>().is_err());
        }
    }

    #[test]
    fn invalid_network_id_values_have_consistent_error_semantics() {
        let error = NetworkId::new("").unwrap_err();
        assert!(matches!(
            error,
            StockTrekError::Value(ValueError::IncorrectType { .. })
        ));

        let error = "not-a-number".parse::<ChainId>().unwrap_err();
        assert!(matches!(
            error,
            StockTrekError::Value(ValueError::IncorrectType { .. })
        ));
    }

    #[test]
    fn network_ids_reject_empty_values_when_deserialized() {
        assert!(serde_json::from_str::<NetworkId>("\"\"").is_err());
        assert!(serde_json::from_str::<NetworkId>("\" \"").is_err());
    }
}
