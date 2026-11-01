use crate::market_data::market_quote::{MarketQuote, PriceQuantity};
use serde::{Deserialize, Deserializer, Serialize};

/// The current bids and asks for a market.
#[derive(Debug, Clone, Serialize)]
pub struct MarketOrderBook {
    /// The bid quotes, ordered from highest to lowest price.
    pub bids: Vec<MarketQuote>,
    /// The ask quotes, ordered from lowest to highest price.
    pub asks: Vec<MarketQuote>,
}

impl MarketOrderBook {
    /// Creates an order book from bid and ask quotes.
    ///
    /// Bids are sorted from highest to lowest price and asks from lowest to
    /// highest price.
    pub fn new(mut bids: Vec<MarketQuote>, mut asks: Vec<MarketQuote>) -> Self {
        bids.sort_by(|a, b| b.price.total_cmp(&a.price));
        asks.sort_by(|a, b| a.price.total_cmp(&b.price));
        Self { bids, asks }
    }
    /// Returns the bids as price/quantity pairs.
    pub fn bid_price_quantities(&self) -> Vec<PriceQuantity> {
        Self::to_price_quantities(&self.bids)
    }
    /// Returns the asks as price/quantity pairs.
    pub fn ask_price_quantities(&self) -> Vec<PriceQuantity> {
        Self::to_price_quantities(&self.asks)
    }

    fn to_price_quantities(vec: &[MarketQuote]) -> Vec<PriceQuantity> {
        vec.iter().map(|quote| quote.to_price_quantity()).collect()
    }
}

impl<'de> Deserialize<'de> for MarketOrderBook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MarketOrderBookData {
            bids: Vec<MarketQuote>,
            asks: Vec<MarketQuote>,
        }

        let data = MarketOrderBookData::deserialize(deserializer)?;
        Ok(Self::new(data.bids, data.asks))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order_book() -> MarketOrderBook {
        MarketOrderBook::new(
            vec![MarketQuote {
                price: 100.0,
                quantity: 1.5,
            }],
            vec![MarketQuote {
                price: 101.0,
                quantity: 2.5,
            }],
        )
    }

    #[test]
    fn bids_and_asks_are_sorted_by_price() {
        let order_book = MarketOrderBook::new(
            vec![
                MarketQuote {
                    price: 99.0,
                    quantity: 1.0,
                },
                MarketQuote {
                    price: 101.0,
                    quantity: 2.0,
                },
                MarketQuote {
                    price: 100.0,
                    quantity: 3.0,
                },
            ],
            vec![
                MarketQuote {
                    price: 103.0,
                    quantity: 1.0,
                },
                MarketQuote {
                    price: 102.0,
                    quantity: 2.0,
                },
                MarketQuote {
                    price: 104.0,
                    quantity: 3.0,
                },
            ],
        );

        assert_eq!(
            order_book.bid_price_quantities(),
            vec![(101.0, 2.0), (100.0, 3.0), (99.0, 1.0)]
        );
        assert_eq!(
            order_book.ask_price_quantities(),
            vec![(102.0, 2.0), (103.0, 1.0), (104.0, 3.0)]
        );
    }

    #[test]
    fn serde_json_round_trip() {
        let order_book = order_book();
        let json = serde_json::to_string(&order_book).unwrap();
        let deserialized: MarketOrderBook = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.bids.len(), 1);
        assert_eq!(deserialized.bids[0].price, 100.0);
        assert_eq!(deserialized.bids[0].quantity, 1.5);
        assert_eq!(deserialized.asks.len(), 1);
        assert_eq!(deserialized.asks[0].price, 101.0);
        assert_eq!(deserialized.asks[0].quantity, 2.5);
        assert_eq!(deserialized.bid_price_quantities(), vec![(100.0, 1.5)]);
        assert_eq!(deserialized.ask_price_quantities(), vec![(101.0, 2.5)]);
    }

    #[test]
    fn deserialization_sorts_bids_and_asks() {
        let json = r#"{
            "bids": [{"price": 99.0, "quantity": 1.0}, {"price": 101.0, "quantity": 2.0}],
            "asks": [{"price": 103.0, "quantity": 1.0}, {"price": 102.0, "quantity": 2.0}]
        }"#;
        let deserialized: MarketOrderBook = serde_json::from_str(json).unwrap();

        assert_eq!(
            deserialized.bid_price_quantities(),
            vec![(101.0, 2.0), (99.0, 1.0)]
        );
        assert_eq!(
            deserialized.ask_price_quantities(),
            vec![(102.0, 2.0), (103.0, 1.0)]
        );
    }
}
