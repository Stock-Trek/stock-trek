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

/// Builder for constructing a [`SignalContext`].
#[derive(Debug, Clone, Default)]
pub struct SignalContextBuilder {
    cex_market_data: HashMap<CexId, CexMarketDataByBaseContext>,
}

impl SignalContextBuilder {
    /// Creates an empty signal context builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a market for a base/quote pair on a CEX.
    pub fn cex_market(
        &mut self,
        cex_id: CexId,
        base: AssetId,
        quote: AssetId,
        market: Market,
    ) -> &mut Self {
        self.cex_market_data
            .entry(cex_id)
            .or_default()
            .markets_by_base
            .entry(base)
            .or_default()
            .markets_by_quote
            .insert(quote, market);
        self
    }

    /// Builds the [`SignalContext`] from the collected market data.
    pub fn build(self) -> SignalContext {
        SignalContext::new(self.cex_market_data)
    }
}

/// Market data grouped by base asset for a single CEX.
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
pub struct CexMarketDataByQuoteContext {
    markets_by_quote: HashMap<AssetId, Market>,
}

impl CexMarketDataByQuoteContext {
    /// Creates quote-asset market data from markets keyed by quote asset.
    pub fn new(markets_by_quote: HashMap<AssetId, Market>) -> Self {
        Self { markets_by_quote }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::market::MarketBuilder;

    fn market() -> Market {
        MarketBuilder::new(0.000_001, 0.01, 1.0).build()
    }

    #[test]
    fn empty_builder_builds_an_empty_context() {
        let context = SignalContextBuilder::new().build();

        assert!(
            context
                .cex_markets_for(AssetId::Bitcoin, AssetId::TetherUSD)
                .next()
                .is_none()
        );
    }

    #[test]
    fn adds_a_market_for_a_cex_base_and_quote() {
        let mut builder = SignalContextBuilder::new();
        builder.cex_market(
            CexId::Binance,
            AssetId::Bitcoin,
            AssetId::TetherUSD,
            market(),
        );
        let context = builder.build();

        assert!(
            context
                .cex_market_for(&CexId::Binance, &AssetId::Bitcoin, &AssetId::TetherUSD)
                .is_some()
        );
        assert!(
            context
                .cex_market_for(&CexId::Binance, &AssetId::Ethereum, &AssetId::TetherUSD)
                .is_none()
        );
    }

    #[test]
    fn adds_markets_for_multiple_pairs() {
        let mut builder = SignalContextBuilder::new();
        builder
            .cex_market(
                CexId::Binance,
                AssetId::Bitcoin,
                AssetId::TetherUSD,
                market(),
            )
            .cex_market(
                CexId::Binance,
                AssetId::Ethereum,
                AssetId::TetherUSD,
                market(),
            );
        let context = builder.build();

        assert_eq!(
            context
                .cex_markets_for(AssetId::Bitcoin, AssetId::TetherUSD)
                .count(),
            1
        );
        assert_eq!(
            context
                .cex_markets_for(AssetId::Ethereum, AssetId::TetherUSD)
                .count(),
            1
        );
    }
}
