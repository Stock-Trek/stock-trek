use crate::market_data::timestamp::TimestampMillis;
use serde::{Deserialize, Serialize};

pub type PriceQuantity = (f64, f64);
pub type TimedPriceQuantity = (TimestampMillis, f64, f64);

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketQuote {
    pub price: f64,
    pub quantity: f64,
}

impl MarketQuote {
    pub fn to_price_quantity(&self) -> PriceQuantity {
        (self.price, self.quantity)
    }
    pub fn to_timed_price_quantity(&self, timestamp: TimestampMillis) -> TimedPriceQuantity {
        (timestamp, self.price, self.quantity)
    }
}
