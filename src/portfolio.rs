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
    pub fn pending_orders(&self) -> f64 {
        self.portfolios
            .values()
            .flat_map(|account_map| account_map.values())
            .flat_map(|portfolio| portfolio.pending_orders.values())
            .map(|orders| orders.len())
            .sum::<usize>() as f64
    }
    /// Number of pending orders with the given `tag` across all accounts.
    pub fn pending_orders_with_tag(&self, tag: &Tag) -> f64 {
        self.portfolios
            .values()
            .flat_map(|account_map| account_map.values())
            .filter_map(|portfolio| portfolio.pending_orders.get(tag))
            .map(|orders| orders.len())
            .sum::<usize>() as f64
    }
    /// Number of pending orders in a specific CEX account.
    pub fn pending_orders_in_cex_account(&self, cex_id: &CexId, account_id: &AccountId) -> f64 {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .map(|portfolio| {
                portfolio
                    .pending_orders
                    .values()
                    .map(|orders| orders.len())
                    .sum::<usize>()
            })
            .unwrap_or(0) as f64
    }
    /// Number of pending orders with the given `tag` in a specific CEX account.
    pub fn pending_orders_in_cex_account_with_tag(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
        tag: &Tag,
    ) -> f64 {
        self.portfolios
            .get(cex_id)
            .and_then(|accounts| accounts.get(account_id))
            .and_then(|portfolio| portfolio.pending_orders.get(tag))
            .map(|orders| orders.len())
            .unwrap_or(0) as f64
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
    pub fn asset_available(&self, asset_id: &AssetId) -> Decimal {
        self.asset_total(asset_id) - self.asset_reserved(asset_id)
    }
    /// Available (free) amount of `asset_id` in a specific CEX account (total - reserved).
    pub fn asset_available_in_cex_account(
        &self,
        asset_id: &AssetId,
        cex_id: &CexId,
        account_id: &AccountId,
    ) -> Decimal {
        self.asset_total_in_cex_account(asset_id, cex_id, account_id)
            - self.asset_reserved_in_cex_account(asset_id, cex_id, account_id)
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
                remaining_quantity(base_quantity, pending_order.filled_base_quantity())
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
                remaining_quantity(quote_quantity, pending_order.filled_quote_quantity())
            }
            OrderRequest::MarketBuy {
                quote,
                quote_quantity,
                ..
            } if asset_id == quote => {
                remaining_quantity(*quote_quantity, pending_order.filled_quote_quantity())
            }
            OrderRequest::MarketSell {
                base,
                base_quantity,
                ..
            } if asset_id == base => {
                remaining_quantity(*base_quantity, pending_order.filled_base_quantity())
            }
            _ => Decimal::ZERO,
        }
    }
}

