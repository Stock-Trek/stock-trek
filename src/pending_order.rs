use stock_trek_types::cex::{asset_id::AssetId, order_request::OrderRequest};

/// An order with the quantities that have already been filled.
#[derive(Debug, Clone)]
pub struct PendingOrder {
    order_request: OrderRequest<AssetId, f64>,
    filled_base_quantity: f64,
    filled_quote_quantity: f64,
}

#[allow(missing_docs)]
impl PendingOrder {
    pub fn new(
        order_request: OrderRequest<AssetId, f64>,
        filled_base_quantity: f64,
        filled_quote_quantity: f64,
    ) -> Self {
        Self {
            order_request,
            filled_base_quantity,
            filled_quote_quantity,
        }
    }

    /// Returns the order request that created this pending order.
    pub fn order_request(&self) -> &OrderRequest<AssetId, f64> {
        &self.order_request
    }

    /// Returns the quantity of the base asset that has been filled.
    pub fn filled_base_quantity(&self) -> f64 {
        self.filled_base_quantity
    }

    /// Returns the quantity of the quote asset that has been filled.
    pub fn filled_quote_quantity(&self) -> f64 {
        self.filled_quote_quantity
    }
}
