use crate::{market_data::market::Market, prelude::TradingPair};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Exchange {
    markets: HashMap<TradingPair, Market>,
}

impl Exchange {
    pub fn new(markets: HashMap<TradingPair, Market>) -> Self {
        Self { markets }
    }
    pub fn markets(&self) -> &HashMap<TradingPair, Market> {
        &self.markets
    }
}
