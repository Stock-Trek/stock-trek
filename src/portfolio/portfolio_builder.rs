use super::{CexAccountPortfolio, Portfolio};
use crate::{error::result::StockTrekResult, pending_order::PendingOrder, validation::Validation};
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
    asset_counts: BTreeMap<AssetId, f64>,
    pending_orders: BTreeMap<Tag, Vec<PendingOrderData>>,
}

/// Builder for constructing a [`Portfolio`].
///
/// Setter methods only collect data. All validation happens at build time:
/// [`PortfolioBuilder::build`] rejects invalid asset counts and invalid
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

    /// Sets the balance of an asset in a CEX account.
    ///
    /// The count is validated when [`PortfolioBuilder::build`] is called.
    pub fn asset_count(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
        count: f64,
    ) -> &mut Self {
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
    /// Returns an error when an asset count is negative or not finite, or when
    /// a pending order is invalid.
    pub fn build(self) -> StockTrekResult<Portfolio> {
        let function = "PortfolioBuilder::build";
        let mut portfolios: BTreeMap<CexId, BTreeMap<AccountId, CexAccountPortfolio>> =
            BTreeMap::new();
        for (cex_id, accounts) in self.portfolios {
            let accounts_by_id = portfolios.entry(cex_id).or_default();
            for (account_id, data) in accounts {
                let mut asset_counts = BTreeMap::new();
                for (asset_id, count) in data.asset_counts {
                    Validation::non_negative(function, "asset count", count)?;
                    asset_counts.insert(asset_id, count);
                }
                let mut pending_orders = BTreeMap::new();
                for (tag, orders) in data.pending_orders {
                    let mut validated_orders = Vec::with_capacity(orders.len());
                    for order in orders {
                        validated_orders.push(PendingOrder::new(
                            order.order_request,
                            order.filled_base_quantity,
                            order.filled_quote_quantity,
                        )?);
                    }
                    pending_orders.insert(tag, validated_orders);
                }
                accounts_by_id.insert(
                    account_id,
                    CexAccountPortfolio {
                        asset_counts,
                        pending_orders,
                    },
                );
            }
        }
        Ok(Portfolio { portfolios })
    }
}
