use crate::dto::raw_decimal::RawDecimal;

#[derive(Debug)]
pub struct RawMarketQuote {
    pub price: RawDecimal,
    pub quantity: RawDecimal,
}
