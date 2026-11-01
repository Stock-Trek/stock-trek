//! Blockchain definitions and address validation.

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

/// A blockchain, its family, and its address format.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    blockchain_id: BlockchainId,
    family_id: FamilyId,
    native_decimals: u8,
    #[serde_as(as = "DisplayFromStr")]
    address_regex: Regex,
}

/// Unique identifier for a blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockchainId(String);

/// Identifier for a blockchain family, such as EVM.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FamilyId(String);

impl BlockchainId {
    /// Creates a blockchain identifier.
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl FamilyId {
    /// Creates a blockchain family identifier.
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl Blockchain {
    /// Creates a blockchain definition.
    ///
    /// # Panics
    ///
    /// Panics if `address_format` is not a valid regular expression.
    pub fn new(
        blockchain_id: BlockchainId,
        family_id: FamilyId,
        native_decimals: u8,
        address_format: String,
    ) -> Self {
        Self {
            blockchain_id,
            family_id,
            native_decimals,
            address_regex: Regex::new(&address_format).expect("Invalid blockchain regex"),
        }
    }
    /// Returns whether `contract_address` matches the blockchain's address format.
    pub fn is_valid_address(&self, contract_address: &str) -> bool {
        self.address_regex.is_match(contract_address)
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.blockchain_id, self.family_id)
    }
}

impl std::fmt::Display for BlockchainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for FamilyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
