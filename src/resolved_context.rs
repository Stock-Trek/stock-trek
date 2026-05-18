use crate::{
    asset_id::AssetId, error::result::StockTrekResult, exchange_id::ExchangeId,
    order::order_request::OrderRequest, portfolios::portfolio::Portfolio,
    scratch::scratch_pad::ScratchPad,
};

pub struct ResolvedContext {
    pub enqueue_order: EnqueueOrderRequestFn,
    pub portfolio: Portfolio,
    pub scratch_pad: ScratchPad,
}

pub type EnqueueOrderRequestFn =
    fn(exchange_id: &ExchangeId, order_request: &OrderRequest<AssetId, f64>) -> StockTrekResult<()>;
