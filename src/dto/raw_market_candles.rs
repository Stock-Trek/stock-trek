use crate::dto::raw_market_candle::RawMarketCandle;

#[derive(Debug)]
pub struct RawMarketCandles {
    pub candles: Vec<RawMarketCandle>,
}
