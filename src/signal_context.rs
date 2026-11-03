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
    pub fn cex_market_for(&self, cex_id: CexId, base: AssetId, quote: AssetId) -> Option<&Market> {
        self.cex_market_data
            .get(&cex_id)
            .and_then(|m| m.markets_by_base.get(&base))
            .and_then(|m| m.markets_by_quote.get(&quote))
    }
}

/// Builder for constructing a [`SignalContext`].
///
/// Setter methods only collect data. All validation happens at build time:
/// [`SignalContextBuilder::build`] rejects duplicate markets.
#[derive(Debug, Clone, Default)]
pub struct SignalContextBuilder {
    cex_market_data: Vec<(CexId, AssetId, AssetId, Market)>,
}

impl SignalContextBuilder {
    /// Creates an empty signal context builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a market for a base/quote pair on a CEX.
    ///
    /// The market is validated when [`SignalContextBuilder::build`] is
    /// called: the market for the same CEX and base/quote pair must not have
    /// been added already.
    pub fn cex_market(
        &mut self,
        cex_id: CexId,
        base: AssetId,
        quote: AssetId,
        market: Market,
    ) -> &mut Self {
        self.cex_market_data.push((cex_id, base, quote, market));
        self
    }

    /// Builds the [`SignalContext`] from the collected market data.
    ///
    /// Returns an error when the market for a CEX and base/quote pair has
    /// been added more than once.
    pub fn build(self) -> StockTrekResult<SignalContext> {
        let mut markets: BTreeMap<CexId, BTreeMap<AssetId, Vec<(AssetId, Market)>>> =
            BTreeMap::new();
        for (cex_id, base, quote, market) in self.cex_market_data {
            markets
                .entry(cex_id)
                .or_default()
                .entry(base)
                .or_default()
                .push((quote, market));
        }

        let mut cex_market_data = Vec::with_capacity(markets.len());
        for (cex_id, markets_by_base) in markets {
            let mut bases = Vec::with_capacity(markets_by_base.len());
            for (base, markets_by_quote) in markets_by_base {
                bases.push((base, CexMarketDataByQuoteContext::new(markets_by_quote)?));
            }
            cex_market_data.push((cex_id, CexMarketDataByBaseContext::new(bases)?));
        }
        SignalContext::new(cex_market_data)
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
