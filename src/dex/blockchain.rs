//! Blockchain definitions and address validation.

use crate::dex::string_id::string_id_funcs;
use crate::error::{
    dex::DexError,
    result::{StockTrekError, StockTrekResult},
};
use regex::Regex;
use regex_syntax::hir::{Hir, HirKind, Look};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

/// Unique identifier for a blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BlockchainId(std::borrow::Cow<'static, str>);

string_id_funcs!(BlockchainId);

/// Identifier for a blockchain family, such as EVM.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct FamilyId(std::borrow::Cow<'static, str>);

string_id_funcs!(FamilyId);

/// A blockchain, its family, and its address format.
#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct Blockchain {
    blockchain_id: BlockchainId,
    family_id: FamilyId,
    native_decimals: u8,
    #[serde_as(as = "DisplayFromStr")]
    address_regex: Regex,
}

#[derive(Deserialize)]
struct BlockchainRepr {
    blockchain_id: BlockchainId,
    family_id: FamilyId,
    native_decimals: u8,
    address_regex: String,
}

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

    /// The identifier of the blockchain.
    pub fn blockchain_id(&self) -> &BlockchainId {
        &self.blockchain_id
    }

    /// The identifier of the family the blockchain belongs to, such as EVM.
    pub fn family_id(&self) -> &FamilyId {
        &self.family_id
    }

    /// The number of decimals the blockchain's native asset uses.
    pub fn native_decimals(&self) -> u8 {
        self.native_decimals
    }

    /// The regular expression that addresses must fully match.
    pub fn address_regex(&self) -> &Regex {
        &self.address_regex
    }

    /// The address format as the regular expression source string.
    pub fn address_format(&self) -> &str {
        self.address_regex.as_str()
    }

    /// Returns whether `contract_address` fully matches the blockchain's address format.
    pub fn is_valid_address(&self, contract_address: &str) -> bool {
        self.address_regex.is_match(contract_address)
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

impl<'de> Deserialize<'de> for Blockchain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let repr = BlockchainRepr::deserialize(deserializer)?;
        Self::new(
            repr.blockchain_id,
            repr.family_id,
            repr.native_decimals,
            &repr.address_regex,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.blockchain_id, self.family_id)
    }
}
