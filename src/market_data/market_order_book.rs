use crate::market_data::market_quote::{MarketQuote, PriceQuantity};
use serde::{Deserialize, Deserializer, Serialize};

/// The current bids and asks for a market.
///
/// Bids are kept from highest to lowest price and asks from lowest to highest
/// price, regardless of the order in which they are supplied.
#[derive(Debug, Clone, Serialize)]
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
    /// Returns bids as `(price, quantity)` pairs.
    pub fn bid_price_quantities(&self) -> Vec<PriceQuantity> {
        Self::to_price_quantities(&self.bids)
    }
    /// Returns asks as `(price, quantity)` pairs.
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
mod tests;
