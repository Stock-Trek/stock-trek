use crate::{
    error::result::{StockTrekError, StockTrekResult},
    pending_order::PendingOrder,
    validation::Validation,
};
use std::collections::{BTreeMap, HashMap};
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, tag::Tag,
};

/// A snapshot of asset balances and pending orders across CEX accounts.
///
/// Balances include both available and reserved amounts. Reserved amounts are
/// derived from unfilled pending orders, so a strategy can ask for either the
/// total holding or the portion it can still spend. Pending orders are grouped
/// by their exchange tag, which lets a strategy cancel or count only the orders
/// it created for a particular purpose.
#[derive(Debug, Clone, Default)]
pub struct Portfolio {
    portfolios: BTreeMap<CexId, BTreeMap<AccountId, CexAccountPortfolio>>,
}

/// Asset balances and pending orders for a single CEX account.
///
/// Asset counts are the total balances reported by the exchange. Pending-order
/// reservations are calculated from the unfilled quantities of orders that are
/// still open.
#[derive(Debug, Clone, Default)]
pub struct CexAccountPortfolio {
    asset_counts: BTreeMap<AssetId, f64>,
    pending_orders: BTreeMap<Tag, Vec<PendingOrder>>,
}

impl Portfolio {
    /// Creates a portfolio from CEX and account portfolios.
    pub fn new(portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>) -> Self {
        let portfolios = portfolios
            .into_iter()
            .map(|(cex_id, accounts)| (cex_id, accounts.into_iter().collect()))
            .collect();
        Self { portfolios }
    }
    /// Whether an account exists
    pub fn has_cex_account(&self, cex_id: &CexId, account_id: &AccountId) -> bool {
        self.cex_account(cex_id, account_id).is_some()
    }

    /// Iterates over every CEX account and its portfolio.
    ///
    /// Each item is `(cex_id, account_id, account_portfolio)`. This is the
    /// primary way for an algorithm to discover which accounts it can use.
    pub fn cex_accounts(&self) -> impl Iterator<Item = (&CexId, &AccountId, &CexAccountPortfolio)> {
        self.portfolios.iter().flat_map(|(cex_id, accounts)| {
            accounts
                .iter()
                .map(move |(account_id, portfolio)| (cex_id, account_id, portfolio))
        })
    }

    /// Iterates over every `(cex_id, account_id)` pair in the portfolio.
    pub fn accounts(&self) -> impl Iterator<Item = (&CexId, &AccountId)> {
        self.cex_accounts()
            .map(|(cex_id, account_id, _)| (cex_id, account_id))
    }

    /// Iterates over the account identifiers in a specific CEX.
    pub fn accounts_in_cex(&self, cex_id: &CexId) -> impl Iterator<Item = &AccountId> {
        self.portfolios
            .get(cex_id)
            .into_iter()
            .flat_map(|accounts| accounts.keys())
    }

