use crate::dto::raw_decimal::RawDecimal;

#[derive(Debug)]
pub struct RawMarketOhlcv {
    pub open: RawDecimal,
    pub high: RawDecimal,
    pub low: RawDecimal,
    pub close: RawDecimal,
    pub volume: RawDecimal,
    pub quote_volume: RawDecimal,
    pub vwap: RawDecimal,
}
