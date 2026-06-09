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
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "arbitrum";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Sepolia,
        Nova,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Sepolia => write!(f, "sepolia"),
                NetworkName::Nova => write!(f, "nova"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(42161)
    }
    pub fn sepolia() -> Network {
        super::evm_network_named(&NetworkName::Sepolia, 421614)
    }
    pub fn nova() -> Network {
        super::evm_network_named(&NetworkName::Nova, 42170)
    }
}

pub mod avalanche {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "avalanche";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Fuji,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Fuji => write!(f, "fuji"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(43114)
    }
    pub fn fuji() -> Network {
        super::evm_network_named(&NetworkName::Fuji, 43113)
    }
}

pub mod base {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "base";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Sepolia,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Sepolia => write!(f, "sepolia"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(8453)
    }
    pub fn sepolia() -> Network {
        super::evm_network_named(&NetworkName::Sepolia, 84532)
    }
}

pub mod bitcoin {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "bitcoin";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
        Signet,
        Regtest,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
                NetworkName::Signet => write!(f, "signet"),
                NetworkName::Regtest => write!(f, "regtest"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_BITCOIN,
            8,
            "^(1|3|bc1)[a-zA-HJ-NP-Z0-9]{25,59}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet()
    }
    pub fn testnet() -> Network {
        super::testnet()
    }
    pub fn signet() -> Network {
        super::network_named(&NetworkName::Signet)
    }
    pub fn regtest() -> Network {
        super::network_named(&NetworkName::Regtest)
    }
}

pub mod bitcoin_cash {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "bitcoin-cash";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_BITCOIN_CASH,
            8,
            "^(q|p|bitcoincash:)[a-zA-HJ-NP-Z0-9]{41,42}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet()
    }
    pub fn testnet() -> Network {
        super::testnet()
    }
}

pub mod bsc {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "bsc";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(56)
    }
    pub fn testnet() -> Network {
        super::evm_testnet(97)
    }
}

pub mod celo {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "celo";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Alfajores,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Alfajores => write!(f, "alfajores"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(42220)
    }
    pub fn alfajores() -> Network {
        super::evm_network_named(&NetworkName::Alfajores, 44787)
    }
}

pub mod cosmos {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "cosmos";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
        Hub,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
                NetworkName::Hub => write!(f, "hub"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_COSMOS,
            6,
            "^cosmos1[a-zA-HJ-NP-Z0-9]{38}$",
        )
    }
    pub fn hub() -> Network {
        super::network_named(&NetworkName::Hub)
    }
    pub fn testnet() -> Network {
        super::testnet()
    }
}

pub mod cronos {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "cronos";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(25)
    }
    pub fn testnet() -> Network {
        super::evm_testnet(338)
    }
}

pub mod dogecoin {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "dogecoin";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_DOGECOIN,
            8,
            "^D[a-km-zA-HJ-NP-Z1-9]{25,34}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet()
    }
    pub fn testnet() -> Network {
        super::testnet()
    }
}

pub mod ethereum {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "ethereum";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Sepolia,
        Holesky,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Sepolia => write!(f, "sepolia"),
                NetworkName::Holesky => write!(f, "holesky"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(1)
    }
    pub fn sepolia() -> Network {
        super::evm_network_named(&NetworkName::Sepolia, 11155111)
    }
    pub fn holesky() -> Network {
        super::evm_network_named(&NetworkName::Holesky, 17000)
    }
}

pub mod fantom {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "fantom";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(250)
    }
    pub fn testnet() -> Network {
        super::evm_testnet(4002)
    }
}

pub mod gnosis {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "gnosis";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Chiado,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Chiado => write!(f, "chiado"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(100)
    }
    pub fn chiado() -> Network {
        super::evm_network_named(&NetworkName::Chiado, 10200)
    }
}

