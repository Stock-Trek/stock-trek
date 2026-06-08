use crate::{
    cex::{asset_id::AssetId, exchange_id::ExchangeId},
    portfolios::portfolio::{Portfolio, PortfolioTrait},
};

#[derive(Debug, Clone, Default)]
pub struct StubPortfolio;

impl StubPortfolio {
    pub fn new() -> Self {
        Self
    }
}

impl From<StubPortfolio> for Portfolio {
    fn from(value: StubPortfolio) -> Self {
        Box::new(value)
    }
}

impl PortfolioTrait for StubPortfolio {
    fn has_account_in_exchange(&self, _exchange_id: &ExchangeId) -> bool {
        true
    }
    fn owns_asset(&self, _asset_id: &AssetId) -> bool {
        true
    }
    fn owns_asset_in_exchange(&self, _asset_id: &AssetId, _exchange_id: &ExchangeId) -> bool {
        true
    }
    fn asset_total(&self, _asset_id: &AssetId) -> f64 {
        1_000_000.0
    }
    fn asset_in_exchange(&self, _asset_id: &AssetId, _exchange_id: &ExchangeId) -> f64 {
        1_000_000.0
    }
    // TODO
    // fn order_by_order_id(
    //     &self,
    //     _exchange_id: &ExchangeId,
    //     _order_id: &OrderId,
    // ) -> Option<OrderResponse> {
    //     None
    // }
    // fn order_by_client_order_id(
    //     &self,
    //     _exchange_id: &ExchangeId,
    //     _client_order_id: &ClientOrderId,
    // ) -> Option<OrderResponse> {
    //     None
    // }
}
