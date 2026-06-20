use crate::cex::capability::CexCapability;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CexId(String);

impl fmt::Display for CexId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

const BINANCE: &str = "Binance";
const COINBASE: &str = "Coinbase";

impl CexId {
    fn new(name: &str) -> Self {
        Self(name.to_string())
    }
    pub fn binance() -> Self {
        Self::new(BINANCE)
    }
    pub fn coinbase() -> Self {
        Self::new(COINBASE)
    }
    pub fn has_capability(&self, capability: CexCapability) -> bool {
        matches!((self.0.as_str(), capability), (BINANCE, _) | (COINBASE, _))
    }
}
