use rust_decimal::Decimal;
use stock_trek_types::cex::{asset_id::AssetId, order_request::OrderRequest};

/// An order with the quantities that have already been filled.
#[derive(Debug, Clone)]
pub struct PendingOrder {
    order_request: OrderRequest<AssetId, Decimal>,
    filled_base_quantity: Decimal,
    filled_quote_quantity: Decimal,
}

impl PendingOrder {
    /// Creates a pending order with its filled base and quote quantities.
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

    /// Returns the order request that created this pending order.
    pub fn order_request(&self) -> &OrderRequest<AssetId, Decimal> {
        &self.order_request
    }

    /// Returns the quantity of the base asset that has been filled.
    pub fn filled_base_quantity(&self) -> Decimal {
        self.filled_base_quantity
    }

    /// Returns the quantity of the quote asset that has been filled.
    pub fn filled_quote_quantity(&self) -> Decimal {
        self.filled_quote_quantity
    }
}