pub mod litecoin {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "litecoin";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_LITECOIN,
            8,
            "^(L|M|ltc1)[a-zA-HJ-NP-Z0-9]{26,60}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet()
    }
    pub fn testnet() -> Network {
        super::testnet()
    }
}

pub mod moonbeam {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "moonbeam";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Moonbase,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Moonbase => write!(f, "moonbase"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(1284)
    }
    pub fn moonbase() -> Network {
        super::evm_network_named(&NetworkName::Moonbase, 1287)
    }
}

pub mod near {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "near";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_NEAR,
            24,
            "^(([a-z0-9]+[\\-_])?[a-z0-9]+\\.)?([a-z0-9]+[\\-_])?[a-z0-9]+$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet()
    }
    pub fn testnet() -> Network {
        super::testnet()
    }
}

pub mod optimism {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "optimism";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Sepolia,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Sepolia => write!(f, "sepolia"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(10)
    }
    pub fn sepolia() -> Network {
        super::evm_network_named(&NetworkName::Sepolia, 11155420)
    }
}

pub mod osmosis {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "osmosis";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_OSMOSIS,
            6,
            "^osmo1[a-zA-HJ-NP-Z0-9]{38}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet()
    }
    pub fn testnet() -> Network {
        super::testnet()
    }
}

pub mod polygon {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "polygon";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Amoy,
        Mumbai,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Amoy => write!(f, "amoy"),
                NetworkName::Mumbai => write!(f, "mumbai"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(137)
    }
    pub fn amoy() -> Network {
        super::evm_network_named(&NetworkName::Amoy, 80002)
    }
    pub fn mumbai() -> Network {
        super::evm_network_named(&NetworkName::Mumbai, 80001)
    }
}


pub mod solana {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "solana";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Testnet,
        Devnet,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Testnet => write!(f, "testnet"),
                NetworkName::Devnet => write!(f, "devnet"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_SOLANA,
            9,
            "^[1-9A-HJ-NP-Za-km-z]{32,44}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet()
    }
    pub fn testnet() -> Network {
        super::testnet()
    }
    pub fn devnet() -> Network {
        super::network_named(&NetworkName::Devnet)
    }
}

pub mod tron {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "tron";

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum NetworkName {
        Mainnet,
        Shasta,
        Nile,
    }

    impl std::fmt::Display for NetworkName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                NetworkName::Mainnet => write!(f, "mainnet"),
                NetworkName::Shasta => write!(f, "shasta"),
                NetworkName::Nile => write!(f, "nile"),
            }
        }
    }

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_TRON,
            6,
            "^T[a-zA-HJ-NP-Z0-9]{33}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet()
    }
    pub fn shasta() -> Network {
        super::network_named(&NetworkName::Shasta)
    }
    pub fn nile() -> Network {
        super::network_named(&NetworkName::Nile)
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

fn mainnet() -> Network {
    Network::new(NetworkId::Mainnet, None)
}

fn testnet() -> Network {
    Network::new(NetworkId::Testnet, None)
}

fn network(network_id: &str) -> Network {
    Network::new(NetworkId::Other(network_id.to_string()), None)
}

fn network_named(network_name: &impl std::fmt::Display) -> Network {
    Network::new(NetworkId::Other(network_name.to_string()), None)
}

fn evm_mainnet(chain_id: u64) -> Network {
    Network::new(NetworkId::Mainnet, Some(ChainId::new(chain_id)))
}

fn evm_testnet(chain_id: u64) -> Network {
    Network::new(NetworkId::Testnet, Some(ChainId::new(chain_id)))
}

fn evm_network(network_id: &str, chain_id: u64) -> Network {
    Network::new(
        NetworkId::Other(network_id.to_string()),
        Some(ChainId::new(chain_id)),
    )
}

fn evm_network_named(network_name: &impl std::fmt::Display, chain_id: u64) -> Network {
    Network::new(
        NetworkId::Other(network_name.to_string()),
        Some(ChainId::new(chain_id)),
    )
}
