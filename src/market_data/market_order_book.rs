use crate::market_data::market_quote::{MarketQuote, PriceQuantity};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOrderBook {
    #[serde(alias = "exact_bids")]
    pub bids: Vec<MarketQuote>,
    #[serde(alias = "exact_asks")]
    pub asks: Vec<MarketQuote>,
    #[serde(skip)]
    bid_price_quantities: OnceLock<Vec<PriceQuantity>>,
    #[serde(skip)]
    ask_price_quantities: OnceLock<Vec<PriceQuantity>>,
}

impl MarketOrderBook {
    pub fn new(bids: Vec<MarketQuote>, asks: Vec<MarketQuote>) -> Self {
        Self {
            bids,
            asks,
            bid_price_quantities: OnceLock::new(),
            ask_price_quantities: OnceLock::new(),
        }
    }
    pub fn bid_price_quantities(&self) -> &Vec<PriceQuantity> {
        self.bid_price_quantities
            .get_or_init(|| to_price_quantities(&self.bids))
    }
    pub fn ask_price_quantities(&self) -> &Vec<PriceQuantity> {
        self.ask_price_quantities
            .get_or_init(|| to_price_quantities(&self.asks))
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
        assert_eq!(deserialized.bid_price_quantities(), &vec![(100.0, 1.5)]);
        assert_eq!(deserialized.ask_price_quantities(), &vec![(101.0, 2.5)]);
    }

    #[test]
    fn deserializes_legacy_exact_field_names() {
        let json = r#"{
            "exact_bids": [{"price": 100.0, "quantity": 1.5}],
            "exact_asks": [{"price": 101.0, "quantity": 2.5}]
        }"#;
        let deserialized: MarketOrderBook = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized.bids[0].price, 100.0);
        assert_eq!(deserialized.asks[0].price, 101.0);
    }
}
