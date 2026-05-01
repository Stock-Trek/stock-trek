use crate::market_data::market_quote::{MarketQuote, PriceQuantity};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Serialize)]
pub struct MarketOrderBook {
    pub bids: Vec<MarketQuote>,
    pub asks: Vec<MarketQuote>,
    #[serde(skip)]
    bid_price_quantities: OnceLock<Vec<PriceQuantity>>,
    #[serde(skip)]
    ask_price_quantities: OnceLock<Vec<PriceQuantity>>,
}

impl<'de> Deserialize<'de> for MarketOrderBook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            exact_bids: Vec<MarketQuote>,
            exact_asks: Vec<MarketQuote>,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(MarketOrderBook::new(helper.exact_bids, helper.exact_asks))
    }
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
