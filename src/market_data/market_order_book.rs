use crate::{
    error::result::{StockTrekError, StockTrekResult},
    market_data::market_quote::MarketQuote,
    validation::Validation,
};
use serde::{Deserialize, Serialize};

/// The current bids and asks for a market.
///
/// Bids are kept from highest to lowest price and asks from lowest to highest
/// price, regardless of the order in which they are supplied. An order book is
/// validated against crossed bids and asks when it is created or when it is
/// deserialized.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MarketOrderBookData")]
pub struct MarketOrderBook {
    bids: Vec<MarketQuote>,
    asks: Vec<MarketQuote>,
}

impl MarketOrderBook {
    /// Creates an order book from bid and ask quotes, rejecting crossed bids
    /// and asks.
    ///
    /// Bids are sorted from highest to lowest price and asks from lowest to
    /// highest price.
    pub fn new(bids: Vec<MarketQuote>, asks: Vec<MarketQuote>) -> StockTrekResult<Self> {
        let mut bids = bids;
        let mut asks = asks;
        bids.sort_by(|a, b| b.price().total_cmp(&a.price()));
        asks.sort_by(|a, b| a.price().total_cmp(&b.price()));
        if let (Some(highest_bid), Some(lowest_ask)) = (bids.first(), asks.first()) {
            Validation::bid_below_ask(
                "MarketOrderBook::new",
                highest_bid.price(),
                lowest_ask.price(),
            )?;
        }
        Ok(Self { bids, asks })
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

impl TryFrom<MarketOrderBookData> for MarketOrderBook {
    type Error = StockTrekError;

    fn try_from(data: MarketOrderBookData) -> Result<Self, Self::Error> {
        Self::new(data.bids, data.asks)
    }
}
