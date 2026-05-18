use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId {
    pub chain: String,
    pub identifier: AssetIdentifier,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetIdentifier {
    Native,
    Contract(String),
}

impl AssetId {
    pub fn new(chain: impl AsRef<str>, contract_id: impl AsRef<str>) -> Self {
        Self {
            chain: chain.as_ref().to_string(),
            identifier: AssetIdentifier::Contract(contract_id.as_ref().to_string()),
        }
    }
    pub fn native(chain: impl AsRef<str>) -> Self {
        Self {
            chain: chain.as_ref().to_string(),
            identifier: AssetIdentifier::Native,
        }
    }
    // Bitcoin
    pub fn bitcoin_native() -> Self {
        Self::native("bitcoin")
    }

    // Ethereum
    pub fn ethereum_native() -> Self {
        Self::native("ethereum")
    }
    pub fn ethereum_usdc() -> Self {
        Self::new("ethereum", "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")
    }
    pub fn ethereum_usdt() -> Self {
        Self::new("ethereum", "0xdac17f958d2ee523a2206206994597c13d831ec7")
    }
    pub fn ethereum_wbtc() -> Self {
        Self::new("ethereum", "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599")
    }
    pub fn ethereum_steth() -> Self {
        Self::new("ethereum", "0xae7ab96520de3a18e5e111b5eaab095312d7fe84")
    }
    pub fn ethereum_reth() -> Self {
        Self::new("ethereum", "0xae78736cd615f374d3085123a210448e74fc6393")
    }

    // Solana
    pub fn solana_native() -> Self {
        Self::native("solana")
    }
    pub fn solana_usdc() -> Self {
        Self::new("solana", "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
    }
    pub fn solana_usdt() -> Self {
        Self::new("solana", "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB")
    }

    // BSC Chain
    pub fn bsc_native() -> Self {
        Self::native("bsc")
    }
    pub fn bsc_usdc() -> Self {
        Self::new("bsc", "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d")
    }
    pub fn bsc_usdt() -> Self {
        Self::new("bsc", "0x55d398326f99059ff775485246999027b3197955")
    }

    // Polygon
    pub fn polygon_native() -> Self {
        Self::native("polygon")
    }
    pub fn polygon_usdc() -> Self {
        Self::new("polygon", "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359")
    }
    pub fn polygon_usdt() -> Self {
        Self::new("polygon", "0xc2132D05D31c914a87C6611C10748AEb04B58e8F")
    }

    // Arbitrum
    pub fn arbitrum_native() -> Self {
        Self::native("arbitrum")
    }
    pub fn arbitrum_usdc() -> Self {
        Self::new("arbitrum", "0xaf88d065e77c8cC2239327C5EDb3A432268e5831")
    }

    // Optimism
    pub fn optimism_native() -> Self {
        Self::native("optimism")
    }
    pub fn optimism_usdc() -> Self {
        Self::new("optimism", "0x0b2c639c533813f4aa9d7837cbf62653d097ff85")
    }

    // Avalanche
    pub fn avalanche_native() -> Self {
        Self::native("avalanche")
    }
    pub fn avalanche_usdc() -> Self {
        Self::new("avalanche", "0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E")
    }

    // Base
    pub fn base_native() -> Self {
        Self::native("base")
    }
    pub fn base_usdc() -> Self {
        Self::new("base", "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")
    }

    // Tron
    pub fn tron_native() -> Self {
        Self::native("tron")
    }
    pub fn tron_usdt() -> Self {
        Self::new("tron", "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t")
    }

    // Sui
    pub fn sui_native() -> Self {
        Self::native("sui")
    }

    // Aptos
    pub fn aptos_native() -> Self {
        Self::native("aptos")
    }

    // Near
    pub fn near_native() -> Self {
        Self::native("near")
    }

    // Cosmos Hub
    pub fn cosmos_native() -> Self {
        Self::native("cosmos")
    }

    // Polkadot
    pub fn polkadot_native() -> Self {
        Self::native("polkadot")
    }

    // Kusama
    pub fn kusama_native() -> Self {
        Self::native("kusama")
    }
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.identifier {
            AssetIdentifier::Native => {
                write!(f, "{}:native", self.chain)
            }
            AssetIdentifier::Contract(contract) => {
                write!(f, "{}:{}", self.chain, contract)
            }
        }
    }
}
