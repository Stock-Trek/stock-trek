use crate::dex::{
    asset_kind::AssetKind,
    asset_on_chain::AssetOnChain,
    blockchain::{Blockchain, BlockchainId, FamilyId},
    network::{ChainId, Network, NetworkId},
};
use crate::error::result::{StockTrekError, StockTrekResult};

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

        impl TryFrom<NetworkName> for NetworkId {
            type Error = crate::error::result::StockTrekError;

            fn try_from(value: NetworkName) -> Result<Self, Self::Error> {
                match value {
                    $(NetworkName::$variant => Ok($network_id),)+
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

            pub fn asset_on_chain(
                network: NetworkName,
                asset_kind: crate::dex::asset_kind::AssetKind,
            ) -> StockTrekResult<crate::dex::asset_on_chain::AssetOnChain> {
                super::AssetOnChain::evm($blockchain, network, asset_kind)
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

            pub fn asset_on_chain(
                network: NetworkName,
                asset_kind: crate::dex::asset_kind::AssetKind,
            ) -> StockTrekResult<crate::dex::asset_on_chain::AssetOnChain> {
                let blockchain = super::Blockchain::from_ids(
                    $blockchain,
                    super::$family_id,
                    $native_decimals,
                    $address_format,
                )?;
                let network = Network::new(NetworkId::try_from(network)?, None);
                crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
            }
        }
    };
}

define_evm_network! {
    arbitrum,
    "arbitrum",
    {
        Mainnet => (42161, NetworkId::MAINNET),
        Sepolia => (421614, NetworkId::new("sepolia")?),
        Nova => (42170, NetworkId::new("nova")?),
    }
}

define_evm_network! {
    avalanche,
    "avalanche",
    {
        Mainnet => (43114, NetworkId::MAINNET),
        Fuji => (43113, NetworkId::new("fuji")?),
    }
}

define_evm_network! {
    base,
    "base",
    {
        Mainnet => (8453, NetworkId::MAINNET),
        Sepolia => (84532, NetworkId::new("sepolia")?),
    }
}

define_network! {
    bitcoin,
    "bitcoin",
    FAMILY_ID_BITCOIN,
    8,
    r"^([13][1-9A-HJ-NP-Za-km-z]{25,34}|bc1[ac-hj-np-z02-9]{39,59}|bc1p[ac-hj-np-z02-9]{58,59})$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
        Signet => NetworkId::new("signet")?,
        Regtest => NetworkId::new("regtest")?,
    }
}

define_network! {
    bitcoin_cash,
    "bitcoin-cash",
    FAMILY_ID_BITCOIN_CASH,
    8,
    "^(bitcoincash:)?(q|p)[a-zA-HJ-NP-Z0-9]{41,42}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_evm_network! {
    bsc,
    "bsc",
    {
        Mainnet => (56, NetworkId::MAINNET),
        Testnet => (97, NetworkId::TESTNET),
    }
}

define_evm_network! {
    celo,
    "celo",
    {
        Mainnet => (42220, NetworkId::MAINNET),
        Alfajores => (44787, NetworkId::new("alfajores")?),
    }
}

define_network! {
    cosmos,
    "cosmos",
    FAMILY_ID_COSMOS,
    6,
    "^cosmos1[a-zA-HJ-NP-Z0-9]{38}$",
    {
        Hub => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_evm_network! {
    cronos,
    "cronos",
    {
        Mainnet => (25, NetworkId::MAINNET),
        Testnet => (338, NetworkId::TESTNET),
    }
}

define_network! {
    dogecoin,
    "dogecoin",
    FAMILY_ID_DOGECOIN,
    8,
    "^D[1-9A-HJ-NP-Za-km-z]{25,34}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_evm_network! {
    ethereum,
    "ethereum",
    {
        Mainnet => (1, NetworkId::MAINNET),
        Sepolia => (11155111, NetworkId::new("sepolia")?),
        Holesky => (17000, NetworkId::new("holesky")?),
    }
}

define_evm_network! {
    fantom,
    "fantom",
    {
        Mainnet => (250, NetworkId::MAINNET),
        Testnet => (4002, NetworkId::TESTNET),
    }
}

define_evm_network! {
    gnosis,
    "gnosis",
    {
        Mainnet => (100, NetworkId::MAINNET),
        Chiado => (10200, NetworkId::new("chiado")?),
    }
}

define_network! {
    litecoin,
    "litecoin",
    FAMILY_ID_LITECOIN,
    8,
    "^(L|M|ltc1|ltc1p)[a-zA-HJ-NP-Z0-9]{26,60}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_evm_network! {
    moonbeam,
    "moonbeam",
    {
        Mainnet => (1284, NetworkId::MAINNET),
        Moonbase => (1287, NetworkId::new("moonbase")?),
    }
}

define_network! {
    near,
    "near",
    FAMILY_ID_NEAR,
    24,
    "^(([a-z0-9]+[\\-_])?[a-z0-9]+\\.)?([a-z0-9]+[\\-_])?[a-z0-9]+$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_evm_network! {
    optimism,
    "optimism",
    {
        Mainnet => (10, NetworkId::MAINNET),
        Sepolia => (11155420, NetworkId::new("sepolia")?),
    }
}

define_network! {
    osmosis,
    "osmosis",
    FAMILY_ID_OSMOSIS,
    6,
    "^osmo1[a-zA-HJ-NP-Z0-9]{38}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_evm_network! {
    polygon,
    "polygon",
    {
        Mainnet => (137, NetworkId::MAINNET),
        Amoy => (80002, NetworkId::new("amoy")?),
        Mumbai => (80001, NetworkId::new("mumbai")?),
    }
}

define_network! {
    solana,
    "solana",
    FAMILY_ID_SOLANA,
    9,
    "^[1-9A-HJ-NP-Za-km-z]{32,44}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
        Devnet => NetworkId::new("devnet")?,
    }
}

define_network! {
    tron,
    "tron",
    FAMILY_ID_TRON,
    6,
    "^T[a-zA-HJ-NP-Z0-9]{33}$",
    {
        Mainnet => NetworkId::MAINNET,
        Shasta => NetworkId::new("shasta")?,
        Nile => NetworkId::new("nile")?,
    }
}

impl Blockchain {
    fn from_ids(
        blockchain_id: &str,
        family_id: &str,
        native_decimals: u8,
        address_format: &str,
    ) -> StockTrekResult<Self> {
        Self::new(
            BlockchainId::new(blockchain_id)?,
            FamilyId::new(family_id)?,
            native_decimals,
            address_format,
        )
    }

    fn evm(blockchain_name: &str) -> StockTrekResult<Self> {
        Self::from_ids(blockchain_name, FAMILY_ID_EVM, 18, "^0x[a-fA-F0-9]{40}$")
    }
}

impl AssetOnChain {
    fn evm(
        blockchain_name: &str,
        network: impl TryInto<NetworkId, Error = StockTrekError> + Into<u64> + Copy,
        asset_kind: AssetKind,
    ) -> StockTrekResult<Self> {
        let blockchain = Blockchain::evm(blockchain_name)?;
        let network_id = network.try_into()?;
        let network = Network::new(network_id, Some(ChainId::new(network.into())));
        Self::new(blockchain, network, asset_kind)
    }
}
