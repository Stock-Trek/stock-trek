use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    pending_order::PendingOrder,
};
use std::{borrow::Cow, collections::HashMap};
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest, tag::Tag,
};

/// A snapshot of asset balances and pending orders across CEX accounts.
#[derive(Debug, Clone, Default)]
pub struct Portfolio {
    portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>,
}

/// Asset balances and pending orders for a single CEX account.
#[derive(Debug, Clone, Default)]
pub struct CexAccountPortfolio {
    asset_counts: HashMap<AssetId, f64>,
    pending_orders: HashMap<Tag, Vec<PendingOrder>>,
}

impl Portfolio {
    /// Creates a portfolio from CEX and account portfolios.
    pub fn new(portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>) -> Self {
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
        sorted_entries(&self.portfolios).flat_map(|(cex_id, accounts)| {
            sorted_entries(accounts)
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
            .flat_map(|accounts| sorted_entries(accounts).map(|(account_id, _)| account_id))
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
                    .assets()
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
            .flat_map(|portfolio| portfolio.assets())
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
                    .pending_orders
                    .iter()
                    .flat_map(move |(tag, orders)| {
                        orders
                            .iter()
                            .map(move |order| (cex_id, account_id, tag, order))
                    })
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

trait PortfolioSortKey {
    fn sort_key(&self) -> Cow<'_, str>;
}

impl PortfolioSortKey for CexId {
    fn sort_key(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}

impl PortfolioSortKey for AssetId {
    fn sort_key(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}

impl PortfolioSortKey for AccountId {
    fn sort_key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.0)
    }
}

impl PortfolioSortKey for Tag {
    fn sort_key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.0)
    }
}

fn sorted_entries<'a, K, V>(map: &'a HashMap<K, V>) -> impl Iterator<Item = (&'a K, &'a V)>
where
    K: PortfolioSortKey,
{
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|(left, _), (right, _)| left.sort_key().cmp(&right.sort_key()));
    entries.into_iter()
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
            Self::validate_asset_count("CexAccountPortfolio::new", "asset balance", *count)?;
        }
        Ok(Self {
            asset_counts,
            pending_orders,
        })
    }

    /// Returns all asset balances (available and reserved) keyed by asset.
    pub fn asset_counts(&self) -> &HashMap<AssetId, f64> {
        &self.asset_counts
    }

    /// Iterates over all asset balances keyed by asset.
    pub fn assets(&self) -> impl Iterator<Item = (&AssetId, &f64)> {
        sorted_entries(&self.asset_counts)
    }

    /// Returns the total balance (available and reserved) of an asset.
    ///
    /// Returns zero when the account does not hold the asset.
    pub fn asset_count(&self, asset_id: &AssetId) -> f64 {
        self.asset_counts.get(asset_id).copied().unwrap_or(0.0)
    }

    /// Returns all pending orders grouped by tag.
    pub fn pending_orders(&self) -> &HashMap<Tag, Vec<PendingOrder>> {
        &self.pending_orders
    }

    /// Returns the pending orders with the given tag, if any.
    pub fn pending_orders_with_tag(&self, tag: &Tag) -> Option<&[PendingOrder]> {
        self.pending_orders.get(tag).map(Vec::as_slice)
    }

    /// Iterates over every pending order together with its tag.
    pub fn pending_orders_iter(&self) -> impl Iterator<Item = (&Tag, &PendingOrder)> {
        sorted_entries(&self.pending_orders)
            .flat_map(|(tag, orders)| orders.iter().map(move |order| (tag, order)))
    }

    fn validate_asset_count(
        function: &'static str,
        field: &'static str,
        value: f64,
    ) -> StockTrekResult<()> {
        if !value.is_finite() {
            return Err(StockTrekError::Stats(StatsError::DomainError {
                function,
                message: format!("{field} {value} is not finite (NaN or infinity)"),
            }));
        }
        if value < 0.0 {
            return Err(StockTrekError::Stats(StatsError::DomainError {
                function,
                message: format!("{field} {value} must be non-negative"),
            }));
        }
        Ok(())
    }
}

/// Builder for constructing a [`Portfolio`].
#[derive(Debug, Clone, Default)]
pub struct PortfolioBuilder {
    portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>,
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
        if let Err(error) = CexAccountPortfolio::validate_asset_count(
            "PortfolioBuilder::asset_count",
            "asset count",
            count,
        ) {
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
        Ok(Portfolio::new(self.portfolios))
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
