use crate::dto::raw_market::RawMarket;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawExchange {
    pub markets: HashMap<String, RawMarket>,
}
