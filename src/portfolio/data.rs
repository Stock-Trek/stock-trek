use super::CexAccountPortfolio;
use crate::{
    error::result::{StockTrekError, StockTrekResult},
    pending_order::PendingOrder,
    validation::Validation,
};
use std::collections::BTreeMap;
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};

/// A snapshot of asset balances and pending orders across CEX accounts.
#[derive(Debug, Clone, Default)]
pub struct Portfolio {
    pub(super) portfolios: BTreeMap<CexId, BTreeMap<AccountId, CexAccountPortfolio>>,
}

fn duplicate_account_error(cex_id: CexId, account_id: &AccountId) -> StockTrekError {
    Validation::error(
        "Portfolio::new",
        format!("duplicate account {account_id:?} for CEX {cex_id}"),
    )
}

impl Portfolio {
    /// Creates a portfolio from flattened `(cex_id, account_id, portfolio)` tuples.
    ///
    /// Accepts any iterator of `(cex_id, account_id, portfolio)` tuples, such as
    /// a `Vec`, an array, or the `flatten`ed output of a map.
    ///
    /// Returns an error when the same CEX account appears more than once.
    pub fn new<I>(portfolios: I) -> StockTrekResult<Self>
    where
        I: IntoIterator<Item = (CexId, AccountId, CexAccountPortfolio)>,
    {
        let mut portfolios_by_cex: BTreeMap<CexId, BTreeMap<AccountId, CexAccountPortfolio>> =
            BTreeMap::new();
        for (cex_id, account_id, portfolio) in portfolios {
            let accounts_by_id = portfolios_by_cex.entry(cex_id).or_default();
            if accounts_by_id.contains_key(&account_id) {
                return Err(duplicate_account_error(cex_id, &account_id));
            }
            accounts_by_id.insert(account_id, portfolio);
        }
        Ok(Self {
            portfolios: portfolios_by_cex,
        })
    }
    /// Whether an account exists.
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
    /// Each item is `(cex_id, account_id, asset_id, total)`, where `total`
    /// includes both available and reserved amounts.
    pub fn assets_iter(&self) -> impl Iterator<Item = (&CexId, &AccountId, &AssetId, &f64)> {
        self.cex_accounts()
            .flat_map(|(cex_id, account_id, portfolio)| {
                portfolio
                    .asset_totals()
                    .map(move |(asset_id, total)| (cex_id, account_id, asset_id, total))
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
            .flat_map(|portfolio| portfolio.asset_totals())
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
            .map(|(_, _, _, total)| *total)
            .sum()
    }
}