fn remaining_quantity(ordered: Decimal, filled: Decimal) -> Decimal {
    (ordered - filled).max(Decimal::ZERO)
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
    pub fn pending_order(
        &mut self,
        cex_id: CexId,
        account_id: AccountId,
        tag: Tag,
        order_request: OrderRequest<AssetId, Decimal>,
        filled_base_quantity: Decimal,
        filled_quote_quantity: Decimal,
    ) -> &mut Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use stock_trek_types::cex::{activation::Activation, time_in_force::TimeInForce};

    fn portfolio_with(
        order_request: OrderRequest<AssetId, Decimal>,
        filled_base_quantity: Decimal,
        filled_quote_quantity: Decimal,
    ) -> Portfolio {
        let mut builder = PortfolioBuilder::new();
        builder
            .asset_count(
                CexId::Binance,
                AccountId::new("account"),
                AssetId::Bitcoin,
                Decimal::from(100),
            )
            .pending_order(
                CexId::Binance,
                AccountId::new("account"),
                Tag::new("tag"),
                order_request,
                filled_base_quantity,
                filled_quote_quantity,
            );
        builder.build()
    }

    fn limit_sell(
        quantity: Quantity<Decimal>,
        limit_price: Decimal,
    ) -> OrderRequest<AssetId, Decimal> {
        OrderRequest::Limit {
            base: AssetId::Bitcoin,
            quote: AssetId::TetherUSD,
            side: Side::Sell,
            activation: Activation::Immediate,
            limit_price,
            time_in_force: TimeInForce::GoodTillCancelled,
            quantity,
            tag: Tag::new("tag"),
        }
    }

    #[test]
    fn reserved_quantity_is_clamped_when_filled_exceeds_ordered() {
        let portfolio = portfolio_with(
            limit_sell(Quantity::OfBase(Decimal::from(10)), Decimal::from(100)),
            Decimal::from(15),
            Decimal::ZERO,
        );

        assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), Decimal::ZERO);
        assert_eq!(
            portfolio.asset_available(&AssetId::Bitcoin),
            portfolio.asset_total(&AssetId::Bitcoin)
        );
    }

    #[test]
    fn zero_price_quote_limited_sell_reserves_nothing() {
        let portfolio = portfolio_with(
            limit_sell(Quantity::OfQuote(Decimal::from(1_000)), Decimal::ZERO),
            Decimal::ZERO,
            Decimal::ZERO,
        );

        assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), Decimal::ZERO);
    }

    #[test]
    fn partial_fill_reserves_remaining_quantity() {
        let portfolio = portfolio_with(
            limit_sell(Quantity::OfBase(Decimal::from(10)), Decimal::from(100)),
            Decimal::from(4),
            Decimal::ZERO,
        );

        assert_eq!(
            portfolio.asset_reserved(&AssetId::Bitcoin),
            Decimal::from(6)
        );
    }

    #[test]
    fn market_orders_reserve_quantities_are_clamped() {
        let market_buy = OrderRequest::MarketBuy {
            base: AssetId::Bitcoin,
            quote: AssetId::TetherUSD,
            activation: Activation::Immediate,
            quote_quantity: Decimal::from(100),
            tag: Tag::new("tag"),
        };
        let portfolio = portfolio_with(market_buy, Decimal::ZERO, Decimal::from(150));
        assert_eq!(portfolio.asset_reserved(&AssetId::TetherUSD), Decimal::ZERO);

        let market_sell = OrderRequest::MarketSell {
            base: AssetId::Bitcoin,
            quote: AssetId::TetherUSD,
            activation: Activation::Immediate,
            base_quantity: Decimal::from(10),
            tag: Tag::new("tag"),
        };
        let portfolio = portfolio_with(market_sell, Decimal::from(15), Decimal::ZERO);
        assert_eq!(portfolio.asset_reserved(&AssetId::Bitcoin), Decimal::ZERO);
    }

    #[test]
    fn accounts_can_be_discovered_with_their_details() {
        let portfolio = portfolio_with(
            limit_sell(Quantity::OfBase(Decimal::from(10)), Decimal::from(100)),
            Decimal::from(4),
            Decimal::ZERO,
        );

        let accounts = portfolio.accounts().collect::<Vec<_>>();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].0, &CexId::Binance);
        assert_eq!(accounts[0].1, &AccountId::new("account"));

        let account = portfolio
            .cex_account(&CexId::Binance, &AccountId::new("account"))
            .unwrap();
        assert_eq!(
            account.asset_counts().get(&AssetId::Bitcoin),
            Some(&Decimal::from(100))
        );
        assert_eq!(account.asset_count(&AssetId::Bitcoin), Decimal::from(100));

        let pending_orders = account.pending_orders_with_tag(&Tag::new("tag")).unwrap();
        assert_eq!(pending_orders.len(), 1);
        assert_eq!(pending_orders[0].filled_base_quantity(), Decimal::from(4));
        assert_eq!(pending_orders[0].filled_quote_quantity(), Decimal::ZERO);
        assert!(matches!(
            pending_orders[0].order_request(),
            OrderRequest::Limit { .. }
        ));

        assert_eq!(portfolio.pending_orders_iter().count(), 1);
        assert_eq!(
            portfolio
                .pending_orders_in_cex_account_iter(&CexId::Binance, &AccountId::new("account"))
                .count(),
            1
        );
        assert_eq!(
            portfolio
                .pending_orders_in_cex_account_with_tag_iter(
                    &CexId::Binance,
                    &AccountId::new("account"),
                    &Tag::new("tag"),
                )
                .count(),
            1
        );
    }
}
