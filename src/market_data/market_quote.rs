use crate::market_data::timestamp::TimestampMillis;
use serde::{Deserialize, Serialize};

/// A price and quantity pair.
pub type PriceQuantity = (f64, f64);
/// A timestamp, price, and quantity tuple.
pub type TimedPriceQuantity = (TimestampMillis, f64, f64);

/// A price and quantity quote from a market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketQuote {
    /// The quoted price.
    pub price: f64,
    /// The quoted quantity.
    pub quantity: f64,
}

impl MarketQuote {
    /// Converts the quote to a price/quantity pair.
    pub fn to_price_quantity(&self) -> PriceQuantity {
        (self.price, self.quantity)
    }
    /// Converts the quote to a timestamped price/quantity tuple.
    pub fn to_timed_price_quantity(&self, timestamp: TimestampMillis) -> TimedPriceQuantity {
        (timestamp, self.price, self.quantity)
    }
}
