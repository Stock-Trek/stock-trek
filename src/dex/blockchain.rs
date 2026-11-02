//! Blockchain definitions and address validation.

use crate::error::{
    dex::DexError,
    result::{StockTrekError, StockTrekResult},
    value::ValueError,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use std::ops::Deref;
use std::str::FromStr;

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

#[allow(missing_docs)]
impl BlockchainId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[allow(missing_docs)]
impl FamilyId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for BlockchainId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for BlockchainId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl FromStr for BlockchainId {
    type Err = StockTrekError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(StockTrekError::Value(ValueError::NotFound {
                name: "BlockchainId".into(),
                key: value.into(),
            }));
        }
        Ok(Self::new(value))
    }
}

impl From<&str> for FamilyId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for FamilyId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl FromStr for FamilyId {
    type Err = StockTrekError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(StockTrekError::Value(ValueError::NotFound {
                name: "FamilyId".into(),
                key: value.into(),
            }));
        }
        Ok(Self::new(value))
    }
}

#[allow(missing_docs)]
impl Blockchain {
    /// Creates a blockchain definition.
    ///
    /// # Errors
    ///
    /// Returns an error if `address_format` is not a valid regular expression.
    pub fn new(
        blockchain_id: BlockchainId,
        family_id: FamilyId,
        native_decimals: u8,
        address_format: &str,
    ) -> StockTrekResult<Self> {
        let address_regex = Regex::new(address_format).map_err(|error| {
            StockTrekError::Dex(DexError::InvalidAddressFormat {
                blockchain_id: blockchain_id.to_string(),
                message: error.to_string(),
            })
        })?;
        Ok(Self {
            blockchain_id,
            family_id,
            native_decimals,
            address_regex,
        })
    }

    pub fn blockchain_id(&self) -> &BlockchainId {
        &self.blockchain_id
    }

    pub fn family_id(&self) -> &FamilyId {
        &self.family_id
    }

    pub fn native_decimals(&self) -> u8 {
        self.native_decimals
    }

    pub fn address_regex(&self) -> &Regex {
        &self.address_regex
    }

    pub fn address_format(&self) -> &str {
        self.address_regex.as_str()
    }

    /// Returns whether `contract_address` matches the blockchain's address format.
    pub fn is_valid_address(&self, contract_address: &str) -> bool {
        self.address_regex.is_match(contract_address)
    }

    /// Validates that `contract_address` matches the blockchain's address format.
    ///
    /// # Errors
    ///
    /// Returns an error if `contract_address` does not match the address format.
    pub fn validate_address(&self, contract_address: &str) -> StockTrekResult<()> {
        if self.is_valid_address(contract_address) {
            return Ok(());
        }

        Err(StockTrekError::Dex(DexError::InvalidAddress {
            blockchain_id: self.blockchain_id.to_string(),
            address: contract_address.to_string(),
        }))
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

impl AsRef<str> for BlockchainId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for FamilyId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for BlockchainId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Deref for FamilyId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl PartialEq<str> for BlockchainId {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for BlockchainId {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<str> for FamilyId {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for FamilyId {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl From<BlockchainId> for String {
    fn from(value: BlockchainId) -> Self {
        value.into_inner()
    }
}

impl From<FamilyId> for String {
    fn from(value: FamilyId) -> Self {
        value.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blockchain(address_format: &str) -> StockTrekResult<Blockchain> {
        Blockchain::new(
            BlockchainId::new("test"),
            FamilyId::new("test"),
            18,
            address_format,
        )
    }

    #[test]
    fn creates_a_blockchain_with_a_valid_address_format() {
        let blockchain = blockchain("^0x[a-fA-F0-9]{40}$").unwrap();

        assert!(blockchain.is_valid_address("0x0000000000000000000000000000000000000000"));
        assert!(!blockchain.is_valid_address("not-an-address"));
    }

    #[test]
    fn returns_an_error_for_an_invalid_address_format() {
        assert!(blockchain("(").is_err());
    }

    #[test]
    fn empty_blockchain_ids_are_rejected_when_parsed() {
        assert!("".parse::<BlockchainId>().is_err());
        assert!("".parse::<FamilyId>().is_err());
    }
}
