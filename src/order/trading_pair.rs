use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradingPair {
    pub base: String,
    pub quote: String,
}

impl TradingPair {
    pub fn new(base: impl AsRef<str>, quote: impl AsRef<str>) -> Self {
        Self {
            base: base.as_ref().to_string(),
            quote: quote.as_ref().to_string(),
        }
    }
}
