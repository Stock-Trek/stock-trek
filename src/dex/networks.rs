use crate::dex::{
    asset_kind::AssetKind,
    asset_on_chain::AssetOnChain,
    blockchain::{Blockchain, BlockchainId, FamilyId},
    network::{ChainId, Network, NetworkId},
};
use crate::error::result::StockTrekResult;

pub const FAMILY_ID_BITCOIN: &str = "bitcoin";
pub const FAMILY_ID_BITCOIN_CASH: &str = "bitcoin-cash";
pub const FAMILY_ID_COSMOS: &str = "cosmos";
pub const FAMILY_ID_DOGECOIN: &str = "dogecoin";
pub const FAMILY_ID_EVM: &str = "evm";
pub const FAMILY_ID_LITECOIN: &str = "litecoin";
pub const FAMILY_ID_NEAR: &str = "near";
pub const FAMILY_ID_OSMOSIS: &str = "osmosis";
pub const FAMILY_ID_SOLANA: &str = "solana";
pub const FAMILY_ID_TRON: &str = "tron";

macro_rules! define_network_name {
    ($($variant:ident => $network_id:expr),+ $(,)?) => {
        #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        #[non_exhaustive]
        pub enum NetworkName {
            $($variant,)+
        }

        impl From<NetworkName> for NetworkId {
            fn from(value: NetworkName) -> Self {
                match value {
                    $(NetworkName::$variant => $network_id,)+
                }
            }
        }
    };
}

macro_rules! define_evm_network {
    (
        $module:ident,
        $blockchain:literal,
        {
            $($variant:ident => ($chain_id:literal, $network_id:expr)),+ $(,)?
        }
    ) => {
        pub mod $module {
            use crate::dex::network::NetworkId;
            use crate::error::result::StockTrekResult;
            use serde::{Deserialize, Serialize};
            use strum::Display;

            define_network_name! {
                $($variant => $network_id,)+
            }

            impl From<NetworkName> for u64 {
                fn from(value: NetworkName) -> Self {
                    match value {
                        $(NetworkName::$variant => $chain_id,)+
                    }
                }
            }

            pub(crate) fn supports_network(network: &crate::dex::network::Network) -> bool {
                $(
                    (network.network_id() == &{
                        let network_id: NetworkId = $network_id;
                        network_id
                    }
                        && network.chain_id()
                            == Some(crate::dex::network::ChainId::new($chain_id)))
                        ||)+ false
            }

            pub fn asset_on_chain(
                network: NetworkName,
                asset_kind: crate::dex::asset_kind::AssetKind,
            ) -> StockTrekResult<crate::dex::asset_on_chain::AssetOnChain> {
                let networks = vec![
                    $(
                        crate::dex::network::Network::for_blockchain(
                            crate::dex::blockchain::BlockchainId::new($blockchain),
                            $network_id,
                            Some(crate::dex::network::ChainId::new($chain_id)),
                        )
                    ),+
                ];
                super::evm_asset_on_chain($blockchain, network, asset_kind, networks)
            }
        }
    };
}

macro_rules! define_network {
    (
        $module:ident,
        $blockchain:literal,
        $family_id:ident,
        $native_decimals:literal,
        $address_format:literal,
        {
            $($variant:ident => $network_id:expr),+ $(,)?
        }
    ) => {
        pub mod $module {
            use crate::dex::network::{Network, NetworkId};
            use crate::error::result::StockTrekResult;
            use serde::{Deserialize, Serialize};
            use strum::Display;

            define_network_name! {
                $($variant => $network_id,)+
            }

            pub(crate) fn supports_network(network: &Network) -> bool {
                network.chain_id().is_none()
                    $(&& network.network_id() == &{
                        let network_id: NetworkId = $network_id;
                        network_id
                    })*
            }

            pub fn asset_on_chain(
                network: NetworkName,
                asset_kind: crate::dex::asset_kind::AssetKind,
            ) -> StockTrekResult<crate::dex::asset_on_chain::AssetOnChain> {
                let network = Network::for_blockchain(
                    crate::dex::blockchain::BlockchainId::new($blockchain),
                    network.into(),
                    None,
                );
                let networks = vec![
                    $(
                        Network::for_blockchain(
                            crate::dex::blockchain::BlockchainId::new($blockchain),
                            $network_id,
                            None,
                        )
                    ),+
                ];
                let blockchain = super::blockchain(
                    $blockchain,
                    super::$family_id,
                    $native_decimals,
                    $address_format,
                    networks,
                )?;
                crate::dex::asset_on_chain::AssetOnChain::new(
                    blockchain,
                    network,
                    asset_kind,
                )
            }
        }
    };
}

