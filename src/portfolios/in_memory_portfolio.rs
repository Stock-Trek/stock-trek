use crate::portfolios::portfolio::{Portfolio, PortfolioTrait};
use digdigdig3::{Asset, ExchangeId};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct InMemoryPortfolio {
    exchange_assets: HashMap<ExchangeId, Assets>,
}
impl InMemoryPortfolio {
    pub fn new(exchange_assets: HashMap<ExchangeId, Assets>) -> Self {
        Self { exchange_assets }
    }
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl From<InMemoryPortfolio> for Portfolio {
    fn from(value: InMemoryPortfolio) -> Self {
        Box::new(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Assets {
    tokens: HashMap<Asset, f64>,
}
impl Assets {
    pub fn new(tokens: HashMap<Asset, f64>) -> Self {
        Self { tokens }
    }
}

impl PortfolioTrait for InMemoryPortfolio {
    fn has_account_in_exchange(&self, exchange: &ExchangeId) -> bool {
        self.exchange_assets.contains_key(exchange)
    }
    fn owns_asset(&self, asset: &Asset) -> bool {
        self.exchange_assets
            .values()
            .any(|assets| assets.tokens.contains_key(asset))
    }
    fn owns_asset_in_exchange(&self, asset: &Asset, exchange: &ExchangeId) -> bool {
        self.exchange_assets
            .get(exchange)
            .map(|assets| assets.tokens.contains_key(asset))
            .unwrap_or(false)
    }
    fn asset_total(&self, asset: &Asset) -> f64 {
        self.exchange_assets
            .values()
            .map(|assets| assets.tokens.get(asset).unwrap_or(&0.0))
            .sum()
    }
    fn asset_in_exchange(&self, asset: &Asset, exchange: &ExchangeId) -> f64 {
        self.exchange_assets
            .get(exchange)
            .and_then(|assets| assets.tokens.get(asset))
            .copied()
            .unwrap_or(0.0)
    }
}

#[derive(Clone, Default)]
pub struct Builder {
    exchange_assets: HashMap<ExchangeId, Assets>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            exchange_assets: HashMap::new(),
        }
    }
    pub fn tokens(&mut self, exchange_id: ExchangeId, asset: Asset, quantity: f64) -> &mut Self {
        self.exchange_assets
            .entry(exchange_id)
            .or_insert_with(|| Assets::new(HashMap::new()))
            .tokens
            .entry(asset)
            .and_modify(|prev| *prev += quantity)
            .or_insert(quantity);
        self
    }
    pub fn build(&self) -> InMemoryPortfolio {
        InMemoryPortfolio::new(self.exchange_assets.clone())
    }
}
