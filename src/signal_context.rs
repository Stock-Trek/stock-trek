use crate::market_data::market::Market;
use std::collections::HashMap;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

pub struct SignalContext {
    cex_market_data: HashMap<CexId, CexMarketDataByBaseContext>,
}

impl SignalContext {
    pub fn new(cex_market_data: HashMap<CexId, CexMarketDataByBaseContext>) -> Self {
        Self { cex_market_data }
    }
    pub fn cex_markets_for(
        &self,
        base: AssetId,
        quote: AssetId,
    ) -> impl Iterator<Item = (CexId, &Market)> {
        self.cex_market_data
            .iter()
            .filter_map(move |(cex_id, by_base)| {
                by_base
                    .markets_by_base
                    .get(&base)
                    .and_then(|by_quote| by_quote.markets_by_quote.get(&quote))
                    .map(|market| (cex_id.clone(), market))
            })
    }
    pub fn cex_market_for(
        &self,
        cex_id: &CexId,
        base: &AssetId,
        quote: &AssetId,
    ) -> Option<&Market> {
        self.cex_market_data
            .get(cex_id)
            .and_then(|m| m.markets_by_base.get(base))
            .and_then(|m| m.markets_by_quote.get(quote))
    }
}

pub struct CexMarketDataByBaseContext {
    markets_by_base: HashMap<AssetId, CexMarketDataByQuoteContext>,
}

impl CexMarketDataByBaseContext {
    pub fn new(markets_by_base: HashMap<AssetId, CexMarketDataByQuoteContext>) -> Self {
        Self { markets_by_base }
    }
}

pub struct CexMarketDataByQuoteContext {
    markets_by_quote: HashMap<AssetId, Market>,
}

impl CexMarketDataByQuoteContext {
    pub fn new(markets_by_quote: HashMap<AssetId, Market>) -> Self {
        Self { markets_by_quote }
    }
}
