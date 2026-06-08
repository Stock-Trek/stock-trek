use crate::cex::{asset_id::AssetId, exchange_id::ExchangeId};

pub type Portfolio = Box<dyn PortfolioTrait>;

pub trait PortfolioTrait {
    fn has_account_in_exchange(&self, exchange_id: &ExchangeId) -> bool;
    fn owns_asset(&self, asset_id: &AssetId) -> bool;
    fn owns_asset_in_exchange(&self, asset_id: &AssetId, exchange_id: &ExchangeId) -> bool;
    fn asset_total(&self, asset_id: &AssetId) -> f64;
    fn asset_in_exchange(&self, asset_id: &AssetId, exchange_id: &ExchangeId) -> f64;
    // TODO
    // fn orders_in_exchange(&self, exchange_id: &ExchangeId) -> f64;
    // fn order_by_order_id(
    //     &self,
    //     exchange_id: &ExchangeId,
    //     order_id: &OrderId,
    // ) -> Option<OrderResponse>;
    // fn order_by_client_order_id(
    //     &self,
    //     exchange_id: &ExchangeId,
    //     client_order_id: &ClientOrderId,
    // ) -> Option<OrderResponse>;
}
