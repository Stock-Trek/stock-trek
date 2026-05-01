use crate::market_data::{
    market_aligned_window::MarketAlignedWindow, market_order_book::MarketOrderBook,
    market_rolling_window::MarketRollingWindow, market_ticks::MarketTicks,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
    pub base_increment: f64,
    pub quote_increment: f64,
    pub minimum_notional: f64,
    pub ticks: MarketTicks,
    pub rolling: MarketRollingWindow,
    pub aligned: MarketAlignedWindow,
    pub order_book: MarketOrderBook,
}
