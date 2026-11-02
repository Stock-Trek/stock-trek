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
mod tests {
    use super::*;

    #[test]
    fn accepts_a_finite_positive_quote() {
        let quote = MarketQuote::new(100.0, 1.5).unwrap();

        assert_eq!(quote.price(), 100.0);
        assert_eq!(quote.quantity(), 1.5);
    }

    #[test]
    fn rejects_non_finite_prices_and_quantities() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(MarketQuote::new(value, 1.0).is_err());
            assert!(MarketQuote::new(1.0, value).is_err());
        }
    }

    #[test]
    fn rejects_zero_and_negative_prices_and_quantities() {
        for value in [0.0, -1.0] {
            assert!(MarketQuote::new(value, 1.0).is_err());
            assert!(MarketQuote::new(1.0, value).is_err());
        }
    }

    #[test]
    fn serde_json_round_trip_preserves_the_quote() {
        let quote = MarketQuote::new(100.0, 1.5).unwrap();
        let json = serde_json::to_string(&quote).unwrap();
        let deserialized: MarketQuote = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.price(), 100.0);
        assert_eq!(deserialized.quantity(), 1.5);
    }

    #[test]
    fn deserialization_rejects_invalid_quotes() {
        for (price, quantity) in [(0.0, 1.0), (1.0, 0.0), (-1.0, 1.0), (1.0, -1.0)] {
            let json = serde_json::json!({
                "price": price,
                "quantity": quantity,
            });

            assert!(serde_json::from_value::<MarketQuote>(json).is_err());
        }
    }

    #[test]
    fn timed_price_quantity_exposes_named_values() {
        let timed = TimedPriceQuantity::new(TimestampMillis::new(1), 100.0, 1.5).unwrap();

        assert_eq!(timed.timestamp_millis(), TimestampMillis::new(1));
        assert_eq!(timed.price(), 100.0);
        assert_eq!(timed.quantity(), 1.5);
    }

    #[test]
    fn timed_price_quantity_rejects_invalid_quotes() {
        assert!(TimedPriceQuantity::new(TimestampMillis::new(1), 0.0, 1.0).is_err());
    }

    #[test]
    fn timed_price_quantity_serde_round_trip() {
        let timed = TimedPriceQuantity::new(TimestampMillis::new(1), 100.0, 1.5).unwrap();
        let json = serde_json::to_string(&timed).unwrap();
        let deserialized: TimedPriceQuantity = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, timed);
    }

    #[test]
    fn timed_price_quantity_deserialization_rejects_invalid_quotes() {
        let json = serde_json::json!({
            "timestamp_millis": 1,
            "price": 0.0,
            "quantity": 1.5,
        });

        assert!(serde_json::from_value::<TimedPriceQuantity>(json).is_err());
    }
}
