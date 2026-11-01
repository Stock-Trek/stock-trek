use crate::market_data::market::Market;
use std::collections::HashMap;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

/// The market data available while computing signals, grouped by CEX.
#[derive(Debug, Clone)]
pub struct SignalContext {
    cex_market_data: HashMap<CexId, CexMarketDataByBaseContext>,
}

impl SignalContext {
    /// Creates a signal context from market data grouped by CEX.
    pub fn new(cex_market_data: HashMap<CexId, CexMarketDataByBaseContext>) -> Self {
        Self { cex_market_data }
    }
    /// Iterates over the markets for a base/quote pair on every CEX.
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
                    .map(|market| (*cex_id, market))
            })
    }
    /// Returns the market for a base/quote pair on a specific CEX.
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

/// Market data grouped by base asset for a single CEX.
#[derive(Debug, Clone)]
pub struct CexMarketDataByBaseContext {
    markets_by_base: HashMap<AssetId, CexMarketDataByQuoteContext>,
}

impl CexMarketDataByBaseContext {
    /// Creates base-asset market data from markets grouped by quote asset.
    pub fn new(markets_by_base: HashMap<AssetId, CexMarketDataByQuoteContext>) -> Self {
        Self { markets_by_base }
    }
}

/// Market data grouped by quote asset.
#[derive(Debug, Clone)]
pub struct CexMarketDataByQuoteContext {
    markets_by_quote: HashMap<AssetId, Market>,
}

impl CexMarketDataByQuoteContext {
    /// Creates quote-asset market data from markets keyed by quote asset.
    pub fn new(markets_by_quote: HashMap<AssetId, Market>) -> Self {
        Self { markets_by_quote }
    }
}
