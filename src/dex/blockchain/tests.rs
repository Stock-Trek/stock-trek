use super::*;

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
