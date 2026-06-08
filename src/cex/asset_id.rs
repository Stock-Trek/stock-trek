use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId(String);

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for AssetId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for AssetId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for AssetId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AssetId {
    pub fn aave() -> Self {
        Self("AAVE".to_string())
    }
    pub fn arbitrum() -> Self {
        Self("ARB".to_string())
    }
    pub fn avalanche() -> Self {
        Self("AVAX".to_string())
    }
    pub fn bitcoin() -> Self {
        Self("BTC".to_string())
    }
    pub fn bitcoin_cash() -> Self {
        Self("BCH".to_string())
    }
    pub fn bnb() -> Self {
        Self("BNB".to_string())
    }
    pub fn celo() -> Self {
        Self("CELO".to_string())
    }
    pub fn cosmos() -> Self {
        Self("ATOM".to_string())
    }
    pub fn cronos() -> Self {
        Self("CRO".to_string())
    }
    pub fn dai() -> Self {
        Self("DAI".to_string())
    }
    pub fn dogecoin() -> Self {
        Self("DOGE".to_string())
    }
    pub fn ethereum() -> Self {
        Self("ETH".to_string())
    }
    pub fn fantom() -> Self {
        Self("FTM".to_string())
    }
    pub fn gnosis() -> Self {
        Self("GNO".to_string())
    }
    pub fn litecoin() -> Self {
        Self("LTC".to_string())
    }
    pub fn link() -> Self {
        Self("LINK".to_string())
    }
    pub fn moonbeam() -> Self {
        Self("GLMR".to_string())
    }
    pub fn near() -> Self {
        Self("NEAR".to_string())
    }
    pub fn optimism() -> Self {
        Self("OP".to_string())
    }
    pub fn osmosis() -> Self {
        Self("OSMO".to_string())
    }
    pub fn polygon() -> Self {
        Self("POL".to_string())
    }
    pub fn solana() -> Self {
        Self("SOL".to_string())
    }
    pub fn tron() -> Self {
        Self("TRX".to_string())
    }
    pub fn uni() -> Self {
        Self("UNI".to_string())
    }
    pub fn usdc() -> Self {
        Self("USDC".to_string())
    }
    pub fn usdt() -> Self {
        Self("USDT".to_string())
    }
    pub fn wbtc() -> Self {
        Self("WBTC".to_string())
    }
    pub fn weth() -> Self {
        Self("WETH".to_string())
    }
}
