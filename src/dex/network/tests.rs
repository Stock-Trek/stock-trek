use super::*;
use crate::error::dex::DexError;

#[test]
fn network_id_round_trips_through_strings() {
    for network_id in [
        NetworkId::Mainnet,
        NetworkId::Testnet,
        NetworkId::Other("devnet".to_string()),
    ] {
        assert_eq!(
            network_id.as_str().parse::<NetworkId>().unwrap(),
            network_id
        );
        assert_eq!(NetworkId::new(network_id.as_str()).unwrap(), network_id);
    }
}

#[test]
fn mainnet_and_testnet_have_a_single_representation() {
    assert_eq!(NetworkId::new("mainnet").unwrap(), NetworkId::Mainnet);
    assert_eq!(NetworkId::new("testnet").unwrap(), NetworkId::Testnet);
    assert_eq!(
        serde_json::to_string(&NetworkId::new("mainnet").unwrap()).unwrap(),
        serde_json::to_string(&NetworkId::Mainnet).unwrap()
    );
    assert_eq!(
        serde_json::to_string(&NetworkId::new("testnet").unwrap()).unwrap(),
        serde_json::to_string(&NetworkId::Testnet).unwrap()
    );
}

#[test]
fn network_ids_serialize_as_plain_strings() {
    assert_eq!(
        serde_json::to_string(&NetworkId::Other("sepolia".to_string())).unwrap(),
        "\"sepolia\""
    );
    assert_eq!(
        serde_json::from_str::<NetworkId>("\"sepolia\"").unwrap(),
        NetworkId::Other("sepolia".to_string())
    );
}

#[test]
fn empty_and_whitespace_only_network_ids_are_rejected() {
    for value in ["", " ", "\t\n"] {
        assert!(NetworkId::new(value).is_err());
        assert!(NetworkId::try_from(value).is_err());
        assert!(NetworkId::from_static(value).is_err());
        assert!(value.parse::<NetworkId>().is_err());
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
    assert!(serde_json::from_str::<NetworkId>("\"\"").is_err());
    assert!(serde_json::from_str::<NetworkId>("\" \"").is_err());
}