define_evm_network! {
    arbitrum,
    "arbitrum",
    {
        Mainnet => (42161, NetworkId::Mainnet),
        Sepolia => (421614, "sepolia".into()),
        Nova => (42170, "nova".into()),
    }
}

define_evm_network! {
    avalanche,
    "avalanche",
    {
        Mainnet => (43114, NetworkId::Mainnet),
        Fuji => (43113, "fuji".into()),
    }
}

define_evm_network! {
    base,
    "base",
    {
        Mainnet => (8453, NetworkId::Mainnet),
        Sepolia => (84532, "sepolia".into()),
    }
}

define_network! {
    bitcoin,
    "bitcoin",
    FAMILY_ID_BITCOIN,
    8,
    r"^([13][1-9A-HJ-NP-Za-km-z]{25,34}|bc1[ac-hj-np-z02-9]{39,59}|bc1p[ac-hj-np-z02-9]{58,59})$",
    {
        Mainnet => NetworkId::Mainnet,
        Testnet => NetworkId::Testnet,
        Signet => "signet".into(),
        Regtest => "regtest".into(),
    }
}

define_network! {
    bitcoin_cash,
    "bitcoin-cash",
    FAMILY_ID_BITCOIN_CASH,
    8,
    "^(bitcoincash:)?(q|p)[a-zA-HJ-NP-Z0-9]{41,42}$",
    {
        Mainnet => NetworkId::Mainnet,
        Testnet => NetworkId::Testnet,
    }
}

define_evm_network! {
    bsc,
    "bsc",
    {
        Mainnet => (56, NetworkId::Mainnet),
        Testnet => (97, NetworkId::Testnet),
    }
}

define_evm_network! {
    celo,
    "celo",
    {
        Mainnet => (42220, NetworkId::Mainnet),
        Alfajores => (44787, "alfajores".into()),
    }
}

define_network! {
    cosmos,
    "cosmos",
    FAMILY_ID_COSMOS,
    6,
    "^cosmos1[a-zA-HJ-NP-Z0-9]{38}$",
    {
        Hub => NetworkId::Mainnet,
        Testnet => NetworkId::Testnet,
    }
}

define_evm_network! {
    cronos,
    "cronos",
    {
        Mainnet => (25, NetworkId::Mainnet),
        Testnet => (338, NetworkId::Testnet),
    }
}

define_network! {
    dogecoin,
    "dogecoin",
    FAMILY_ID_DOGECOIN,
    8,
    "^D[1-9A-HJ-NP-Za-km-z]{25,34}$",
    {
        Mainnet => NetworkId::Mainnet,
        Testnet => NetworkId::Testnet,
    }
}

define_evm_network! {
    ethereum,
    "ethereum",
    {
        Mainnet => (1, NetworkId::Mainnet),
        Sepolia => (11155111, "sepolia".into()),
        Holesky => (17000, "holesky".into()),
    }
}

define_evm_network! {
    fantom,
    "fantom",
    {
        Mainnet => (250, NetworkId::Mainnet),
        Testnet => (4002, NetworkId::Testnet),
    }
}

define_evm_network! {
    gnosis,
    "gnosis",
    {
        Mainnet => (100, NetworkId::Mainnet),
        Chiado => (10200, "chiado".into()),
    }
}

define_network! {
    litecoin,
    "litecoin",
    FAMILY_ID_LITECOIN,
    8,
    "^(L|M|ltc1|ltc1p)[a-zA-HJ-NP-Z0-9]{26,60}$",
    {
        Mainnet => NetworkId::Mainnet,
        Testnet => NetworkId::Testnet,
    }
}

