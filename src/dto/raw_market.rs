use crate::dto::raw_decimal::RawDecimal;
use crate::dto::raw_market_candles::RawMarketCandles;
use crate::dto::{
    raw_market_candle::RawMarketCandle, raw_market_order_book::RawMarketOrderBook,
    raw_market_tick::RawMarketTick,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawMarket {
    pub base_increment: RawDecimal,
    pub quote_increment: RawDecimal,
    pub minimum_notional: RawDecimal,
    pub ticks: Vec<RawMarketTick>,
    pub rolling: HashMap<u8, RawMarketCandle>,
    pub aligned: HashMap<u8, RawMarketCandles>,
    pub order_book: RawMarketOrderBook,
}
