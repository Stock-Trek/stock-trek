use crate::market_data::market::Market;
use std::collections::BTreeMap;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

/// The market data available while computing signals, grouped by CEX.
#[derive(Debug, Clone)]
pub struct SignalContext {
    cex_market_data: BTreeMap<CexId, CexMarketDataByBaseContext>,
}

impl SignalContext {
    /// Creates a signal context from market data grouped by CEX.
    pub fn new(
        cex_market_data: impl IntoIterator<Item = (CexId, CexMarketDataByBaseContext)>,
    ) -> Self {
        Self {
            cex_market_data: cex_market_data.into_iter().collect(),
        }
    }
    /// Iterates over the markets for a base/quote pair on every CEX.
    ///
    /// Markets are yielded in ascending CEX order.
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
    cex_market_data: BTreeMap<CexId, CexMarketDataByBaseContext>,
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
    markets_by_base: BTreeMap<AssetId, CexMarketDataByQuoteContext>,
}

impl CexMarketDataByBaseContext {
    /// Creates base-asset market data from markets grouped by quote asset.
    pub fn new(
        markets_by_base: impl IntoIterator<Item = (AssetId, CexMarketDataByQuoteContext)>,
    ) -> Self {
        Self {
            markets_by_base: markets_by_base.into_iter().collect(),
        }
    }
}

/// Market data grouped by quote asset.
#[derive(Debug, Clone, Default)]
pub struct CexMarketDataByQuoteContext {
    markets_by_quote: BTreeMap<AssetId, Market>,
}

impl CexMarketDataByQuoteContext {
    /// Creates quote-asset market data from markets keyed by quote asset.
    pub fn new(markets_by_quote: impl IntoIterator<Item = (AssetId, Market)>) -> Self {
        Self {
            markets_by_quote: markets_by_quote.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::market::MarketBuilder;

    fn market() -> Market {
        MarketBuilder::new(0.000_001, 0.01, 1.0).build().unwrap()
    }

    #[test]
    fn markets_are_indexed_by_cex_base_and_quote() {
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

        assert!(
            context
                .cex_market_for(&CexId::Binance, &AssetId::Bitcoin, &AssetId::TetherUSD)
                .is_some()
        );
        assert!(
            context
                .cex_market_for(&CexId::Binance, &AssetId::Ethereum, &AssetId::TetherUSD)
                .is_some()
        );
        assert!(
            context
                .cex_market_for(&CexId::Binance, &AssetId::Bitcoin, &AssetId::USDCoin)
                .is_none()
        );
        assert_eq!(
            context
                .cex_markets_for(AssetId::Bitcoin, AssetId::TetherUSD)
                .count(),
            1
        );
    }
}
