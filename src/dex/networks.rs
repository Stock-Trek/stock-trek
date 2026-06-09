use crate::dex::{
    blockchain::{Blockchain, BlockchainId, FamilyId},
    network::{ChainId, Network, NetworkId},
};

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

pub mod arbitrum {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Sepolia,
        Nova,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 42161,
                NetworkName::Sepolia => 421614,
                NetworkName::Nova => 42170,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Sepolia => "sepolia".into(),
                NetworkName::Nova => "nova".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("arbitrum");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod avalanche {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Fuji,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 43114,
                NetworkName::Fuji => 43113,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Fuji => "fuji".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("avalanche");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod base {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Sepolia,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 8453,
                NetworkName::Sepolia => 84532,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Sepolia => "sepolia".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("base");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod bitcoin {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
        Signet,
        Regtest,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
                NetworkName::Signet => "signet".into(),
                NetworkName::Regtest => "regtest".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::blockchain(
            "bitcoin",
            super::FAMILY_ID_BITCOIN,
            8,
            "^(1|3|bc1)[a-zA-HJ-NP-Z0-9]{25,59}$",
        );
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod bitcoin_cash {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::blockchain(
            "bitcoin-cash",
            super::FAMILY_ID_BITCOIN_CASH,
            8,
            "^(q|p|bitcoincash:)[a-zA-HJ-NP-Z0-9]{41,42}$",
        );
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod bsc {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 56,
                NetworkName::Testnet => 97,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("bsc");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod celo {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Alfajores,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 42220,
                NetworkName::Alfajores => 44787,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Alfajores => "alfajores".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("celo");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod cosmos {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Hub,
        Testnet,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Hub => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::blockchain(
            "cosmos",
            super::FAMILY_ID_COSMOS,
            6,
            "^cosmos1[a-zA-HJ-NP-Z0-9]{38}$",
        );
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod cronos {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 25,
                NetworkName::Testnet => 338,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("cronos");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod dogecoin {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::blockchain(
            "dogecoin",
            super::FAMILY_ID_DOGECOIN,
            8,
            "^D[a-km-zA-HJ-NP-Z1-9]{25,34}$",
        );
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod ethereum {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Sepolia,
        Holesky,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 1,
                NetworkName::Sepolia => 11155111,
                NetworkName::Holesky => 17000,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Sepolia => "sepolia".into(),
                NetworkName::Holesky => "holesky".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("ethereum");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod fantom {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 250,
                NetworkName::Testnet => 4002,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("fantom");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod gnosis {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Chiado,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 100,
                NetworkName::Chiado => 10200,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Chiado => "chiado".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("gnosis");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod litecoin {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::blockchain(
            "litecoin",
            super::FAMILY_ID_LITECOIN,
            8,
            "^(L|M|ltc1)[a-zA-HJ-NP-Z0-9]{26,60}$",
        );
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod moonbeam {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Moonbase,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 1284,
                NetworkName::Moonbase => 1287,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Moonbase => "moonbase".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("moonbeam");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod near {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::blockchain(
            "near",
            super::FAMILY_ID_NEAR,
            24,
            "^(([a-z0-9]+[\\-_])?[a-z0-9]+\\.)?([a-z0-9]+[\\-_])?[a-z0-9]+$",
        );
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod optimism {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Sepolia,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 10,
                NetworkName::Sepolia => 11155420,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Sepolia => "sepolia".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("optimism");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod osmosis {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::blockchain(
            "osmosis",
            super::FAMILY_ID_OSMOSIS,
            6,
            "^osmo1[a-zA-HJ-NP-Z0-9]{38}$",
        );
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod polygon {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Amoy,
        Mumbai,
    }

    impl From<NetworkName> for u64 {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => 137,
                NetworkName::Amoy => 80002,
                NetworkName::Mumbai => 80001,
            }
        }
    }
    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Amoy => "amoy".into(),
                NetworkName::Mumbai => "mumbai".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::evm_blockchain("polygon");
        let network = super::evm_network(network.into(), network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod solana {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
        Devnet,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Testnet => NetworkId::Testnet,
                NetworkName::Devnet => "devnet".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain = super::blockchain(
            "solana",
            super::FAMILY_ID_SOLANA,
            9,
            "^[1-9A-HJ-NP-Za-km-z]{32,44}$",
        );
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

pub mod tron {
    use crate::dex::network::NetworkId;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Shasta,
        Nile,
    }

    impl From<NetworkName> for NetworkId {
        fn from(value: NetworkName) -> Self {
            match value {
                NetworkName::Mainnet => NetworkId::Mainnet,
                NetworkName::Shasta => "shasta".into(),
                NetworkName::Nile => "nile".into(),
            }
        }
    }

    pub fn asset_on_chain(
        network: NetworkName,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> crate::dex::asset_on_chain::AssetOnChain {
        let blockchain =
            super::blockchain("tron", super::FAMILY_ID_TRON, 6, "^T[a-zA-HJ-NP-Z0-9]{33}$");
        let network = super::network(network.into());
        crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
    }
}

fn blockchain(
    blockchain_id: &str,
    family_id: &str,
    native_decimals: u8,
    address_format: &str,
) -> Blockchain {
    Blockchain::new(
        BlockchainId::new(blockchain_id),
        FamilyId::new(family_id),
        native_decimals,
        address_format.to_string(),
    )
}

fn evm_blockchain(blockchain_id: &str) -> Blockchain {
    Blockchain::new(
        BlockchainId::new(blockchain_id),
        FamilyId::new(FAMILY_ID_EVM),
        18,
        "^0x[a-fA-F0-9]{40}$".to_string(),
    )
}

fn network(network_id: NetworkId) -> Network {
    Network::new(network_id, None)
}

fn evm_network(network_id: NetworkId, chain_id: u64) -> Network {
    Network::new(network_id, Some(ChainId::new(chain_id)))
}
