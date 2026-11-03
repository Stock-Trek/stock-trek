use crate::{
    error::result::StockTrekResult, market_data::timestamp::TimestampMillis, validation::Validation,
};
use serde::{Deserialize, Serialize};

/// A price and quantity quote from a market.
///
/// Prices and quantities are validated when the quote is created or
/// deserialized.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "MarketQuoteData")]
pub struct MarketQuote {
    price: f64,
    quantity: f64,
}

impl MarketQuote {
    /// Creates a quote from its price and quantity.
    ///
    /// Returns an error unless both values are finite and positive.
    pub fn new(price: f64, quantity: f64) -> StockTrekResult<Self> {
        Self::from_parts("MarketQuote::new", price, quantity)
    }

    fn from_parts(function: &'static str, price: f64, quantity: f64) -> StockTrekResult<Self> {
        Validation::positive(function, "price", price)?;
        Validation::positive(function, "quantity", quantity)?;
        Ok(Self { price, quantity })
    }

    /// Returns the quoted price.
    pub fn price(&self) -> f64 {
        self.price
    }

    /// Returns the quoted quantity.
    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    /// Converts the quote to a timestamped price and quantity pair.
    pub fn to_timed_price_quantity(&self, timestamp: TimestampMillis) -> TimedPriceQuantity {
        TimedPriceQuantity::from_quote(timestamp, self.clone())
    }
}

/// A timestamp and price/quantity pair.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TimedPriceQuantityData")]
pub struct TimedPriceQuantity {
    timestamp_millis: TimestampMillis,
    price: f64,
    quantity: f64,
}

impl TimedPriceQuantity {
    /// Creates a timestamped price and quantity pair.
    ///
    /// Returns an error unless both values are finite and positive.
    pub fn new(
        timestamp_millis: TimestampMillis,
        price: f64,
        quantity: f64,
    ) -> StockTrekResult<Self> {
        let quote = MarketQuote::new(price, quantity)?;
        Ok(Self::from_quote(timestamp_millis, quote))
    }

    /// Creates a timestamped price and quantity pair from an existing quote.
    pub fn from_quote(timestamp_millis: TimestampMillis, quote: MarketQuote) -> Self {
        Self {
            timestamp_millis,
            price: quote.price(),
            quantity: quote.quantity(),
        }
    }

    /// Returns the timestamp in milliseconds since the Unix epoch.
    pub fn timestamp_millis(&self) -> TimestampMillis {
        self.timestamp_millis
    }

    /// Returns the quoted price.
    pub fn price(&self) -> f64 {
        self.price
    }

    /// Returns the quoted quantity.
    pub fn quantity(&self) -> f64 {
        self.quantity
    }
}

#[derive(Deserialize)]
struct TimedPriceQuantityData {
    timestamp_millis: TimestampMillis,
    price: f64,
    quantity: f64,
}

impl TryFrom<TimedPriceQuantityData> for TimedPriceQuantity {
    type Error = crate::error::result::StockTrekError;

    fn try_from(data: TimedPriceQuantityData) -> Result<Self, Self::Error> {
        Self::new(data.timestamp_millis, data.price, data.quantity)
    }
}

#[derive(Deserialize)]
struct MarketQuoteData {
    price: f64,
    quantity: f64,
}

impl TryFrom<MarketQuoteData> for MarketQuote {
    type Error = crate::error::result::StockTrekError;

    fn try_from(data: MarketQuoteData) -> Result<Self, Self::Error> {
        Self::new(data.price, data.quantity)
    }
}

#[cfg(test)]
#[path = "market_quote_tests.rs"]
mod tests;
