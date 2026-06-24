use crate::portfolios::portfolio::{Portfolio, PortfolioTrait};
use std::collections::HashMap;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId, tag::Tag};

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
    fn active_orders(&self) -> f64 {
        0.0
    }
    fn active_orders_with_tag(&self, _tag: &Tag) -> f64 {
        0.0
    }
    fn active_orders_in_cex(&self, _cex_id: &CexId) -> f64 {
        0.0
    }
    fn active_orders_in_cex_with_tag(&self, _cex_id: &CexId, _tag: &Tag) -> f64 {
        0.0
    }
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
