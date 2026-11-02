use crate::{error::result::StockTrekResult, validation::Validation};
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
    /// Creates a pending order from its order request and filled quantities.
    ///
    /// Returns an error when either filled quantity is negative or not finite.
    pub fn new(
        order_request: OrderRequest<AssetId, f64>,
        filled_base_quantity: f64,
        filled_quote_quantity: f64,
    ) -> StockTrekResult<Self> {
        Validation::Stats.non_negative(
            "PendingOrder::new",
            "filled base quantity",
            filled_base_quantity,
        )?;
        Validation::Stats.non_negative(
            "PendingOrder::new",
            "filled quote quantity",
            filled_quote_quantity,
        )?;
        Ok(Self {
            order_request,
            filled_base_quantity,
            filled_quote_quantity,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use stock_trek_types::cex::{
        activation::Activation, asset_id::AssetId, order_request::OrderRequest, side::Side,
        tag::Tag, time_in_force::TimeInForce,
    };

    fn order_request() -> OrderRequest<AssetId, f64> {
        OrderRequest::Limit {
            base: AssetId::Bitcoin,
            quote: AssetId::TetherUSD,
            side: Side::Sell,
            activation: Activation::Immediate,
            limit_price: 100.0,
            time_in_force: TimeInForce::GoodTillCancelled,
            quantity: stock_trek_types::cex::quantity::Quantity::OfBase(1.0),
            tag: Tag::new("tag"),
        }
    }

    #[test]
    fn accepts_non_negative_filled_quantities() {
        let order = PendingOrder::new(order_request(), 0.5, 50.0).unwrap();

        assert_eq!(order.filled_base_quantity(), 0.5);
        assert_eq!(order.filled_quote_quantity(), 50.0);
    }

    #[test]
    fn rejects_negative_filled_quantities() {
        assert!(PendingOrder::new(order_request(), -0.1, 0.0).is_err());
        assert!(PendingOrder::new(order_request(), 0.0, -0.1).is_err());
    }

    #[test]
    fn rejects_non_finite_filled_quantities() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(PendingOrder::new(order_request(), value, 0.0).is_err());
            assert!(PendingOrder::new(order_request(), 0.0, value).is_err());
        }
    }
}
