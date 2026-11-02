use crate::{
    error::result::StockTrekResult,
    market_data::{timestamp::TimestampMillis, validation::Validation},
};
use serde::{Deserialize, Serialize};

/// A price and quantity pair.
pub type PriceQuantity = (f64, f64);
/// A timestamp, price, and quantity tuple.
pub type TimedPriceQuantity = (TimestampMillis, f64, f64);

/// A price and quantity quote from a market.
///
/// Prices and quantities are validated when the quote is created or
/// deserialized.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub(crate) fn validate(&self, function: &'static str) -> StockTrekResult<()> {
        Validation::positive(function, "price", self.price)?;
        Validation::positive(function, "quantity", self.quantity)
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

    /// Converts the quote to a price/quantity pair.
    pub fn to_price_quantity(&self) -> PriceQuantity {
        (self.price, self.quantity)
    }
    /// Converts the quote to a timestamped price/quantity tuple.
    pub fn to_timed_price_quantity(&self, timestamp: TimestampMillis) -> TimedPriceQuantity {
        (timestamp, self.price, self.quantity)
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
        assert_eq!(quote.to_price_quantity(), (100.0, 1.5));
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
}
