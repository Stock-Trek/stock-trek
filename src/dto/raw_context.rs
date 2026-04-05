use crate::dto::raw_exchange::RawExchange;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawContext {
    pub exchanges: HashMap<String, RawExchange>,
}
