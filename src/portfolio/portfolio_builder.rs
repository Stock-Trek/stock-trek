use super::{CexAccountPortfolio, Portfolio};
use crate::{
    error::result::{StockTrekError, StockTrekResult},
    pending_order::PendingOrder,
    validation::Validation,
};
use std::collections::BTreeMap;
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, tag::Tag,
};

/// Builder for constructing a [`Portfolio`].
#[derive(Debug, Clone, Default)]
pub struct PortfolioBuilder {
    portfolios: BTreeMap<CexId, BTreeMap<AccountId, CexAccountPortfolio>>,
    error: Option<StockTrekError>,
}

impl PortfolioBuilder {
    /// Creates an empty portfolio builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the balance of an asset in a CEX account.
    #[must_use]
    pub fn asset_count(
        mut self,
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
        count: f64,
    ) -> Self {
        if let Err(error) =
            Validation::non_negative("PortfolioBuilder::asset_count", "asset count", count)
        {
            self.record_error(error);
            return self;
        }
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_default()
            .asset_counts
            .insert(asset_id, count);
        self
    }

    /// Adds a pending order to a CEX account.
    ///
    /// The order is grouped under the tag embedded in `order_request`.
    #[must_use]
    pub fn pending_order(
        mut self,
        cex_id: CexId,
        account_id: AccountId,
        order_request: OrderRequest<AssetId, f64>,
        filled_base_quantity: f64,
        filled_quote_quantity: f64,
    ) -> Self {
        let pending_order =
            match PendingOrder::new(order_request, filled_base_quantity, filled_quote_quantity) {
                Ok(pending_order) => pending_order,
                Err(error) => {
                    self.record_error(error);
                    return self;
                }
            };
        let tag = Self::order_request_tag(pending_order.order_request()).clone();
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_default()
            .pending_orders
            .entry(tag)
            .or_default()
            .push(pending_order);
        self
    }

    /// Builds the [`Portfolio`] from the collected data.
    ///
    /// Returns an error when an asset count or pending order added to the
    /// builder was invalid.
    pub fn build(self) -> StockTrekResult<Portfolio> {
        if let Some(error) = self.error {
            return Err(error);
        }
        Ok(Portfolio {
            portfolios: self.portfolios,
        })
    }

    fn record_error(&mut self, error: StockTrekError) {
        if self.error.is_none() {
            self.error = Some(error);
        }
    }

    fn order_request_tag(order_request: &OrderRequest<AssetId, f64>) -> &Tag {
        match order_request {
            OrderRequest::Limit { tag, .. }
            | OrderRequest::MarketBuy { tag, .. }
            | OrderRequest::MarketSell { tag, .. } => tag,
        }
    }
}
