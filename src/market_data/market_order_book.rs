use crate::{
    dto::raw_market_order_book::RawMarketOrderBook,
    market_data::{extract::vec_quote_to_f64, market_quote::MarketQuote},
};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct MarketOrderBook {
    exact_bids: Vec<MarketQuote>,
    exact_asks: Vec<MarketQuote>,
    bids: OnceLock<Vec<(f64, f64)>>,
    asks: OnceLock<Vec<(f64, f64)>>,
}

impl MarketOrderBook {
    pub fn bids(&self) -> &Vec<(f64, f64)> {
        self.bids.get_or_init(|| vec_quote_to_f64(&self.exact_bids))
    }
    pub fn asks(&self) -> &Vec<(f64, f64)> {
        self.asks.get_or_init(|| vec_quote_to_f64(&self.exact_asks))
    }
}

impl From<RawMarketOrderBook> for MarketOrderBook {
    fn from(value: RawMarketOrderBook) -> Self {
        let RawMarketOrderBook { bids, asks } = value;
        let exact_bids = bids.into_iter().map(|bid| MarketQuote::from(bid)).collect();
        let exact_asks = asks.into_iter().map(|ask| MarketQuote::from(ask)).collect();
        MarketOrderBook {
            exact_bids,
            exact_asks,
            bids: OnceLock::new(),
            asks: OnceLock::new(),
        }
    }
}
