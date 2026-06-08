use crate::cex::{asset_id::AssetId, cex_id::CexId};

pub type Portfolio = Box<dyn PortfolioTrait>;

pub trait PortfolioTrait {
    fn has_account_in_cex(&self, cex_id: &CexId) -> bool;
    fn owns_asset(&self, asset_id: &AssetId) -> bool;
    fn owns_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> bool;
    fn asset_total(&self, asset_id: &AssetId) -> f64;
    fn asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64;
    // TODO
    // fn orders_in_cex(&self, cex_id: &CexId) -> f64;
    // fn order_by_order_id(
    //     &self,
    //     cex_id: &CexId,
    //     order_id: &OrderId,
    // ) -> Option<OrderResponse>;
    // fn order_by_client_order_id(
    //     &self,
    //     cex_id: &CexId,
    //     client_order_id: &ClientOrderId,
    // ) -> Option<OrderResponse>;
}
