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
