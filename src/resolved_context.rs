use crate::{
    asset_id::AssetId, error::result::StockTrekResult, exchange_id::ExchangeId,
    order::order_request::OrderRequest, portfolios::portfolio::Portfolio, signal::signals::Signals,
};

pub struct ResolvedContext {
    pub enqueue_order: EnqueueOrderRequestFn,
    pub portfolio: Portfolio,
    pub signals: Signals,
}

pub type EnqueueOrderRequestFn =
    fn(exchange_id: ExchangeId, order_request: OrderRequest<AssetId, f64>) -> StockTrekResult<()>;
