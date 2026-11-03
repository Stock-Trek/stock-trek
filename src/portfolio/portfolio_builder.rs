use super::{CexAccountPortfolio, Portfolio};
use crate::{error::result::StockTrekResult, pending_order::PendingOrder};
use std::collections::BTreeMap;
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, tag::Tag,
};

/// The data collected for a single pending order by a [`PortfolioBuilder`].
#[derive(Debug, Clone)]
struct PendingOrderData {
    order_request: OrderRequest<AssetId, f64>,
    filled_base_quantity: f64,
    filled_quote_quantity: f64,
}

/// The data collected for a single CEX account by a [`PortfolioBuilder`].
#[derive(Debug, Clone, Default)]
struct CexAccountPortfolioData {
    asset_totals: BTreeMap<AssetId, f64>,
    pending_orders: BTreeMap<Tag, Vec<PendingOrderData>>,
}

/// Builder for constructing a [`Portfolio`].
///
/// Setter methods only collect data. All validation happens at build time:
/// [`PortfolioBuilder::build`] rejects invalid asset totals and invalid
/// pending orders.
#[derive(Debug, Clone, Default)]
pub struct PortfolioBuilder {
    portfolios: BTreeMap<CexId, BTreeMap<AccountId, CexAccountPortfolioData>>,
}

impl PortfolioBuilder {
    /// Creates an empty portfolio builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the total of an asset in a CEX account.
    ///
    /// The total is validated when [`PortfolioBuilder::build`] is called.
    pub fn asset_total(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
        total: f64,
    ) -> &mut Self {
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_default()
            .asset_totals
            .insert(asset_id, total);
        self
    }

    /// Adds a pending order to a CEX account.
    ///
    /// The order is grouped under the tag embedded in `order_request`. The
    /// order is validated when [`PortfolioBuilder::build`] is called.
    pub fn pending_order(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        order_request: OrderRequest<AssetId, f64>,
        filled_base_quantity: f64,
        filled_quote_quantity: f64,
    ) -> &mut Self {
        let tag = match &order_request {
            OrderRequest::Limit { tag, .. }
            | OrderRequest::MarketBuy { tag, .. }
            | OrderRequest::MarketSell { tag, .. } => tag.clone(),
        };
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_default()
            .pending_orders
            .entry(tag)
            .or_default()
            .push(PendingOrderData {
                order_request,
                filled_base_quantity,
                filled_quote_quantity,
            });
        self
    }

    /// Builds the [`Portfolio`] from the collected data.
    ///
    /// Returns an error when an asset total is negative or not finite, or when
    /// a pending order is invalid.
    pub fn build(self) -> StockTrekResult<Portfolio> {
        let mut accounts = Vec::new();
        for (cex_id, accounts_by_id) in self.portfolios {
            for (account_id, data) in accounts_by_id {
                let pending_orders = data
                    .pending_orders
                    .into_iter()
                    .map(|(tag, orders)| {
                        let orders = orders
                            .into_iter()
                            .map(|order| {
                                PendingOrder::new(
                                    order.order_request,
                                    order.filled_base_quantity,
                                    order.filled_quote_quantity,
                                )
                            })
                            .collect::<StockTrekResult<Vec<_>>>()?;
                        Ok((tag, orders))
                    })
                    .collect::<StockTrekResult<BTreeMap<_, _>>>()?;
                accounts.push((
                    cex_id,
                    account_id,
                    CexAccountPortfolio::new(data.asset_totals, pending_orders)?,
                ));
            }
        }
        Portfolio::new(accounts)
    }
}
