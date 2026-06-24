use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId, tag::Tag};

pub type Portfolio = Box<dyn PortfolioTrait>;

pub trait PortfolioTrait {
    fn has_account_in_cex(&self, cex_id: &CexId) -> bool;
    fn owns_asset(&self, asset_id: &AssetId) -> bool;
    fn owns_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> bool;
    fn asset_total(&self, asset_id: &AssetId) -> f64;
    fn asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64;
    fn active_orders(&self) -> f64;
    fn active_orders_with_tag(&self, order_tag: &Tag) -> f64;
    fn active_orders_in_cex(&self, cex_id: &CexId) -> f64;
    fn active_orders_in_cex_with_tag(&self, cex_id: &CexId, order_tag: &Tag) -> f64;
}
