use super::network::*;

use crate::error::{result::StockTrekError, value::ValueError};

use crate::error::dex::DexError;

#[test]
fn network_new_accepts_valid_ids_and_chain_ids() {
    let network = Network::new(NetworkId::MAINNET, Some(ChainId::new(1))).unwrap();
    assert_eq!(network.network_id(), &NetworkId::MAINNET);
    assert_eq!(network.chain_id(), Some(ChainId::new(1)));

    let network = Network::new(NetworkId::new("sepolia").unwrap(), None).unwrap();
    assert_eq!(network.network_id(), &NetworkId::new("sepolia").unwrap());
    assert_eq!(network.chain_id(), None);
}

#[test]
fn network_deserialization_rejects_invalid_network_ids() {
    for network_id in ["", " ", "\t\n"] {
        let json = serde_json::json!({
            "network_id": network_id,
            "chain_id": 1
        });
        assert!(serde_json::from_value::<Network>(json).is_err());
    }
}

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
