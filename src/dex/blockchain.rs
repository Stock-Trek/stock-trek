use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    blockchain_id: BlockchainId,
    family_id: FamilyId,
    native_decimals: u8,
    #[serde_as(as = "DisplayFromStr")]
    address_regex: Regex,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockchainId(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FamilyId(String);

impl BlockchainId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl FamilyId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl Blockchain {
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
    pub fn is_valid_address(&self, contract_address: &str) -> bool {
        self.address_regex.is_match(contract_address)
    }
}
