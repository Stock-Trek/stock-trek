//! Networks within a blockchain.

use crate::dex::string_id::string_id_funcs;
use crate::error::{
    result::{StockTrekError, StockTrekResult},
    value::ValueError,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A network on a blockchain.
#[derive(Debug, Clone, Serialize)]
pub struct Network {
    network_id: NetworkId,
    chain_id: Option<ChainId>,
}

#[derive(Deserialize)]
struct NetworkRepr {
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

    /// The Alfajores test network on Celo.
    pub const ALFAJORES: Self = Self(std::borrow::Cow::Borrowed("alfajores"));

    /// The Amoy test network on Polygon.
    pub const AMOY: Self = Self(std::borrow::Cow::Borrowed("amoy"));

    /// The Chiado test network on Gnosis.
    pub const CHIADO: Self = Self(std::borrow::Cow::Borrowed("chiado"));

    /// The devnet network on Solana.
    pub const DEVNET: Self = Self(std::borrow::Cow::Borrowed("devnet"));

    /// The Fuji test network on Avalanche.
    pub const FUJI: Self = Self(std::borrow::Cow::Borrowed("fuji"));

    /// The Holesky test network on Ethereum.
    pub const HOLESKY: Self = Self(std::borrow::Cow::Borrowed("holesky"));

    /// The Moonbase test network on Moonbeam.
    pub const MOONBASE: Self = Self(std::borrow::Cow::Borrowed("moonbase"));

    /// The Mumbai test network on Polygon.
    pub const MUMBAI: Self = Self(std::borrow::Cow::Borrowed("mumbai"));

    /// The Nile test network on Tron.
    pub const NILE: Self = Self(std::borrow::Cow::Borrowed("nile"));

    /// The Nova network on Arbitrum.
    pub const NOVA: Self = Self(std::borrow::Cow::Borrowed("nova"));

    /// The Regtest network on Bitcoin.
    pub const REGTEST: Self = Self(std::borrow::Cow::Borrowed("regtest"));

    /// The Sepolia test network.
    pub const SEPOLIA: Self = Self(std::borrow::Cow::Borrowed("sepolia"));

    /// The Shasta test network on Tron.
    pub const SHASTA: Self = Self(std::borrow::Cow::Borrowed("shasta"));

    /// The Signet test network on Bitcoin.
    pub const SIGNET: Self = Self(std::borrow::Cow::Borrowed("signet"));
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

impl Network {
    /// Creates a network from its identifier and, for EVM-family chains, its
    /// chain id.
    ///
    /// # Errors
    ///
    /// Returns an error if the network identifier is invalid.
    pub fn new(network_id: NetworkId, chain_id: Option<ChainId>) -> StockTrekResult<Self> {
        NetworkId::validate(network_id.as_str())?;
        Ok(Self {
            network_id,
            chain_id,
        })
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

impl<'de> Deserialize<'de> for Network {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let repr = NetworkRepr::deserialize(deserializer)?;
        Self::new(repr.network_id, repr.chain_id).map_err(serde::de::Error::custom)
    }
}
