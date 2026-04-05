use rust_decimal::Decimal;

use crate::dto::raw_market_quote::RawMarketQuote;

#[derive(Debug)]
pub struct MarketQuote {
    price: Decimal,
    quantity: Decimal,
}

impl MarketQuote {
    pub fn price(&self) -> Decimal {
        self.price
    }
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}

impl From<RawMarketQuote> for MarketQuote {
    fn from(value: RawMarketQuote) -> Self {
        let RawMarketQuote { price, quantity } = value;
        MarketQuote {
            price: Decimal::from(price),
            quantity: Decimal::from(quantity),
        }
    }
}
