use super::*;
use crate::dex::{
    asset_kind::AssetKind,
    network::{ChainId, NetworkId},
};

#[test]
fn evm_networks_register_expected_chain_and_network_ids() {
    macro_rules! check {
        ($module:ident, $blockchain:literal, [$($variant:ident => ($chain_id:literal, $network_id:expr)),+ $(,)?]) => {
            $(
                assert_eq!(u64::from($module::NetworkName::$variant), $chain_id);

                let network_id: NetworkId = $module::NetworkName::$variant.try_into().unwrap();
                assert_eq!(network_id, $network_id);

                let asset = $module::asset_on_chain(
                    $module::NetworkName::$variant,
                    AssetKind::Native,
                )
                .unwrap();
                assert_eq!(asset.blockchain().blockchain_id(), $blockchain);
                assert_eq!(asset.blockchain().family_id(), FAMILY_ID_EVM);
                assert_eq!(asset.network().network_id(), &network_id);
                assert_eq!(asset.network().chain_id(), Some(ChainId::new($chain_id)));
            )+
        };
    }

    check!(
        arbitrum,
        "arbitrum",
        [
            Mainnet => (42161, NetworkId::MAINNET),
            Sepolia => (421614, NetworkId::new("sepolia").unwrap()),
            Nova => (42170, NetworkId::new("nova").unwrap()),
        ]
    );
    check!(
        avalanche,
        "avalanche",
        [
            Mainnet => (43114, NetworkId::MAINNET),
            Fuji => (43113, NetworkId::new("fuji").unwrap()),
        ]
    );
    check!(
        base,
        "base",
        [
            Mainnet => (8453, NetworkId::MAINNET),
            Sepolia => (84532, NetworkId::new("sepolia").unwrap()),
        ]
    );
    check!(
        bsc,
        "bsc",
        [
            Mainnet => (56, NetworkId::MAINNET),
            Testnet => (97, NetworkId::TESTNET),
        ]
    );
    check!(
        celo,
        "celo",
        [
            Mainnet => (42220, NetworkId::MAINNET),
            Alfajores => (44787, NetworkId::new("alfajores").unwrap()),
        ]
    );
    check!(
        cronos,
        "cronos",
        [
            Mainnet => (25, NetworkId::MAINNET),
            Testnet => (338, NetworkId::TESTNET),
        ]
    );
    check!(
        ethereum,
        "ethereum",
        [
            Mainnet => (1, NetworkId::MAINNET),
            Sepolia => (11155111, NetworkId::new("sepolia").unwrap()),
            Holesky => (17000, NetworkId::new("holesky").unwrap()),
        ]
    );
    check!(
        fantom,
        "fantom",
        [
            Mainnet => (250, NetworkId::MAINNET),
            Testnet => (4002, NetworkId::TESTNET),
        ]
    );
    check!(
        gnosis,
        "gnosis",
        [
            Mainnet => (100, NetworkId::MAINNET),
            Chiado => (10200, NetworkId::new("chiado").unwrap()),
        ]
    );
    check!(
        moonbeam,
        "moonbeam",
        [
            Mainnet => (1284, NetworkId::MAINNET),
            Moonbase => (1287, NetworkId::new("moonbase").unwrap()),
        ]
    );
    check!(
        optimism,
        "optimism",
        [
            Mainnet => (10, NetworkId::MAINNET),
            Sepolia => (11155420, NetworkId::new("sepolia").unwrap()),
        ]
    );
    check!(
        polygon,
        "polygon",
        [
            Mainnet => (137, NetworkId::MAINNET),
            Amoy => (80002, NetworkId::new("amoy").unwrap()),
            Mumbai => (80001, NetworkId::new("mumbai").unwrap()),
        ]
    );
}

#[test]
fn non_evm_networks_register_expected_blockchains() {
    macro_rules! check {
        ($module:ident, $blockchain:literal, $family_id:expr, $native_decimals:literal, [$($variant:ident => $network_id:expr),+ $(,)?]) => {
            $(
                let network_id: NetworkId = $module::NetworkName::$variant.try_into().unwrap();
                assert_eq!(network_id, $network_id);

                let asset = $module::asset_on_chain(
                    $module::NetworkName::$variant,
                    AssetKind::Native,
                )
                .unwrap();
                assert_eq!(asset.blockchain().blockchain_id(), $blockchain);
                assert_eq!(asset.blockchain().family_id(), $family_id);
                assert_eq!(asset.blockchain().native_decimals(), $native_decimals);
                assert_eq!(asset.network().network_id(), &network_id);
                assert_eq!(asset.network().chain_id(), None);
            )+
        };
    }

    check!(
        bitcoin,
        "bitcoin",
        FAMILY_ID_BITCOIN,
        8,
        [
            Mainnet => NetworkId::MAINNET,
            Testnet => NetworkId::TESTNET,
            Signet => NetworkId::new("signet").unwrap(),
            Regtest => NetworkId::new("regtest").unwrap(),
        ]
    );
    check!(
        bitcoin_cash,
        "bitcoin-cash",
        FAMILY_ID_BITCOIN_CASH,
        8,
        [
            Mainnet => NetworkId::MAINNET,
            Testnet => NetworkId::TESTNET,
        ]
    );
    check!(
        cosmos,
        "cosmos",
        FAMILY_ID_COSMOS,
        6,
        [
            Hub => NetworkId::MAINNET,
            Testnet => NetworkId::TESTNET,
        ]
    );
    check!(
        dogecoin,
        "dogecoin",
        FAMILY_ID_DOGECOIN,
        8,
        [
            Mainnet => NetworkId::MAINNET,
            Testnet => NetworkId::TESTNET,
        ]
    );
    check!(
        litecoin,
        "litecoin",
        FAMILY_ID_LITECOIN,
        8,
        [
            Mainnet => NetworkId::MAINNET,
            Testnet => NetworkId::TESTNET,
        ]
    );
    check!(
        near,
        "near",
        FAMILY_ID_NEAR,
        24,
        [
            Mainnet => NetworkId::MAINNET,
            Testnet => NetworkId::TESTNET,
        ]
    );
    check!(
        osmosis,
        "osmosis",
        FAMILY_ID_OSMOSIS,
        6,
        [
            Mainnet => NetworkId::MAINNET,
            Testnet => NetworkId::TESTNET,
        ]
    );
    check!(
        solana,
        "solana",
        FAMILY_ID_SOLANA,
        9,
        [
            Mainnet => NetworkId::MAINNET,
            Testnet => NetworkId::TESTNET,
            Devnet => NetworkId::new("devnet").unwrap(),
        ]
    );
    check!(
        tron,
        "tron",
        FAMILY_ID_TRON,
        6,
        [
            Mainnet => NetworkId::MAINNET,
            Shasta => NetworkId::new("shasta").unwrap(),
            Nile => NetworkId::new("nile").unwrap(),
        ]
    );
}
