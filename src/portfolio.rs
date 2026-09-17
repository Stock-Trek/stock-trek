use rust_decimal::Decimal;
use std::collections::HashMap;
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest,
    quantity::Quantity, side::Side, tag::Tag,
};

#[derive(Debug, Clone)]
pub struct Portfolio {
    portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>,
}

#[derive(Debug, Clone)]
pub struct CexAccountPortfolio {
    asset_counts: HashMap<AssetId, Decimal>,
    pending_orders: HashMap<Tag, Vec<PendingOrder>>,
}

#[derive(Debug, Clone)]
pub struct PendingOrder {
    order_request: OrderRequest<AssetId, Decimal>,
    filled_base_quantity: Decimal,
    filled_quote_quantity: Decimal,
}

impl Portfolio {
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
        match &pending_order.order_request {
            OrderRequest::Limit {
                base,
                quote,
                side,
                limit_price,
                quantity,
                ..
            } if asset_id == base && Side::Sell == *side => {
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
                base_quantity - pending_order.filled_base_quantity
            }
            OrderRequest::Limit {
                base,
                quote,
                side,
                limit_price,
                quantity,
                ..
            } if asset_id == quote && Side::Buy == *side => {
                let quote_quantity = match quantity {
                    Quantity::OfBase(base_quantity) => base_quantity * limit_price,
                    Quantity::OfQuote(quote_quantity) => *quote_quantity,
                };
                quote_quantity - pending_order.filled_quote_quantity
            }
            OrderRequest::MarketBuy {
                quote,
                quote_quantity,
                ..
            } if asset_id == quote => *quote_quantity - pending_order.filled_quote_quantity,
            OrderRequest::MarketSell {
                base,
                base_quantity,
                ..
            } if asset_id == base => *base_quantity - pending_order.filled_base_quantity,
            _ => Decimal::ZERO,
        }
    }
}

impl CexAccountPortfolio {
    pub fn new(
        asset_counts: HashMap<AssetId, Decimal>,
        pending_orders: HashMap<Tag, Vec<PendingOrder>>,
    ) -> Self {
        Self {
            asset_counts,
            pending_orders,
        }
    }
}

impl PendingOrder {
    pub fn new(
        order_request: OrderRequest<AssetId, Decimal>,
        filled_base_quantity: Decimal,
        filled_quote_quantity: Decimal,
    ) -> Self {
        Self {
            order_request,
            filled_base_quantity,
            filled_quote_quantity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PortfolioBuilder {
    portfolios: HashMap<CexId, HashMap<AccountId, CexAccountPortfolio>>,
}

impl PortfolioBuilder {
    pub fn new() -> Self {
        Self {
            portfolios: HashMap::new(),
        }
    }
}

impl Default for PortfolioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PortfolioBuilder {
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
            .or_insert_with(|| CexAccountPortfolio {
                asset_counts: HashMap::new(),
                pending_orders: HashMap::new(),
            })
            .asset_counts
            .insert(asset_id, count);
        self
    }

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
            .or_insert_with(|| CexAccountPortfolio {
                asset_counts: HashMap::new(),
                pending_orders: HashMap::new(),
            })
            .pending_orders
            .entry(tag)
            .or_default()
            .push(PendingOrder {
                order_request,
                filled_base_quantity,
                filled_quote_quantity,
            });
        self
    }

    pub fn build(&self) -> Portfolio {
        Portfolio {
            portfolios: self.portfolios.clone(),
        }
    }
}
