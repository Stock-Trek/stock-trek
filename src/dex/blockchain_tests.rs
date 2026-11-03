use super::blockchain::*;

use crate::error::result::StockTrekResult;

use crate::error::{dex::DexError, result::StockTrekError};

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
fn identifiers_are_trimmed() {
    for blockchain_id in [
        BlockchainId::new(" ethereum ").unwrap(),
        BlockchainId::try_from("\tethereum\n".to_string()).unwrap(),
        " ethereum ".parse::<BlockchainId>().unwrap(),
        serde_json::from_str::<BlockchainId>("\" ethereum \"").unwrap(),
    ] {
        assert_eq!(blockchain_id.as_str(), "ethereum");
        assert_eq!(
            serde_json::to_string(&blockchain_id).unwrap(),
            "\"ethereum\""
        );
    }

    for family_id in [
        FamilyId::new(" evm ").unwrap(),
        FamilyId::try_from("\tevm\n".to_string()).unwrap(),
        " evm ".parse::<FamilyId>().unwrap(),
        serde_json::from_str::<FamilyId>("\" evm \"").unwrap(),
    ] {
        assert_eq!(family_id.as_str(), "evm");
        assert_eq!(serde_json::to_string(&family_id).unwrap(), "\"evm\"");
    }
}

#[test]
fn invalid_identifiers_are_reported_as_dex_errors() {
    for error in [
        BlockchainId::new("").unwrap_err(),
        FamilyId::new(" ").unwrap_err(),
    ] {
        assert!(matches!(
            error,
            StockTrekError::Dex(DexError::InvalidIdentifier { .. })
        ));
    }
}

#[test]
fn identifiers_reject_empty_values_when_deserialized() {
    assert!(serde_json::from_str::<BlockchainId>("\"").is_err());
    assert!(serde_json::from_str::<FamilyId>("\" ").is_err());
}

#[test]
fn rejects_invalid_address_formats_when_deserializing() {
    for address_regex in [".*", "0x[a-fA-F0-9]{40}", "^0x[a-fA-F0-9]{40}", "("] {
        let json = serde_json::json!({
            "blockchain_id": "test",
            "family_id": "test",
            "native_decimals": 18,
            "address_regex": address_regex,
        });

        assert!(
            serde_json::from_value::<Blockchain>(json).is_err(),
            "accepted invalid address format {address_regex:?}"
        );
    }
}

#[test]
fn deserializes_an_anchored_address_format() {
    let json = serde_json::json!({
        "blockchain_id": "test",
        "family_id": "test",
        "native_decimals": 18,
        "address_regex": "^0x[a-fA-F0-9]{40}$",
    });
    let blockchain: Blockchain = serde_json::from_value(json).unwrap();

    assert!(blockchain.is_valid_address("0x0000000000000000000000000000000000000000"));
    assert!(!blockchain.is_valid_address("anything"));
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
