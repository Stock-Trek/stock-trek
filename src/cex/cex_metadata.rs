use crate::market_data::market::Market;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

/// Static trading metadata for a market on a CEX.
///
/// This contains only the exchange-defined order rules. It intentionally does
/// not contain ticks, candles, or order books, because those are not available
/// when actions are resolved.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MarketMetadata {
    /// The minimum increment for the base asset quantity.
    pub base_increment: f64,
    /// The minimum increment for the quote asset price.
    pub quote_increment: f64,
    /// The minimum notional value for an order.
    pub minimum_notional: f64,
}

impl MarketMetadata {
    /// Creates market metadata from the market's static trading rules.
    pub fn new(base_increment: f64, quote_increment: f64, minimum_notional: f64) -> Self {
        Self {
            base_increment,
            quote_increment,
            minimum_notional,
        }
    }
}

impl From<&Market> for MarketMetadata {
    fn from(market: &Market) -> Self {
        Self {
            base_increment: market.base_increment,
            quote_increment: market.quote_increment,
            minimum_notional: market.minimum_notional,
        }
    }
}

impl From<Market> for MarketMetadata {
    fn from(market: Market) -> Self {
        Self::from(&market)
    }
}

/// Alias for [`MarketMetadata`] when used as CEX metadata.
pub type CexMarketMetadata = MarketMetadata;

/// The static CEX metadata available while resolving actions, grouped by CEX.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CexMetadata {
    market_metadata: HashMap<CexId, HashMap<AssetId, HashMap<AssetId, MarketMetadata>>>,
}

impl CexMetadata {
    /// Creates CEX metadata from market metadata grouped by CEX, base, and quote.
    pub fn new(
        market_metadata: HashMap<CexId, HashMap<AssetId, HashMap<AssetId, MarketMetadata>>>,
    ) -> Self {
        Self { market_metadata }
    }

    /// Returns the metadata for a base/quote pair on a specific CEX.
    pub fn cex_market_for(
        &self,
        cex_id: &CexId,
        base: &AssetId,
        quote: &AssetId,
    ) -> Option<&MarketMetadata> {
        self.market_metadata
            .get(cex_id)
            .and_then(|markets_by_base| markets_by_base.get(base))
            .and_then(|markets_by_quote| markets_by_quote.get(quote))
    }

    /// Returns the metadata for a base/quote pair on a specific CEX.
    pub fn market(
        &self,
        cex_id: &CexId,
        base: &AssetId,
        quote: &AssetId,
    ) -> Option<&MarketMetadata> {
        self.cex_market_for(cex_id, base, quote)
    }
}

/// Builder for constructing [`CexMetadata`].
#[derive(Debug, Clone, Default)]
pub struct CexMetadataBuilder {
    market_metadata: HashMap<CexId, HashMap<AssetId, HashMap<AssetId, MarketMetadata>>>,
}

impl CexMetadataBuilder {
    /// Creates an empty CEX metadata builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds market metadata for a base/quote pair on a CEX.
    pub fn cex_market(
        &mut self,
        cex_id: CexId,
        base: AssetId,
        quote: AssetId,
        metadata: MarketMetadata,
    ) -> &mut Self {
        self.market_metadata
            .entry(cex_id)
            .or_default()
            .entry(base)
            .or_default()
            .insert(quote, metadata);
        self
    }

    /// Builds the [`CexMetadata`] from the collected market metadata.
    pub fn build(self) -> CexMetadata {
        CexMetadata::new(self.market_metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata() -> MarketMetadata {
        MarketMetadata::new(0.000_001, 0.01, 1.0)
    }

    #[test]
    fn empty_builder_builds_empty_metadata() {
        let metadata = CexMetadataBuilder::new().build();

        assert!(
            metadata
                .cex_market_for(&CexId::Binance, &AssetId::Bitcoin, &AssetId::TetherUSD)
                .is_none()
        );
    }

    #[test]
    fn adds_market_metadata_for_a_cex_base_and_quote() {
        let mut builder = CexMetadataBuilder::new();
        builder.cex_market(
            CexId::Binance,
            AssetId::Bitcoin,
            AssetId::TetherUSD,
            metadata(),
        );
        let cex_metadata = builder.build();

        assert_eq!(
            cex_metadata.cex_market_for(&CexId::Binance, &AssetId::Bitcoin, &AssetId::TetherUSD),
            Some(&metadata())
        );
        assert!(
            cex_metadata
                .cex_market_for(&CexId::Binance, &AssetId::Ethereum, &AssetId::TetherUSD)
                .is_none()
        );
    }

    #[test]
    fn market_metadata_is_extracted_from_market() {
        use crate::market_data::market::MarketBuilder;

        let market = MarketBuilder::new(0.001, 0.01, 5.0).build();

        assert_eq!(
            MarketMetadata::from(&market),
            MarketMetadata::new(0.001, 0.01, 5.0)
        );
    }
}
