use super::asset_on_chain::*;

use crate::{
    dex::{asset_kind::AssetKind, blockchain::Blockchain, network::Network},
    error::result::StockTrekResult,
};

use crate::dex::{
    blockchain::{BlockchainId, FamilyId},
    network::{ChainId, NetworkId},
};

fn blockchain() -> Blockchain {
    Blockchain::new(
        BlockchainId::new("ethereum").unwrap(),
        FamilyId::new("evm").unwrap(),
        18,
        "^0x[a-fA-F0-9]{40}$",
    )
    .unwrap()
}

fn network() -> Network {
    Network::new(NetworkId::MAINNET, Some(ChainId::new(1))).unwrap()
}

#[test]
fn creates_a_native_asset() {
    let asset = AssetOnChain::new(blockchain(), network(), AssetKind::Native).unwrap();

    assert_eq!(asset.identifier(), &AssetKind::Native);
}

#[test]
fn creates_a_contract_asset_with_a_valid_address() {
    let address = "0x0000000000000000000000000000000000000000";
    let asset = AssetOnChain::new(
        blockchain(),
        network(),
        AssetKind::Contract(address.to_string()),
    )
    .unwrap();

    assert_eq!(
        asset.identifier(),
        &AssetKind::Contract(address.to_string())
    );
}

#[test]
fn rejects_a_contract_asset_with_an_invalid_address() {
    let result = AssetOnChain::new(
        blockchain(),
        network(),
        AssetKind::Contract("not-an-address".to_string()),
    );

    assert!(result.is_err());
}

#[test]
fn rejects_a_contract_asset_with_an_invalid_address_when_deserializing() {
    let json = serde_json::json!({
        "blockchain": {
            "blockchain_id": "ethereum",
            "family_id": "evm",
            "native_decimals": 18,
            "address_regex": "^0x[a-fA-F0-9]{40}$"
        },
        "network": {
            "network_id": "mainnet",
            "chain_id": 1
        },
        "identifier": {
            "contract": "not-an-address"
        }
    });

    assert!(serde_json::from_value::<AssetOnChain>(json).is_err());
}

#[test]
fn rejects_an_invalid_blockchain_address_format_when_deserializing() {
    let json = serde_json::json!({
        "blockchain": {
            "blockchain_id": "ethereum",
            "family_id": "evm",
            "native_decimals": 18,
            "address_regex": ".*"
        },
        "network": {
            "network_id": "mainnet",
            "chain_id": 1
        },
        "identifier": {
            "contract": "anything"
        }
    });

    assert!(serde_json::from_value::<AssetOnChain>(json).is_err());
}

#[test]
fn round_trips_a_contract_asset_with_a_valid_address() {
    let address = "0x0000000000000000000000000000000000000000";
    let asset = AssetOnChain::new(
        blockchain(),
        network(),
        AssetKind::Contract(address.to_string()),
    )
    .unwrap();

    let json = serde_json::to_string(&asset).unwrap();
    let deserialized: AssetOnChain = serde_json::from_str(&json).unwrap();

    assert_eq!(
        deserialized.identifier(),
        &AssetKind::Contract(address.to_string())
    );
}
