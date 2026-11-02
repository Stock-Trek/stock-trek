use crate::market_data::timestamp::TimestampMillis;
use serde::{Deserialize, Serialize};

/// A price and quantity pair.
pub type PriceQuantity = (f64, f64);
/// A timestamp, price, and quantity tuple.
pub type TimedPriceQuantity = (TimestampMillis, f64, f64);

/// A price and quantity quote from a market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketQuote {
    price: f64,
    quantity: f64,
}

impl MarketQuote {
    /// Creates a quote from its price and quantity.
    pub fn new(price: f64, quantity: f64) -> Self {
        Self { price, quantity }
    }

    /// Returns the quoted price.
    pub fn price(&self) -> f64 {
        self.price
    }

    /// Returns the quoted quantity.
    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    /// Converts the quote to a price/quantity pair.
    pub fn to_price_quantity(&self) -> PriceQuantity {
        (self.price, self.quantity)
    }
    /// Converts the quote to a timestamped price/quantity tuple.
    pub fn to_timed_price_quantity(&self, timestamp: TimestampMillis) -> TimedPriceQuantity {
        (timestamp, self.price, self.quantity)
    }
}
