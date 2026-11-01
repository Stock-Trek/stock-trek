use crate::market_data::market_quote::{MarketQuote, PriceQuantity};
use serde::{Deserialize, Serialize};

/// The current bids and asks for a market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOrderBook {
    /// The bid quotes, ordered from highest to lowest price.
    pub bids: Vec<MarketQuote>,
    /// The ask quotes, ordered from lowest to highest price.
    pub asks: Vec<MarketQuote>,
}

impl MarketOrderBook {
    /// Creates an order book from bid and ask quotes.
    pub fn new(bids: Vec<MarketQuote>, asks: Vec<MarketQuote>) -> Self {
        Self { bids, asks }
    }
    /// Returns the bids as price/quantity pairs.
    pub fn bid_price_quantities(&self) -> Vec<PriceQuantity> {
        to_price_quantities(&self.bids)
    }
    /// Returns the asks as price/quantity pairs.
    pub fn ask_price_quantities(&self) -> Vec<PriceQuantity> {
        to_price_quantities(&self.asks)
    }
}

fn to_price_quantities(vec: &[MarketQuote]) -> Vec<PriceQuantity> {
    vec.iter().map(|quote| quote.to_price_quantity()).collect()
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
    fn price_quantities_reflect_current_quotes() {
        let mut order_book = order_book();

        assert_eq!(order_book.bid_price_quantities(), vec![(100.0, 1.5)]);
        assert_eq!(order_book.ask_price_quantities(), vec![(101.0, 2.5)]);

        order_book.bids[0].price = 200.0;
        order_book.asks[0].quantity = 3.5;

        assert_eq!(order_book.bid_price_quantities(), vec![(200.0, 1.5)]);
        assert_eq!(order_book.ask_price_quantities(), vec![(101.0, 3.5)]);
    }
}
