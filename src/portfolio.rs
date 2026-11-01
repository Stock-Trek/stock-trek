use crate::pending_order::PendingOrder;
use rust_decimal::Decimal;
use std::collections::HashMap;
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest,
    quantity::Quantity, side::Side, tag::Tag,
};

/// A snapshot of asset balances and pending orders across CEX accounts.
#[derive(Debug, Clone)]
pub struct Portfolio {
    portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>,
}

/// Asset balances and pending orders for a single CEX account.
#[derive(Debug, Clone, Default)]
pub struct CexAccountPortfolio {
    asset_counts: HashMap<AssetId, Decimal>,
    pending_orders: HashMap<Tag, Vec<PendingOrder>>,
}

impl Portfolio {
    /// Creates a portfolio from CEX and account portfolios.
    pub fn new(portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>) -> Self {
        Self { portfolios }
    }
    /// Whether an account exists
    pub fn has_cex_account(&self, cex_id: &CexId, account_id: &AccountId) -> bool {
        self.portfolios
            .get(cex_id)
            .map(|map| map.contains_key(account_id))
            .unwrap_or_default()
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

    /// Returns the portfolio for a specific CEX account, if it exists.
    ///
    /// This is an alias for [`Portfolio::cex_account`].
    pub fn account(&self, cex_id: &CexId, account_id: &AccountId) -> Option<&CexAccountPortfolio> {
        self.cex_account(cex_id, account_id)
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
        self.portfolios
            .values()
            .flat_map(|account_map| account_map.values())
            .flat_map(|portfolio| portfolio.pending_orders.values())
            .map(|orders| orders.len())
            .sum()
    }
    /// Number of pending orders with the given `tag` across all accounts.
    pub fn pending_orders_with_tag_count(&self, tag: &Tag) -> usize {
        self.portfolios
            .values()
            .flat_map(|account_map| account_map.values())
            .filter_map(|portfolio| portfolio.pending_orders.get(tag))
            .map(|orders| orders.len())
            .sum()
    }
    /// Number of pending orders in a specific CEX account.
    pub fn pending_orders_in_cex_account_count(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> usize {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .map(|portfolio| {
                portfolio
                    .pending_orders
                    .values()
                    .map(|orders| orders.len())
                    .sum()
            })
            .unwrap_or(0)
    }
    /// Number of pending orders with the given `tag` in a specific CEX account.
    pub fn pending_orders_in_cex_account_with_tag_count(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
        tag: &Tag,
    ) -> usize {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .and_then(|portfolio| portfolio.pending_orders.get(tag))
            .map(|orders| orders.len())
            .unwrap_or(0)
    }

    /// Total `asset_id` held across **all** accounts (counts available and reserved).
    pub fn asset_total(&self, asset_id: &AssetId) -> Decimal {
        self.portfolios
            .values()
            .flat_map(|accounts| accounts.values())
            .flat_map(|account| account.asset_counts.get(asset_id))
            .copied()
            .sum()
    }
    /// `asset_id` held in a **specific** CEX account (counts available and reserved).
    pub fn asset_total_in_cex_account(
        &self,
        asset_id: &AssetId,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> Decimal {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .and_then(|account| account.asset_counts.get(asset_id))
            .copied()
            .unwrap_or(Decimal::ZERO)
    }
    /// Total available (free) amount of `asset_id` across all accounts (total - reserved).
    ///
    /// The result is clamped to zero when pending orders reserve more than the
    /// portfolio holds.
    pub fn asset_available(&self, asset_id: &AssetId) -> Decimal {
        Self::available_quantity(self.asset_total(asset_id), self.asset_reserved(asset_id))
    }
    /// Available (free) amount of `asset_id` in a specific CEX account (total - reserved).
    ///
    /// The result is clamped to zero when pending orders reserve more than the
    /// account holds.
    pub fn asset_available_in_cex_account(
        &self,
        asset_id: &AssetId,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> Decimal {
        Self::available_quantity(
            self.asset_total_in_cex_account(asset_id, cex_id, account_id),
            self.asset_reserved_in_cex_account(asset_id, cex_id, account_id),
        )
    }
    /// Total amount of `asset_id` reserved (locked in unfilled pending orders) across all accounts.
    pub fn asset_reserved(&self, asset_id: &AssetId) -> Decimal {
        let mut total = Decimal::ZERO;
        for accounts in self.portfolios.values() {
            for account in accounts.values() {
                for pending_orders in account.pending_orders.values() {
                    for pending_order in pending_orders {
                        total += self.reserved_quantity_for_pending_order(pending_order, asset_id);
                    }
                }
            }
        }
        total
    }
    /// Amount of `asset_id` reserved (locked in unfilled pending orders) in a specific CEX account.
    pub fn asset_reserved_in_cex_account(
        &self,
        asset_id: &AssetId,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> Decimal {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .map(|portfolio| {
                portfolio
                    .pending_orders
                    .values()
                    .flatten()
                    .map(|pending_order| {
                        self.reserved_quantity_for_pending_order(pending_order, asset_id)
                    })
                    .sum::<Decimal>()
            })
            .unwrap_or(Decimal::ZERO)
    }

    fn reserved_quantity_for_pending_order(
        &self,
        pending_order: &PendingOrder,
        asset_id: &AssetId,
    ) -> Decimal {
        match pending_order.order_request() {
            OrderRequest::Limit {
                base,
                side: Side::Sell,
                limit_price,
                quantity,
                ..
            } if asset_id == base => {
                let base_quantity = match quantity {
                    Quantity::OfBase(base_quantity) => *base_quantity,
                    Quantity::OfQuote(quote_quantity) => {
                        if limit_price.is_zero() {
                            Decimal::ZERO
                        } else {
                            *quote_quantity / limit_price
                        }
                    }
                };
                Self::remaining_quantity(base_quantity, pending_order.filled_base_quantity())
            }
            OrderRequest::Limit {
                quote,
                side: Side::Buy,
                limit_price,
                quantity,
                ..
            } if asset_id == quote => {
                let quote_quantity = match quantity {
                    Quantity::OfBase(base_quantity) => base_quantity * limit_price,
                    Quantity::OfQuote(quote_quantity) => *quote_quantity,
                };
                Self::remaining_quantity(quote_quantity, pending_order.filled_quote_quantity())
            }
            OrderRequest::MarketBuy {
                quote,
                quote_quantity,
                ..
            } if asset_id == quote => {
                Self::remaining_quantity(*quote_quantity, pending_order.filled_quote_quantity())
            }
            OrderRequest::MarketSell {
                base,
                base_quantity,
                ..
            } if asset_id == base => {
                Self::remaining_quantity(*base_quantity, pending_order.filled_base_quantity())
            }
            _ => Decimal::ZERO,
        }
    }

    fn remaining_quantity(ordered: Decimal, filled: Decimal) -> Decimal {
        (ordered - filled).max(Decimal::ZERO)
    }

    fn available_quantity(total: Decimal, reserved: Decimal) -> Decimal {
        (total - reserved).max(Decimal::ZERO)
    }
}

impl CexAccountPortfolio {
    /// Creates an account portfolio from asset balances and pending orders.
    pub fn new(
        asset_counts: HashMap<AssetId, Decimal>,
        pending_orders: HashMap<Tag, Vec<PendingOrder>>,
    ) -> Self {
        Self {
            asset_counts,
            pending_orders,
        }
    }

    /// Returns all asset balances (available and reserved) keyed by asset.
    pub fn asset_counts(&self) -> &HashMap<AssetId, Decimal> {
        &self.asset_counts
    }

    /// Iterates over all asset balances keyed by asset.
    pub fn assets(&self) -> impl Iterator<Item = (&AssetId, &Decimal)> {
        self.asset_counts.iter()
    }

    /// Returns the total balance (available and reserved) of an asset.
    ///
    /// Returns zero when the account does not hold the asset.
    pub fn asset_count(&self, asset_id: &AssetId) -> Decimal {
        self.asset_counts
            .get(asset_id)
            .copied()
            .unwrap_or(Decimal::ZERO)
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
        self.pending_orders
            .iter()
            .flat_map(|(tag, orders)| orders.iter().map(move |order| (tag, order)))
    }
}

/// Builder for constructing a [`Portfolio`].
#[derive(Debug, Clone, Default)]
pub struct PortfolioBuilder {
    portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>,
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
        count: Decimal,
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
    /// The order is grouped under the tag embedded in `order_request`.
    pub fn pending_order(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        order_request: OrderRequest<AssetId, Decimal>,
        filled_base_quantity: Decimal,
        filled_quote_quantity: Decimal,
    ) -> &mut Self {
        let tag = Self::order_request_tag(&order_request).clone();
        self.portfolios
            .entry(cex_id)
            .or_default()
            .entry(account_id)
            .or_default()
            .pending_orders
            .entry(tag)
            .or_default()
            .push(PendingOrder::new(
                order_request,
                filled_base_quantity,
                filled_quote_quantity,
            ));
        self
    }

    /// Builds the [`Portfolio`] from the collected data.
    pub fn build(self) -> Portfolio {
        Portfolio::new(self.portfolios)
    }

    fn order_request_tag(order_request: &OrderRequest<AssetId, Decimal>) -> &Tag {
        match order_request {
            OrderRequest::Limit { tag, .. }
            | OrderRequest::MarketBuy { tag, .. }
            | OrderRequest::MarketSell { tag, .. } => tag,
        }
    }
}

#[cfg(test)]
mod tests;
