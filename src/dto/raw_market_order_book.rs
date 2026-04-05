use crate::dto::raw_market_quote::RawMarketQuote;

#[derive(Debug)]
pub struct RawMarketOrderBook {
    pub bids: Vec<RawMarketQuote>,
    pub asks: Vec<RawMarketQuote>,
}
