//! Networks within a blockchain.

use crate::dex::string_id::string_id_funcs;
use crate::error::{result::StockTrekError, value::ValueError};
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
