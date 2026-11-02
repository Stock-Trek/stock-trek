//! Networks within a blockchain.

use crate::dex::string_id::string_id_funcs;
use crate::error::{stock_trek_error::StockTrekError, value::ValueError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A network on a blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    network_id: NetworkId,
    chain_id: Option<ChainId>,
}

/// Identifier for a network on a blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct NetworkId(std::borrow::Cow<'static, str>);

impl NetworkId {
    /// The main network of a blockchain.
    pub const MAINNET: Self = Self(std::borrow::Cow::Borrowed("mainnet"));

    /// A test network of a blockchain.
    pub const TESTNET: Self = Self(std::borrow::Cow::Borrowed("testnet"));
}

string_id_funcs!(NetworkId);

/// Numeric chain identifier, such as an EVM chain ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainId(u64);

impl ChainId {
    /// Creates a chain id from its numeric value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// The numeric value of the chain id.
    pub fn value(&self) -> u64 {
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
        value.value()
    }
}

impl Network {
    /// Creates a network from its identifier and, for EVM-family chains, its
    /// chain id.
    pub fn new(network_id: NetworkId, chain_id: Option<ChainId>) -> Self {
        Self {
            network_id,
            chain_id,
        }
    }

    /// The identifier of the network.
    pub fn network_id(&self) -> &NetworkId {
        &self.network_id
    }

    /// The chain id, present only for networks on EVM-family chains.
    pub fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::dex::DexError;

    #[test]
    fn network_id_round_trips_through_strings() {
        for network_id in [
            NetworkId::MAINNET,
            NetworkId::TESTNET,
            NetworkId::new("devnet").unwrap(),
        ] {
            assert_eq!(
                network_id.as_str().parse::<NetworkId>().unwrap(),
                network_id
            );
            assert_eq!(NetworkId::new(network_id.as_str()).unwrap(), network_id);

            let serialized = serde_json::to_string(&network_id).unwrap();
            assert_eq!(
                serde_json::from_str::<NetworkId>(&serialized).unwrap(),
                network_id
            );
        }
    }

    #[test]
    fn mainnet_and_testnet_have_a_single_representation() {
        assert_eq!(NetworkId::new("mainnet").unwrap(), NetworkId::MAINNET);
        assert_eq!(NetworkId::new("testnet").unwrap(), NetworkId::TESTNET);
        assert_eq!(NetworkId::new("mainnet").unwrap(), NetworkId::MAINNET);
        assert_eq!(NetworkId::new("testnet").unwrap(), NetworkId::TESTNET);
        assert_eq!(
            serde_json::to_string(&NetworkId::new("mainnet").unwrap()).unwrap(),
            serde_json::to_string(&NetworkId::MAINNET).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&NetworkId::new("testnet").unwrap()).unwrap(),
            serde_json::to_string(&NetworkId::TESTNET).unwrap()
        );
    }

    #[test]
    fn network_names_are_trimmed_and_canonicalized() {
        assert_eq!(NetworkId::new(" mainnet ").unwrap(), NetworkId::MAINNET);
        assert_eq!(
            NetworkId::new("\tsepolia\n").unwrap(),
            NetworkId::new("sepolia").unwrap()
        );
        assert_eq!(
            serde_json::from_str::<NetworkId>("\" mainnet \"").unwrap(),
            NetworkId::MAINNET
        );

        let network_id = NetworkId::new(" sepolia ").unwrap();
        let serialized = serde_json::to_string(&network_id).unwrap();
        assert_eq!(serialized, "\"sepolia\"");
        assert_eq!(
            serde_json::from_str::<NetworkId>(&serialized).unwrap(),
            network_id
        );
    }

    #[test]
    fn network_ids_serialize_as_plain_strings() {
        let sepolia = NetworkId::new("sepolia").unwrap();
        assert_eq!(serde_json::to_string(&sepolia).unwrap(), "\"sepolia\"");
        assert_eq!(
            serde_json::from_str::<NetworkId>("\"sepolia\"").unwrap(),
            sepolia
        );
    }

    #[test]
    fn empty_and_whitespace_only_network_ids_are_rejected() {
        for value in ["", " ", "\t\n", "\u{a0}"] {
            assert!(NetworkId::new(value).is_err());
            assert!(NetworkId::try_from(value).is_err());
            assert!(value.parse::<NetworkId>().is_err());
            assert!(NetworkId::try_from(value.to_string()).is_err());
        }
    }

    #[test]
    fn invalid_network_id_values_have_consistent_error_semantics() {
        let error = NetworkId::new("").unwrap_err();
        assert!(matches!(
            error,
            StockTrekError::Dex(DexError::InvalidIdentifier { .. })
        ));

        let error = "not-a-number".parse::<ChainId>().unwrap_err();
        assert!(matches!(
            error,
            StockTrekError::Value(ValueError::IncorrectType { .. })
        ));
    }

    #[test]
    fn network_ids_reject_empty_values_when_deserialized() {
        for value in ["\"\"", "\" \"", "\"\t\n\""] {
            assert!(serde_json::from_str::<NetworkId>(value).is_err());
        }
    }
}