define_evm_network! {
    moonbeam,
    "moonbeam",
    {
        Mainnet => (1284, NetworkId::Mainnet),
        Moonbase => (1287, "moonbase".into()),
    }
}

define_network! {
    near,
    "near",
    FAMILY_ID_NEAR,
    24,
    "^(([a-z0-9]+[\\-_])?[a-z0-9]+\\.)?([a-z0-9]+[\\-_])?[a-z0-9]+$",
    {
        Mainnet => NetworkId::Mainnet,
        Testnet => NetworkId::Testnet,
    }
}

define_evm_network! {
    optimism,
    "optimism",
    {
        Mainnet => (10, NetworkId::Mainnet),
        Sepolia => (11155420, "sepolia".into()),
    }
}

define_network! {
    osmosis,
    "osmosis",
    FAMILY_ID_OSMOSIS,
    6,
    "^osmo1[a-zA-HJ-NP-Z0-9]{38}$",
    {
        Mainnet => NetworkId::Mainnet,
        Testnet => NetworkId::Testnet,
    }
}

define_evm_network! {
    polygon,
    "polygon",
    {
        Mainnet => (137, NetworkId::Mainnet),
        Amoy => (80002, "amoy".into()),
        Mumbai => (80001, "mumbai".into()),
    }
}

define_network! {
    solana,
    "solana",
    FAMILY_ID_SOLANA,
    9,
    "^[1-9A-HJ-NP-Za-km-z]{32,44}$",
    {
        Mainnet => NetworkId::Mainnet,
        Testnet => NetworkId::Testnet,
        Devnet => "devnet".into(),
    }
}

define_network! {
    tron,
    "tron",
    FAMILY_ID_TRON,
    6,
    "^T[a-zA-HJ-NP-Z0-9]{33}$",
    {
        Mainnet => NetworkId::Mainnet,
        Shasta => "shasta".into(),
        Nile => "nile".into(),
    }
}

macro_rules! canonical_network_lookup {
    ($($module:ident => $blockchain_id:literal),+ $(,)?) => {
        pub(crate) fn canonical_network_belongs(
            blockchain_id: &str,
            network: &Network,
        ) -> Option<bool> {
            match blockchain_id {
                $($blockchain_id => Some($module::supports_network(network)),)+
                _ => None,
            }
        }
    };
}

canonical_network_lookup! {
    arbitrum => "arbitrum",
    avalanche => "avalanche",
    base => "base",
    bitcoin => "bitcoin",
    bitcoin_cash => "bitcoin-cash",
    bsc => "bsc",
    celo => "celo",
    cosmos => "cosmos",
    cronos => "cronos",
    dogecoin => "dogecoin",
    ethereum => "ethereum",
    fantom => "fantom",
    gnosis => "gnosis",
    litecoin => "litecoin",
    moonbeam => "moonbeam",
    near => "near",
    optimism => "optimism",
    osmosis => "osmosis",
    polygon => "polygon",
    solana => "solana",
    tron => "tron",
}

fn blockchain(
    blockchain_id: &str,
    family_id: &str,
    native_decimals: u8,
    address_format: &str,
    networks: Vec<Network>,
) -> StockTrekResult<Blockchain> {
    Blockchain::new_with_networks(
        BlockchainId::new(blockchain_id),
        FamilyId::new(family_id),
        native_decimals,
        address_format,
        networks,
    )
}

fn evm_blockchain(blockchain_name: &str, networks: Vec<Network>) -> StockTrekResult<Blockchain> {
    Blockchain::new_with_networks(
        BlockchainId::new(blockchain_name),
        FamilyId::new(FAMILY_ID_EVM),
        18,
        "^0x[a-fA-F0-9]{40}$",
        networks,
    )
}

fn evm_asset_on_chain(
    blockchain_name: &str,
    network: impl Into<NetworkId> + Into<u64> + Copy,
    asset_kind: AssetKind,
    networks: Vec<Network>,
) -> StockTrekResult<AssetOnChain> {
    let blockchain = evm_blockchain(blockchain_name, networks)?;
    let network = Network::for_blockchain(
        BlockchainId::new(blockchain_name),
        network.into(),
        Some(ChainId::new(network.into())),
    );
    AssetOnChain::new(blockchain, network, asset_kind)
}
