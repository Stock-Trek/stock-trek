use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId(String);

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AssetId {
    fn new(name: &str) -> Self {
        Self(name.to_string())
    }
    pub fn aave() -> Self {
        Self::new("Aave")
    }
    pub fn arbitrum() -> Self {
        Self::new("Arbitrum")
    }
    pub fn atom() -> Self {
        Self::new("Atom")
    }
    pub fn avalanche() -> Self {
        Self::new("Avalanche")
    }
    pub fn bitcoin() -> Self {
        Self::new("Bitcoin")
    }
    pub fn bitcoin_cash() -> Self {
        Self::new("Bitcoin Cash")
    }
    pub fn bnb() -> Self {
        Self::new("BNB")
    }
    pub fn celo() -> Self {
        Self::new("Celo")
    }
    pub fn chainlink() -> Self {
        Self::new("Chainlink")
    }
    pub fn cronos() -> Self {
        Self::new("Cronos")
    }
    pub fn dai() -> Self {
        Self::new("Dai")
    }
    pub fn dogecoin() -> Self {
        Self::new("Dogecoin")
    }
    pub fn ethereum() -> Self {
        Self::new("Ethereum")
    }
    pub fn fantom() -> Self {
        Self::new("Fantom")
    }
    pub fn gnosis() -> Self {
        Self::new("Gnosis")
    }
    pub fn litecoin() -> Self {
        Self::new("Litecoin")
    }
    pub fn moonbeam() -> Self {
        Self::new("Moonbeam")
    }
    pub fn near() -> Self {
        Self::new("NEAR")
    }
    pub fn optimism() -> Self {
        Self::new("Optimism")
    }
    pub fn osmosis() -> Self {
        Self::new("Osmosis")
    }
    pub fn polygon() -> Self {
        Self::new("Polygon")
    }
    pub fn solana() -> Self {
        Self::new("Solana")
    }
    pub fn tether_usd() -> Self {
        Self::new("Tether USD")
    }
    pub fn tron() -> Self {
        Self::new("TRON")
    }
    pub fn uniswap() -> Self {
        Self::new("Uniswap")
    }
    pub fn usd_coin() -> Self {
        Self::new("USD Coin")
    }
    pub fn wrapped_bitcoin() -> Self {
        Self::new("Wrapped Bitcoin")
    }
    pub fn wrapped_ethereum() -> Self {
        Self::new("Wrapped Ethereum")
    }
}