    /// Returns the portfolio for a specific CEX account, if it exists.
    pub fn cex_account(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> Option<&CexAccountPortfolio> {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
    }

    /// Iterates over every asset balance with its CEX, account, and asset.
    ///
    /// Each item is `(cex_id, account_id, asset_id, count)`, where `count`
    /// includes both available and reserved amounts.
    pub fn assets_iter(&self) -> impl Iterator<Item = (&CexId, &AccountId, &AssetId, &f64)> {
        self.cex_accounts()
            .flat_map(|(cex_id, account_id, portfolio)| {
                portfolio
                    .asset_counts()
                    .map(move |(asset_id, count)| (cex_id, account_id, asset_id, count))
            })
    }

    /// Iterates over the asset balances in a specific CEX account.
    pub fn assets_in_cex_account_iter(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> impl Iterator<Item = (&AssetId, &f64)> {
        self.cex_account(cex_id, account_id)
            .into_iter()
            .flat_map(|portfolio| portfolio.asset_counts())
    }

    /// Iterates over every pending order with its CEX, account, and tag.
    ///
    /// Each item is `(cex_id, account_id, tag, pending_order)`.
    pub fn pending_orders_iter(
        &self,
    ) -> impl Iterator<Item = (&CexId, &AccountId, &Tag, &PendingOrder)> {
        self.cex_accounts()
            .flat_map(|(cex_id, account_id, portfolio)| {
                portfolio
                    .pending_orders_iter()
                    .map(move |(tag, order)| (cex_id, account_id, tag, order))
            })
    }

    /// Iterates over the pending orders in a specific CEX account.
    pub fn pending_orders_in_cex_account_iter(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> impl Iterator<Item = (&Tag, &PendingOrder)> {
        self.cex_account(cex_id, account_id)
            .into_iter()
            .flat_map(|portfolio| portfolio.pending_orders_iter())
    }

    /// Iterates over the pending orders with `tag` in a specific CEX account.
    pub fn pending_orders_in_cex_account_with_tag_iter<'a>(
        &'a self,
        cex_id: &CexId,
        account_id: &AccountId,
        tag: &'a Tag,
    ) -> impl Iterator<Item = &'a PendingOrder> {
        self.cex_account(cex_id, account_id)
            .and_then(|portfolio| portfolio.pending_orders_with_tag(tag))
            .into_iter()
            .flatten()
    }

    /// Total number of pending orders across all CEXes and accounts.
    pub fn pending_orders_count(&self) -> usize {
        self.pending_orders_iter().count()
    }
    /// Number of pending orders with the given `tag` across all accounts.
    pub fn pending_orders_with_tag_count(&self, tag: &Tag) -> usize {
        self.pending_orders_iter()
            .filter(|(_, _, order_tag, _)| *order_tag == tag)
            .count()
    }
    /// Number of pending orders in a specific CEX account.
    pub fn pending_orders_in_cex_account_count(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> usize {
        self.pending_orders_in_cex_account_iter(cex_id, account_id)
            .count()
    }
    /// Number of pending orders with the given `tag` in a specific CEX account.
    pub fn pending_orders_in_cex_account_with_tag_count(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
        tag: &Tag,
    ) -> usize {
        self.pending_orders_in_cex_account_with_tag_iter(cex_id, account_id, tag)
            .count()
    }

    /// Total `asset_id` held across **all** accounts (counts available and reserved).
    pub fn asset_total(&self, asset_id: &AssetId) -> f64 {
        self.assets_iter()
            .filter(|(_, _, id, _)| *id == asset_id)
            .map(|(_, _, _, count)| *count)
            .sum()
    }
    /// `asset_id` held in a **specific** CEX account (counts available and reserved).
    pub fn asset_total_in_cex_account(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
        asset_id: &AssetId,
    ) -> f64 {
        self.assets_in_cex_account_iter(cex_id, account_id)
            .filter(|(id, _)| *id == asset_id)
            .map(|(_, count)| *count)
            .sum()
    }
}

impl CexAccountPortfolio {
    /// Creates an account portfolio from asset balances and pending orders.
    ///
    /// Returns an error when any asset balance is negative or not finite.
    pub fn new(
        asset_counts: HashMap<AssetId, f64>,
        pending_orders: HashMap<Tag, Vec<PendingOrder>>,
    ) -> StockTrekResult<Self> {
        for count in asset_counts.values() {
            Validation::non_negative("CexAccountPortfolio::new", "asset balance", *count)?;
        }
        let asset_counts = asset_counts.into_iter().collect();
        let pending_orders = pending_orders.into_iter().collect();
        Ok(Self {
            asset_counts,
            pending_orders,
        })
    }

    /// Iterates over all asset balances keyed by asset.
    pub fn asset_counts(&self) -> impl Iterator<Item = (&AssetId, &f64)> {
        self.asset_counts.iter()
    }

    /// Returns the total balance (available and reserved) of an asset.
    ///
    /// Returns zero when the account does not hold the asset.
    pub fn asset_count(&self, asset_id: &AssetId) -> f64 {
        self.asset_counts.get(asset_id).copied().unwrap_or(0.0)
    }

    /// Iterates over all pending orders grouped by tag.
    pub fn pending_orders(&self) -> impl Iterator<Item = (&Tag, &[PendingOrder])> {
        self.pending_orders
            .iter()
            .map(|(tag, orders)| (tag, orders.as_slice()))
    }

    /// Returns the pending orders with the given tag, if any.
    pub fn pending_orders_with_tag(&self, tag: &Tag) -> Option<&[PendingOrder]> {
        self.pending_orders.get(tag).map(Vec::as_slice)
    }

    /// Iterates over every pending order together with its tag.
    pub fn pending_orders_iter(&self) -> impl Iterator<Item = (&Tag, &PendingOrder)> {
        self.pending_orders
            .iter()
            .flat_map(|(tag, orders)| orders.iter().map(move |order| (tag, order)))
    }
}

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
    pub fn asset_count(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
        count: f64,
    ) -> &mut Self {
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
    pub fn pending_order(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        order_request: OrderRequest<AssetId, f64>,
        filled_base_quantity: f64,
        filled_quote_quantity: f64,
    ) -> &mut Self {
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

mod reservation;

#[cfg(test)]
mod tests;
