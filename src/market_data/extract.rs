use crate::market_data::market_quote::MarketQuote;
use rust_decimal::{prelude::ToPrimitive, Decimal};

pub fn dec_to_f64(dec: Decimal) -> f64 {
    dec.to_f64().unwrap()
}

pub fn vec_dec_to_f64(vec: &[Decimal]) -> Vec<f64> {
    vec.iter().map(|dec| dec_to_f64(*dec)).collect()
}

pub fn quote_to_f64(quote: &MarketQuote) -> (f64, f64) {
    (dec_to_f64(quote.price()), dec_to_f64(quote.quantity()))
}

pub fn vec_quote_to_f64(vec: &[MarketQuote]) -> Vec<(f64, f64)> {
    vec.iter().map(|quote| quote_to_f64(quote)).collect()
}
