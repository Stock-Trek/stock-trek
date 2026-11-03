use super::{CexAccountPortfolio, Portfolio};
use crate::{error::result::StockTrekResult, pending_order::PendingOrder, validation::Validation};
use std::collections::BTreeMap;
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest,
};

/// Builder for constructing a [`Portfolio`].
///
/// Invalid asset counts and pending orders are rejected immediately by the
/// method that adds them, so an invalid builder can never be constructed.
/// Setter methods borrow the builder mutably, so a rejected value can be
/// corrected and retried without rebuilding the collected data.
#[derive(Debug, Clone, Default)]
pub struct PortfolioBuilder {
    portfolios: BTreeMap<CexId, BTreeMap<AccountId, CexAccountPortfolio>>,
}

impl PortfolioBuilder {
    /// Creates an empty portfolio builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the balance of an asset in a CEX account.
    ///
    /// Returns an error when `count` is negative or not finite.
    pub fn asset_count(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
        count: f64,
    ) -> StockTrekResult<&mut Self> {
        Validation::non_negative("PortfolioBuilder::asset_count", "asset count", count)?;
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_default()
            .asset_counts
            .insert(asset_id, count);
        Ok(self)
    }

    /// Adds a pending order to a CEX account.
    ///
    /// The order is grouped under the tag embedded in `order_request`. Returns
    /// an error when the pending order is invalid.
    pub fn pending_order(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        order_request: OrderRequest<AssetId, f64>,
        filled_base_quantity: f64,
        filled_quote_quantity: f64,
    ) -> StockTrekResult<&mut Self> {
        let pending_order =
            PendingOrder::new(order_request, filled_base_quantity, filled_quote_quantity)?;
        let tag = pending_order.tag().clone();
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_default()
            .pending_orders
            .entry(tag)
            .or_default()
            .push(pending_order);
        Ok(self)
    }

    /// Builds the [`Portfolio`] from the collected data.
    pub fn build(self) -> Portfolio {
        Portfolio {
            portfolios: self.portfolios,
        }
    }
}
