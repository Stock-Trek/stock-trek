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

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 42161)
    }
    pub fn sepolia() -> Network {
        super::evm_network(blockchain(), "sepolia", 421614)
    }
    pub fn nova() -> Network {
        super::evm_network(blockchain(), "nova", 42170)
    }
}

pub mod avalanche {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "avalanche";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 43114)
    }
    pub fn fuji() -> Network {
        super::evm_network(blockchain(), "fuji", 43113)
    }
}

pub mod base {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "base";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 8453)
    }
    pub fn sepolia() -> Network {
        super::evm_network(blockchain(), "sepolia", 84532)
    }
}

pub mod bitcoin {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "bitcoin";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_BITCOIN,
            8,
            "^(1|3|bc1)[a-zA-HJ-NP-Z0-9]{25,59}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet(blockchain())
    }
    pub fn testnet() -> Network {
        super::testnet(blockchain())
    }
    pub fn signet() -> Network {
        super::network(blockchain(), "signet")
    }
    pub fn regtest() -> Network {
        super::network(blockchain(), "regtest")
    }
}

pub mod bitcoin_cash {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "bitcoin-cash";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_BITCOIN_CASH,
            8,
            "^(q|p|bitcoincash:)[a-zA-HJ-NP-Z0-9]{41,42}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet(blockchain())
    }
    pub fn testnet() -> Network {
        super::testnet(blockchain())
    }
}

pub mod bsc {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "bsc";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 56)
    }
    pub fn testnet() -> Network {
        super::evm_testnet(blockchain(), 97)
    }
}

pub mod celo {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "celo";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 42220)
    }
    pub fn alfajores() -> Network {
        super::evm_network(blockchain(), "alfajores", 44787)
    }
}

pub mod cosmos {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "cosmos";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_COSMOS,
            6,
            "^cosmos1[a-zA-HJ-NP-Z0-9]{38}$",
        )
    }
    pub fn hub() -> Network {
        super::network(blockchain(), "hub")
    }
    pub fn testnet() -> Network {
        super::testnet(blockchain())
    }
}

pub mod cronos {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "cronos";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 25)
    }
    pub fn testnet() -> Network {
        super::evm_testnet(blockchain(), 338)
    }
}

pub mod dogecoin {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "dogecoin";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_DOGECOIN,
            8,
            "^D[a-km-zA-HJ-NP-Z1-9]{25,34}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet(blockchain())
    }
    pub fn testnet() -> Network {
        super::testnet(blockchain())
    }
}

pub mod ethereum {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "ethereum";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 1)
    }
    pub fn sepolia() -> Network {
        super::evm_network(blockchain(), "sepolia", 11155111)
    }
    pub fn holesky() -> Network {
        super::evm_network(blockchain(), "holesky", 17000)
    }
}

pub mod fantom {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "fantom";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 250)
    }
    pub fn testnet() -> Network {
        super::evm_testnet(blockchain(), 4002)
    }
}

pub mod gnosis {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "gnosis";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 100)
    }
    pub fn chiado() -> Network {
        super::evm_network(blockchain(), "chiado", 10200)
    }
}

pub mod litecoin {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "litecoin";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_LITECOIN,
            8,
            "^(L|M|ltc1)[a-zA-HJ-NP-Z0-9]{26,60}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet(blockchain())
    }
    pub fn testnet() -> Network {
        super::testnet(blockchain())
    }
}

pub mod moonbeam {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "moonbeam";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 1284)
    }
    pub fn moonbase() -> Network {
        super::evm_network(blockchain(), "moonbase", 1287)
    }
}

pub mod near {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "near";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_NEAR,
            24,
            "^(([a-z0-9]+[\\-_])?[a-z0-9]+\\.)?([a-z0-9]+[\\-_])?[a-z0-9]+$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet(blockchain())
    }
    pub fn testnet() -> Network {
        super::testnet(blockchain())
    }
}

pub mod optimism {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "optimism";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 10)
    }
    pub fn sepolia() -> Network {
        super::evm_network(blockchain(), "sepolia", 11155420)
    }
}

pub mod osmosis {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "osmosis";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_OSMOSIS,
            6,
            "^osmo1[a-zA-HJ-NP-Z0-9]{38}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet(blockchain())
    }
    pub fn testnet() -> Network {
        super::testnet(blockchain())
    }
}

pub mod polygon {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "polygon";

    pub fn blockchain() -> Blockchain {
        super::evm_blockchain(BLOCKCHAIN_ID)
    }
    pub fn mainnet() -> Network {
        super::evm_mainnet(blockchain(), 137)
    }
    pub fn amoy() -> Network {
        super::evm_network(blockchain(), "amoy", 80002)
    }
    pub fn mumbai() -> Network {
        super::evm_network(blockchain(), "mumbai", 80001)
    }
}

pub mod solana {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "solana";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_SOLANA,
            9,
            "^[1-9A-HJ-NP-Za-km-z]{32,44}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet(blockchain())
    }
    pub fn testnet() -> Network {
        super::testnet(blockchain())
    }
    pub fn devnet() -> Network {
        super::network(blockchain(), "devnet")
    }
}

pub mod tron {
    use crate::dex::{blockchain::Blockchain, network::Network};

    pub const BLOCKCHAIN_ID: &str = "tron";

    pub fn blockchain() -> Blockchain {
        super::blockchain(
            BLOCKCHAIN_ID,
            super::FAMILY_ID_TRON,
            6,
            "^T[a-zA-HJ-NP-Z0-9]{33}$",
        )
    }
    pub fn mainnet() -> Network {
        super::mainnet(blockchain())
    }
    pub fn shasta() -> Network {
        super::network(blockchain(), "shasta")
    }
    pub fn nile() -> Network {
        super::network(blockchain(), "nile")
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

fn mainnet(blockchain: Blockchain) -> Network {
    Network::new(blockchain, NetworkId::Mainnet, None)
}

fn testnet(blockchain: Blockchain) -> Network {
    Network::new(blockchain, NetworkId::Testnet, None)
}

fn network(blockchain: Blockchain, network_id: &str) -> Network {
    Network::new(blockchain, NetworkId::Other(network_id.to_string()), None)
}

fn evm_mainnet(blockchain: Blockchain, chain_id: u64) -> Network {
    Network::new(blockchain, NetworkId::Mainnet, Some(ChainId::new(chain_id)))
}

fn evm_testnet(blockchain: Blockchain, chain_id: u64) -> Network {
    Network::new(blockchain, NetworkId::Testnet, Some(ChainId::new(chain_id)))
}

fn evm_network(blockchain: Blockchain, network_id: &str, chain_id: u64) -> Network {
    Network::new(
        blockchain,
        NetworkId::Other(network_id.to_string()),
        Some(ChainId::new(chain_id)),
    )
}
