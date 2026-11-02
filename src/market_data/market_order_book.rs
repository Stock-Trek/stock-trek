use crate::market_data::market_quote::MarketQuote;
use serde::{Deserialize, Serialize};

/// The current bids and asks for a market.
///
/// Bids are kept from highest to lowest price and asks from lowest to highest
/// price, regardless of the order in which they are supplied.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "MarketOrderBookData")]
pub struct MarketOrderBook {
    bids: Vec<MarketQuote>,
    asks: Vec<MarketQuote>,
}

impl MarketOrderBook {
    /// Creates an order book from bid and ask quotes.
    ///
    /// Bids are sorted from highest to lowest price and asks from lowest to
    /// highest price.
    pub fn new(mut bids: Vec<MarketQuote>, mut asks: Vec<MarketQuote>) -> Self {
        bids.sort_by(|a, b| b.price().total_cmp(&a.price()));
        asks.sort_by(|a, b| a.price().total_cmp(&b.price()));
        Self { bids, asks }
    }

    /// Returns bids ordered from highest to lowest price.
    pub fn bids(&self) -> &[MarketQuote] {
        &self.bids
    }
    /// Returns asks ordered from lowest to highest price.
    pub fn asks(&self) -> &[MarketQuote] {
        &self.asks
    }
}

#[derive(Deserialize)]
struct MarketOrderBookData {
    bids: Vec<MarketQuote>,
    asks: Vec<MarketQuote>,
}

impl From<MarketOrderBookData> for MarketOrderBook {
    fn from(data: MarketOrderBookData) -> Self {
        Self::new(data.bids, data.asks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order_book() -> MarketOrderBook {
        MarketOrderBook::new(
            vec![MarketQuote::new(100.0, 1.5).unwrap()],
            vec![MarketQuote::new(101.0, 2.5).unwrap()],
        )
    }

    fn quote(price: f64, quantity: f64) -> MarketQuote {
        MarketQuote::new(price, quantity).unwrap()
    }

    #[test]
    fn bids_and_asks_are_sorted_by_price() {
        let order_book = MarketOrderBook::new(
            vec![
                MarketQuote::new(99.0, 1.0).unwrap(),
                MarketQuote::new(101.0, 2.0).unwrap(),
                MarketQuote::new(100.0, 3.0).unwrap(),
            ],
            vec![
                MarketQuote::new(103.0, 1.0).unwrap(),
                MarketQuote::new(102.0, 2.0).unwrap(),
                MarketQuote::new(104.0, 3.0).unwrap(),
            ],
        );

        assert_eq!(
            order_book.bids(),
            [quote(101.0, 2.0), quote(100.0, 3.0), quote(99.0, 1.0)]
        );
        assert_eq!(
            order_book.asks(),
            [quote(102.0, 2.0), quote(103.0, 1.0), quote(104.0, 3.0)]
        );
    }

    #[test]
    fn serde_json_round_trip() {
        let order_book = order_book();
        let json = serde_json::to_string(&order_book).unwrap();
        let deserialized: MarketOrderBook = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.bids, [quote(100.0, 1.5)]);
        assert_eq!(deserialized.asks, [quote(101.0, 2.5)]);
    }

    #[test]
    fn deserialization_sorts_bids_and_asks() {
        let json = r#"{
            "bids": [{"price": 99.0, "quantity": 1.0}, {"price": 101.0, "quantity": 2.0}],
            "asks": [{"price": 103.0, "quantity": 1.0}, {"price": 102.0, "quantity": 2.0}]
        }"#;
        let deserialized: MarketOrderBook = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized.bids(), [quote(101.0, 2.0), quote(99.0, 1.0)]);
        assert_eq!(deserialized.asks(), [quote(102.0, 2.0), quote(103.0, 1.0)]);
    }

    #[test]
    fn deserialization_rejects_invalid_quotes() {
        let json = r#"{
            "bids": [{"price": 0.0, "quantity": 1.0}],
            "asks": [{"price": 102.0, "quantity": -1.0}]
        }"#;

        assert!(serde_json::from_str::<MarketOrderBook>(json).is_err());
    }
}
