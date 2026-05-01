use crate::{
    exchanges::{
        exchange::Exchange, in_memory_exchange::InMemoryExchange, order_capability::OrderCapability,
    },
    market_data::market::Market,
};
use digdigdig3::Symbol;
use std::collections::{HashMap, HashSet};

pub struct Exchanges {}

impl Exchanges {
    pub fn in_memory(
        capabilities: HashSet<OrderCapability>,
        markets: HashMap<Symbol, Market>,
    ) -> Exchange {
        InMemoryExchange::new(capabilities, markets)
    }
}
