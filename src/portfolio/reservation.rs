use super::Portfolio;
use crate::pending_order::PendingOrder;
use stock_trek_types::cex::{
    account_id::AccountId, asset_id::AssetId, cex_id::CexId, order_request::OrderRequest,
    quantity::Quantity, side::Side,
};

impl Portfolio {
    /// Total available (free) amount of `asset_id` across all accounts.
    ///
    /// Availability is calculated and clamped to zero per account before being
    /// summed, so an over-reserved account cannot reduce the free funds
    /// reported for other accounts.
    pub fn asset_available(&self, asset_id: &AssetId) -> f64 {
        self.cex_accounts()
            .map(|(cex_id, account_id, _)| {
                self.asset_available_in_cex_account(cex_id, account_id, asset_id)
            })
            .sum()
    }
    /// Available (free) amount of `asset_id` in a specific CEX account (total - reserved).
    ///
    /// The result is clamped to zero when pending orders reserve more than the
    /// account holds.
    pub fn asset_available_in_cex_account(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
        asset_id: &AssetId,
    ) -> f64 {
        Self::available_quantity(
            self.asset_total_in_cex_account(cex_id, account_id, asset_id),
            self.asset_reserved_in_cex_account(cex_id, account_id, asset_id),
        )
    }
    /// Total amount of `asset_id` reserved (locked in unfilled pending orders) across all accounts.
    pub fn asset_reserved(&self, asset_id: &AssetId) -> f64 {
        self.pending_orders_iter()
            .map(|(_, _, _, pending_order)| {
                self.reserved_quantity_for_pending_order(pending_order, asset_id)
            })
            .sum()
    }
    /// Amount of `asset_id` reserved (locked in unfilled pending orders) in a specific CEX account.
    pub fn asset_reserved_in_cex_account(
        &self,
        cex_id: &CexId,
        account_id: &AccountId,
        asset_id: &AssetId,
    ) -> f64 {
        self.pending_orders_in_cex_account_iter(cex_id, account_id)
            .map(|(_, pending_order)| {
                self.reserved_quantity_for_pending_order(pending_order, asset_id)
            })
            .sum()
    }

    fn reserved_quantity_for_pending_order(
        &self,
        pending_order: &PendingOrder,
        asset_id: &AssetId,
    ) -> f64 {
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
                    Quantity::OfQuote(quote_quantity) => *quote_quantity / limit_price,
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
            _ => 0.0,
        }
    }

    fn remaining_quantity(ordered: f64, filled: f64) -> f64 {
        (ordered - filled).max(0.0)
    }

    fn available_quantity(total: f64, reserved: f64) -> f64 {
        (total - reserved).max(0.0)
    }
}
