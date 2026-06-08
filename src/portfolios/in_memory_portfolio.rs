use crate::{
    cex::{asset_id::AssetId, cex_id::CexId},
    portfolios::portfolio::{Portfolio, PortfolioTrait},
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct InMemoryPortfolio {
    cex_assets: HashMap<CexId, Assets>,
}
impl InMemoryPortfolio {
    pub fn new(cex_assets: HashMap<CexId, Assets>) -> Self {
        Self { cex_assets }
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
    fn has_account_in_cex(&self, cex_id: &CexId) -> bool {
        self.cex_assets.contains_key(cex_id)
    }
    fn owns_asset(&self, asset_id: &AssetId) -> bool {
        self.cex_assets
            .values()
            .any(|assets| assets.asset_counts.contains_key(asset_id))
    }
    fn owns_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> bool {
        self.cex_assets
            .get(cex_id)
            .map(|assets| assets.asset_counts.contains_key(asset_id))
            .unwrap_or(false)
    }
    fn asset_total(&self, asset_id: &AssetId) -> f64 {
        self.cex_assets
            .values()
            .map(|assets| assets.asset_counts.get(asset_id).unwrap_or(&0.0))
            .sum()
    }
    fn asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64 {
        self.cex_assets
            .get(cex_id)
            .and_then(|assets| assets.asset_counts.get(asset_id))
            .copied()
            .unwrap_or(0.0)
    }
    // TODO
    // fn order_by_order_id(
    //     &self,
    //     cex_id: &CexId,
    //     order_id: &OrderId,
    // ) -> Option<OrderResponse> {
    //     self.cex_orders
    //         .get(cex_id)
    //         .and_then(|v| v.iter().find(|o| &o.id == order_id))
    //         .cloned()
    // }
    // fn order_by_client_order_id(
    //     &self,
    //     cex_id: &CexId,
    //     client_order_id: &ClientOrderId,
    // ) -> Option<OrderResponse> {
    //     self.cex_orders
    //         .get(cex_id)
    //         .and_then(|v| v.iter().find(|o| &o.client_order_id == client_order_id))
    //         .cloned()
    // }
}

#[derive(Clone, Default)]
pub struct Builder {
    cex_assets: HashMap<CexId, Assets>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            cex_assets: HashMap::new(),
        }
    }
    pub fn assets(&mut self, cex_id: CexId, asset_id: AssetId, quantity: f64) -> &mut Self {
        self.cex_assets
            .entry(cex_id)
            .or_insert_with(|| Assets::new(HashMap::new()))
            .asset_counts
            .entry(asset_id)
            .and_modify(|prev| *prev += quantity)
            .or_insert(quantity);
        self
    }
    pub fn build(&self) -> InMemoryPortfolio {
        InMemoryPortfolio::new(self.cex_assets.clone())
    }
}
