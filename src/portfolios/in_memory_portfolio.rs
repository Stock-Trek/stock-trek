use crate::{
    cex::{asset_id::AssetId, exchange_id::ExchangeId},
    portfolios::portfolio::{Portfolio, PortfolioTrait},
};
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
    asset_counts: HashMap<AssetId, f64>,
}
impl Assets {
    pub fn new(asset_counts: HashMap<AssetId, f64>) -> Self {
        Self { asset_counts }
    }
}

impl PortfolioTrait for InMemoryPortfolio {
    fn has_account_in_exchange(&self, exchange_id: &ExchangeId) -> bool {
        self.exchange_assets.contains_key(exchange_id)
    }
    fn owns_asset(&self, asset_id: &AssetId) -> bool {
        self.exchange_assets
            .values()
            .any(|assets| assets.asset_counts.contains_key(asset_id))
    }
    fn owns_asset_in_exchange(&self, asset_id: &AssetId, exchange_id: &ExchangeId) -> bool {
        self.exchange_assets
            .get(exchange_id)
            .map(|assets| assets.asset_counts.contains_key(asset_id))
            .unwrap_or(false)
    }
    fn asset_total(&self, asset_id: &AssetId) -> f64 {
        self.exchange_assets
            .values()
            .map(|assets| assets.asset_counts.get(asset_id).unwrap_or(&0.0))
            .sum()
    }
    fn asset_in_exchange(&self, asset_id: &AssetId, exchange_id: &ExchangeId) -> f64 {
        self.exchange_assets
            .get(exchange_id)
            .and_then(|assets| assets.asset_counts.get(asset_id))
            .copied()
            .unwrap_or(0.0)
    }
    // TODO
    // fn order_by_order_id(
    //     &self,
    //     exchange: &ExchangeId,
    //     order_id: &OrderId,
    // ) -> Option<OrderResponse> {
    //     self.exchange_orders
    //         .get(exchange)
    //         .and_then(|v| v.iter().find(|o| &o.id == order_id))
    //         .cloned()
    // }
    // fn order_by_client_order_id(
    //     &self,
    //     exchange: &ExchangeId,
    //     client_order_id: &ClientOrderId,
    // ) -> Option<OrderResponse> {
    //     self.exchange_orders
    //         .get(exchange)
    //         .and_then(|v| v.iter().find(|o| &o.client_order_id == client_order_id))
    //         .cloned()
    // }
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
    pub fn assets(
        &mut self,
        exchange_id: ExchangeId,
        asset_id: AssetId,
        quantity: f64,
    ) -> &mut Self {
        self.exchange_assets
            .entry(exchange_id)
            .or_insert_with(|| Assets::new(HashMap::new()))
            .asset_counts
            .entry(asset_id)
            .and_modify(|prev| *prev += quantity)
            .or_insert(quantity);
        self
    }
    pub fn build(&self) -> InMemoryPortfolio {
        InMemoryPortfolio::new(self.exchange_assets.clone())
    }
}
