use crate::market_data::{extract::vec_quote_to_f64, market_quote::MarketQuote};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct MarketOrderBook {
    exact_bids: Vec<MarketQuote>,
    exact_asks: Vec<MarketQuote>,
    bids: OnceLock<Vec<(f64, f64)>>,
    asks: OnceLock<Vec<(f64, f64)>>,
}

impl MarketOrderBook {
    pub fn new(exact_bids: Vec<MarketQuote>, exact_asks: Vec<MarketQuote>) -> Self {
        Self {
            exact_bids,
            exact_asks,
            bids: OnceLock::new(),
            asks: OnceLock::new(),
        }
    }
    pub fn bids(&self) -> &Vec<(f64, f64)> {
        self.bids.get_or_init(|| vec_quote_to_f64(&self.exact_bids))
    }
    pub fn asks(&self) -> &Vec<(f64, f64)> {
        self.asks.get_or_init(|| vec_quote_to_f64(&self.exact_asks))
    }
}
