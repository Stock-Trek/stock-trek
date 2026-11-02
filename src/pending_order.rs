use crate::{error::result::StockTrekResult, validation::Validation};
use stock_trek_types::cex::{
    activation::Activation, asset_id::AssetId, order_request::OrderRequest, quantity::Quantity,
};

/// An order with the quantities that have already been filled.
#[derive(Debug, Clone)]
pub struct PendingOrder {
    order_request: OrderRequest<AssetId, f64>,
    filled_base_quantity: f64,
    filled_quote_quantity: f64,
}

impl PendingOrder {
    /// Creates a pending order from its order request and filled quantities.
    ///
    /// Returns an error when the order request is invalid or when either filled
    /// quantity is negative or not finite.
    pub fn new(
        order_request: OrderRequest<AssetId, f64>,
        filled_base_quantity: f64,
        filled_quote_quantity: f64,
    ) -> StockTrekResult<Self> {
        Self::validate_order_request(&order_request)?;
        Validation::non_negative(
            "PendingOrder::new",
            "filled base quantity",
            filled_base_quantity,
        )?;
        Validation::non_negative(
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

    fn validate_order_request(order_request: &OrderRequest<AssetId, f64>) -> StockTrekResult<()> {
        let function = "PendingOrder::new";
        match order_request {
            OrderRequest::Limit {
                activation,
                limit_price,
                quantity,
                ..
            } => {
                Validation::positive(function, "limit price", *limit_price)?;
                match quantity {
                    Quantity::OfBase(value) => {
                        Validation::positive(function, "base quantity", *value)?;
                    }
                    Quantity::OfQuote(value) => {
                        Validation::positive(function, "quote quantity", *value)?;
                    }
                }
                Self::validate_activation(function, activation)?;
            }
            OrderRequest::MarketBuy {
                activation,
                quote_quantity,
                ..
            } => {
                Validation::positive(function, "quote quantity", *quote_quantity)?;
                Self::validate_activation(function, activation)?;
            }
            OrderRequest::MarketSell {
                activation,
                base_quantity,
                ..
            } => {
                Validation::positive(function, "base quantity", *base_quantity)?;
                Self::validate_activation(function, activation)?;
            }
        }
        Ok(())
    }

    fn validate_activation(
        function: &'static str,
        activation: &Activation<f64>,
    ) -> StockTrekResult<()> {
        match activation {
            Activation::Immediate => {}
            Activation::PriceTriggered {
                activation_price, ..
            }
            | Activation::Trailing {
                activation_price, ..
            } => {
                Validation::finite(function, "activation price", *activation_price)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stock_trek_types::cex::{
        activation::Activation, asset_id::AssetId, order_request::OrderRequest,
        price_basis::PriceBasis, side::Side, tag::Tag, time_in_force::TimeInForce,
        trigger_direction::TriggerDirection, trigger_mode::TriggerMode,
    };

    fn limit_sell(quantity: Quantity<f64>, limit_price: f64) -> OrderRequest<AssetId, f64> {
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

    fn order_request() -> OrderRequest<AssetId, f64> {
        limit_sell(Quantity::OfBase(1.0), 100.0)
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

    #[test]
    fn rejects_non_positive_limit_prices() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(PendingOrder::new(limit_sell(Quantity::OfBase(1.0), value), 0.0, 0.0).is_err());
            assert!(
                PendingOrder::new(limit_sell(Quantity::OfQuote(100.0), value), 0.0, 0.0).is_err()
            );
        }
    }

    #[test]
    fn rejects_non_positive_limit_quantities() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(
                PendingOrder::new(limit_sell(Quantity::OfBase(value), 100.0), 0.0, 0.0).is_err()
            );
            assert!(
                PendingOrder::new(limit_sell(Quantity::OfQuote(value), 100.0), 0.0, 0.0).is_err()
            );
        }
    }

    #[test]
    fn rejects_non_positive_market_order_quantities() {
        for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let market_buy = OrderRequest::MarketBuy {
                base: AssetId::Bitcoin,
                quote: AssetId::TetherUSD,
                activation: Activation::Immediate,
                quote_quantity: value,
                tag: Tag::new("tag"),
            };
            assert!(PendingOrder::new(market_buy, 0.0, 0.0).is_err());

            let market_sell = OrderRequest::MarketSell {
                base: AssetId::Bitcoin,
                quote: AssetId::TetherUSD,
                activation: Activation::Immediate,
                base_quantity: value,
                tag: Tag::new("tag"),
            };
            assert!(PendingOrder::new(market_sell, 0.0, 0.0).is_err());
        }
    }

    #[test]
    fn rejects_non_finite_activation_prices() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let price_triggered = OrderRequest::Limit {
                base: AssetId::Bitcoin,
                quote: AssetId::TetherUSD,
                side: Side::Sell,
                activation: Activation::PriceTriggered {
                    activation_price: value,
                    basis: PriceBasis::Last,
                    direction: TriggerDirection::Above,
                    mode: TriggerMode::Touch,
                },
                limit_price: 100.0,
                time_in_force: TimeInForce::GoodTillCancelled,
                quantity: Quantity::OfBase(1.0),
                tag: Tag::new("tag"),
            };
            assert!(PendingOrder::new(price_triggered, 0.0, 0.0).is_err());

            let trailing = OrderRequest::Limit {
                base: AssetId::Bitcoin,
                quote: AssetId::TetherUSD,
                side: Side::Sell,
                activation: Activation::Trailing {
                    activation_price: value,
                    basis: PriceBasis::Last,
                    callback_rate_bps: 100,
                    direction: TriggerDirection::Below,
                },
                limit_price: 100.0,
                time_in_force: TimeInForce::GoodTillCancelled,
                quantity: Quantity::OfBase(1.0),
                tag: Tag::new("tag"),
            };
            assert!(PendingOrder::new(trailing, 0.0, 0.0).is_err());
        }
    }
}
