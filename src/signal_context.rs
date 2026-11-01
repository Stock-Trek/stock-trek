use crate::{market_data::market::Market, portfolio::Portfolio};
use std::collections::HashMap;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

/// The market data and portfolio available while computing signals.
#[derive(Debug, Clone)]
pub struct SignalContext {
    cex_market_data: HashMap<CexId, CexMarketDataByBaseContext>,
    /// The current portfolio state, including all CEX accounts and pending orders.
    pub portfolio: Portfolio,
}

impl SignalContext {
    /// Creates a signal context from market data and the current portfolio.
    pub fn new(
        cex_market_data: HashMap<CexId, CexMarketDataByBaseContext>,
        portfolio: Portfolio,
    ) -> Self {
        Self {
            cex_market_data,
            portfolio,
        }
    }
    /// Creates a signal context from market data and an empty portfolio.
    pub fn from_market_data(cex_market_data: HashMap<CexId, CexMarketDataByBaseContext>) -> Self {
        Self::new(cex_market_data, Portfolio::new(HashMap::new()))
    }
    /// Returns the current portfolio state.
    pub fn portfolio(&self) -> &Portfolio {
        &self.portfolio
    }
    /// Replaces the portfolio state, returning the updated context.
    pub fn with_portfolio(mut self, portfolio: Portfolio) -> Self {
        self.portfolio = portfolio;
        self
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PortfolioBuilder;

    #[test]
    fn exposes_portfolio_accounts_to_algorithms() {
        let mut builder = PortfolioBuilder::new();
        builder.asset_count(
            CexId::Binance,
            stock_trek_types::cex::account_id::AccountId::new("main"),
            AssetId::Bitcoin,
            rust_decimal::Decimal::ONE,
        );

        let context = SignalContext::new(HashMap::new(), builder.build());
        let accounts = context.portfolio().accounts().collect::<Vec<_>>();

        assert_eq!(accounts.len(), 1);
        assert_eq!(
            accounts[0].1,
            &stock_trek_types::cex::account_id::AccountId::new("main")
        );
    }

    #[test]
    fn can_be_created_without_a_portfolio() {
        let context = SignalContext::from_market_data(HashMap::new());

        assert_eq!(context.portfolio().accounts().count(), 0);
    }
}
