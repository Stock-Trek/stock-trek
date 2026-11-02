//! Blockchain definitions and address validation.

use crate::error::{
    dex::DexError,
    result::{StockTrekError, StockTrekResult},
};
use regex::Regex;
use regex_syntax::hir::{Hir, HirKind, Look};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

use super::string_id::define_string_id;

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

define_string_id! {
    /// Unique identifier for a blockchain.
    BlockchainId
}

define_string_id! {
    /// Identifier for a blockchain family, such as EVM.
    FamilyId
}

#[allow(missing_docs)]
impl Blockchain {
    /// Creates a blockchain definition.
    ///
    /// # Errors
    ///
    /// Returns an error if `address_format` is not a valid regular expression or
    /// is not anchored at both the start and end.
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
        let hir = regex_syntax::parse(address_format).map_err(|error| {
            StockTrekError::Dex(DexError::InvalidAddressFormat {
                blockchain_id: blockchain_id.to_string(),
                message: error.to_string(),
            })
        })?;
        if !Self::has_anchors(&hir) {
            return Err(StockTrekError::Dex(DexError::InvalidAddressFormat {
                blockchain_id: blockchain_id.to_string(),
                message: "address format must start and end with an anchor".into(),
            }));
        }
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

    /// Returns whether `contract_address` fully matches the blockchain's address format.
    pub fn is_valid_address(&self, contract_address: &str) -> bool {
        self.address_regex
            .find(contract_address)
            .is_some_and(|matched| matched.start() == 0 && matched.end() == contract_address.len())
    }

    fn has_anchors(hir: &Hir) -> bool {
        Self::starts_anchored(hir) && Self::ends_anchored(hir)
    }

    fn starts_anchored(hir: &Hir) -> bool {
        match hir.kind() {
            HirKind::Look(Look::Start) => true,
            HirKind::Capture(capture) => Self::starts_anchored(&capture.sub),
            HirKind::Concat(items) => items.first().is_some_and(Self::starts_anchored),
            _ => false,
        }
    }

    fn ends_anchored(hir: &Hir) -> bool {
        match hir.kind() {
            HirKind::Look(Look::End) => true,
            HirKind::Capture(capture) => Self::ends_anchored(&capture.sub),
            HirKind::Concat(items) => items.last().is_some_and(Self::ends_anchored),
            _ => false,
        }
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.blockchain_id, self.family_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::value::ValueError;

    fn blockchain(address_format: &str) -> StockTrekResult<Blockchain> {
        Blockchain::new(
            BlockchainId::new("test")?,
            FamilyId::new("test")?,
            18,
            address_format,
        )
    }

    #[test]
    fn creates_a_blockchain_with_a_valid_address_format() {
        let blockchain = blockchain("^0x[a-fA-F0-9]{40}$").unwrap();

        assert!(blockchain.is_valid_address("0x0000000000000000000000000000000000000000"));
        assert!(!blockchain.is_valid_address("not-an-address"));
        assert!(!blockchain.is_valid_address("prefix-0x0000000000000000000000000000000000000000"));
        assert!(!blockchain.is_valid_address("0x0000000000000000000000000000000000000000-suffix"));
    }

    #[test]
    fn returns_an_error_for_an_invalid_address_format() {
        assert!(blockchain("(").is_err());
    }

    #[test]
    fn empty_and_whitespace_only_identifiers_are_rejected() {
        for value in ["", " ", "\t\n"] {
            assert!(BlockchainId::new(value).is_err());
            assert!(BlockchainId::try_from(value).is_err());
            assert!(value.parse::<BlockchainId>().is_err());
            assert!(FamilyId::new(value).is_err());
            assert!(FamilyId::try_from(value).is_err());
            assert!(value.parse::<FamilyId>().is_err());
        }
    }

    #[test]
    fn invalid_identifiers_are_reported_as_incorrect_types() {
        let error = BlockchainId::new("").unwrap_err();

        assert!(matches!(
            error,
            StockTrekError::Value(ValueError::IncorrectType { .. })
        ));
    }

    #[test]
    fn identifiers_reject_empty_values_when_deserialized() {
        assert!(serde_json::from_str::<BlockchainId>("\"\"").is_err());
        assert!(serde_json::from_str::<FamilyId>("\" \"").is_err());
    }

    #[test]
    fn returns_an_error_for_an_unanchored_address_format() {
        assert!(blockchain("0x[a-fA-F0-9]{40}").is_err());
        assert!(blockchain("^0x[a-fA-F0-9]{40}").is_err());
        assert!(blockchain("0x[a-fA-F0-9]{40}$").is_err());
    }

    #[test]
    fn returns_an_error_for_an_unanchored_address_format_with_scoped_flags() {
        assert!(blockchain("(?i:0x[a-f0-9]{40})").is_err());
        assert!(blockchain("(?i:^0x[a-f0-9]{40})").is_err());
        assert!(blockchain("(?i:0x[a-f0-9]{40}$)").is_err());
    }

    #[test]
    fn accepts_an_address_format_with_scoped_flags() {
        let blockchain = blockchain("(?i:^0x[a-f0-9]{40}$)").unwrap();

        assert!(blockchain.is_valid_address("0x0000000000000000000000000000000000000000"));
        assert!(blockchain.is_valid_address("0X0000000000000000000000000000000000000000"));
        assert!(!blockchain.is_valid_address("not-an-address"));
        assert!(!blockchain.is_valid_address("0x000000000000000000000000000000000000000"));
    }

    #[test]
    fn preserves_anchored_address_formats() {
        assert_eq!(
            blockchain(r"\A0x[a-fA-F0-9]{40}\z")
                .unwrap()
                .address_format(),
            r"\A0x[a-fA-F0-9]{40}\z"
        );
        assert_eq!(
            blockchain("(?i)^0x[a-f0-9]{40}$").unwrap().address_format(),
            "(?i)^0x[a-f0-9]{40}$"
        );
    }
}
