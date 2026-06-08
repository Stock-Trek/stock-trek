use crate::cex::asset_id::AssetId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradingPair {
    base: AssetId,
    quote: AssetId,
}
impl TradingPair {
    pub fn new(base: AssetId, quote: AssetId) -> Self {
        if base == quote {
            panic!("Base {} cannot be the same as Quote {}", base, quote);
        }
        Self { base, quote }
    }
}
