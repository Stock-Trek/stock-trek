use crate::{error::result::StockTrekResult, validation::Validation};
use stock_trek_types::cex::{
    activation::Activation, asset_id::AssetId, order_request::OrderRequest, quantity::Quantity,
    tag::Tag,
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
    /// Returns an error when the order request is invalid, when either filled
    /// quantity is negative or not finite, or when an amount the order reserves
    /// (such as the notional of a limit buy) would not be finite.
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

    pub(crate) fn tag(&self) -> &Tag {
        match &self.order_request {
            OrderRequest::Limit { tag, .. }
            | OrderRequest::MarketBuy { tag, .. }
            | OrderRequest::MarketSell { tag, .. } => tag,
        }
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
                        let reserved_quote_quantity = value * limit_price;
                        Validation::finite(
                            function,
                            "reserved quote quantity",
                            reserved_quote_quantity,
                        )?;
                    }
                    Quantity::OfQuote(value) => {
                        Validation::positive(function, "quote quantity", *value)?;
                        let reserved_base_quantity = value / limit_price;
                        Validation::finite(
                            function,
                            "reserved base quantity",
                            reserved_base_quantity,
                        )?;
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
                Validation::positive(function, "activation price", *activation_price)?;
            }
        }
        Ok(())
    }
}
