use crate::{error::result::StockTrekResult, market_data::market::Market, validation::Validation};
use std::collections::BTreeMap;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

/// The market data available while computing signals, grouped by CEX.
#[derive(Debug, Clone)]
pub struct SignalContext {
    cex_market_data: BTreeMap<CexId, CexMarketDataByBaseContext>,
}

impl SignalContext {
    /// Creates a signal context from market data grouped by CEX.
    ///
    /// Returns an error when the same CEX appears more than once.
    pub fn new(
        cex_market_data: impl IntoIterator<Item = (CexId, CexMarketDataByBaseContext)>,
    ) -> StockTrekResult<Self> {
        let mut markets_by_cex = BTreeMap::new();
        for (cex_id, cex_market_data) in cex_market_data {
            if markets_by_cex.insert(cex_id, cex_market_data).is_some() {
                return Err(Validation::error(
                    "SignalContext::new",
                    format!("duplicate market data for CEX {cex_id}"),
                ));
            }
        }
        Ok(Self {
            cex_market_data: markets_by_cex,
        })
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
    ///
    /// Returns an error when the market for this CEX and base/quote pair has
    /// already been added.
    pub fn cex_market(
        &mut self,
        cex_id: CexId,
        base: AssetId,
        quote: AssetId,
        market: Market,
    ) -> StockTrekResult<&mut Self> {
        let already_exists = self
            .cex_market_data
            .get(&cex_id)
            .and_then(|by_base| by_base.markets_by_base.get(&base))
            .and_then(|by_quote| by_quote.markets_by_quote.get(&quote))
            .is_some();
        if already_exists {
            return Err(Validation::error(
                "SignalContextBuilder::cex_market",
                format!("duplicate market for {base}/{quote} on CEX {cex_id}"),
            ));
        }

        self.cex_market_data
            .entry(cex_id)
            .or_default()
            .markets_by_base
            .entry(base)
            .or_default()
            .markets_by_quote
            .insert(quote, market);
        Ok(self)
    }

    /// Builds the [`SignalContext`] from the collected market data.
    pub fn build(self) -> SignalContext {
        SignalContext {
            cex_market_data: self.cex_market_data,
        }
    }
}

/// Market data grouped by base asset for a single CEX.
#[derive(Debug, Clone, Default)]
pub struct CexMarketDataByBaseContext {
    markets_by_base: BTreeMap<AssetId, CexMarketDataByQuoteContext>,
}

impl CexMarketDataByBaseContext {
    /// Creates base-asset market data from markets grouped by quote asset.
    ///
    /// Returns an error when the same base asset appears more than once.
    pub fn new(
        markets_by_base: impl IntoIterator<Item = (AssetId, CexMarketDataByQuoteContext)>,
    ) -> StockTrekResult<Self> {
        let mut markets_by_base_map = BTreeMap::new();
        for (base, markets) in markets_by_base {
            if markets_by_base_map.insert(base, markets).is_some() {
                return Err(Validation::error(
                    "CexMarketDataByBaseContext::new",
                    format!("duplicate market data for base asset {base}"),
                ));
            }
        }
        Ok(Self {
            markets_by_base: markets_by_base_map,
        })
    }
}

/// Market data grouped by quote asset.
#[derive(Debug, Clone, Default)]
pub struct CexMarketDataByQuoteContext {
    markets_by_quote: BTreeMap<AssetId, Market>,
}

impl CexMarketDataByQuoteContext {
    /// Creates quote-asset market data from markets keyed by quote asset.
    ///
    /// Returns an error when the same quote asset appears more than once.
    pub fn new(
        markets_by_quote: impl IntoIterator<Item = (AssetId, Market)>,
    ) -> StockTrekResult<Self> {
        let mut markets_by_quote_map = BTreeMap::new();
        for (quote, market) in markets_by_quote {
            if markets_by_quote_map.insert(quote, market).is_some() {
                return Err(Validation::error(
                    "CexMarketDataByQuoteContext::new",
                    format!("duplicate market for quote asset {quote}"),
                ));
            }
        }
        Ok(Self {
            markets_by_quote: markets_by_quote_map,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::market::MarketBuilder;

    fn market() -> Market {
        MarketBuilder::new(0.000_001, 0.01, 1.0).unwrap().build()
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
            .unwrap()
            .cex_market(
                CexId::Binance,
                AssetId::Ethereum,
                AssetId::TetherUSD,
                market(),
            )
            .unwrap();
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

    #[test]
    fn constructors_reject_duplicate_keys() {
        let by_quote = CexMarketDataByQuoteContext::new([
            (AssetId::TetherUSD, market()),
            (AssetId::TetherUSD, market()),
        ]);
        assert!(by_quote.is_err());

        let by_quote = CexMarketDataByQuoteContext::new([(AssetId::TetherUSD, market())]).unwrap();
        let by_base = CexMarketDataByBaseContext::new([
            (AssetId::Bitcoin, by_quote.clone()),
            (AssetId::Bitcoin, by_quote.clone()),
        ]);
        assert!(by_base.is_err());

        let by_base = CexMarketDataByBaseContext::new([(AssetId::Bitcoin, by_quote)]).unwrap();
        let context =
            SignalContext::new([(CexId::Binance, by_base.clone()), (CexId::Binance, by_base)]);
        assert!(context.is_err());
    }

    #[test]
    fn builder_rejects_duplicate_markets() {
        let mut builder = SignalContextBuilder::new();
        builder
            .cex_market(
                CexId::Binance,
                AssetId::Bitcoin,
                AssetId::TetherUSD,
                market(),
            )
            .unwrap();

        let duplicate = builder.cex_market(
            CexId::Binance,
            AssetId::Bitcoin,
            AssetId::TetherUSD,
            market(),
        );

        assert!(duplicate.is_err());
    }
}
