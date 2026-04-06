use rust_decimal::Decimal;

#[derive(Debug)]
pub struct MarketQuote {
    price: Decimal,
    quantity: Decimal,
}

impl MarketQuote {
    pub fn new(price: Decimal, quantity: Decimal) -> Self {
        Self { price, quantity }
    }
    pub fn price(&self) -> Decimal {
        self.price
    }
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}
