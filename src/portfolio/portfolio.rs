use super::CexAccountPortfolio;
use crate::pending_order::PendingOrder;
use std::collections::{BTreeMap, HashMap};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};

/// A snapshot of asset balances and pending orders across CEX accounts.
#[derive(Debug, Clone, Default)]
pub struct Portfolio {
    pub(super) portfolios: BTreeMap<CexId, BTreeMap<AccountId, CexAccountPortfolio>>,
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
