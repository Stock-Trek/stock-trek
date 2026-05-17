use crate::{
    asset_id::AssetId, exchange_id::ExchangeId, order::order_request::OrderRequest,
    portfolios::portfolio::Portfolio, scratch::scratch_pad::ScratchPad,
};
use rust_decimal::RoundingStrategy;

pub struct ResolvedContext {
    pub price_rounding: RoundingStrategy,
    pub quantity_rounding: RoundingStrategy,
    pub rate_rounding: RoundingStrategy,
    pub enqueue_order: fn(exchange_id: &ExchangeId, order_request: &OrderRequest<AssetId, f64>),
    pub portfolio: Portfolio,
    pub scratch_pad: ScratchPad,
}
