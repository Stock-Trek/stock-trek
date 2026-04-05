use rust_decimal::Decimal;

use crate::dto::raw_market_ohlcv::RawMarketOhlcv;

#[derive(Debug, Clone)]
pub struct MarketOhlcv {
    exact_open: Decimal,
    exact_high: Decimal,
    exact_low: Decimal,
    exact_close: Decimal,
    exact_volume: Decimal,
    exact_quote_volume: Decimal,
    exact_vwap: Decimal,
}

impl MarketOhlcv {
    pub fn exact_open(&self) -> Decimal {
        self.exact_open
    }
    pub fn exact_high(&self) -> Decimal {
        self.exact_high
    }
    pub fn exact_low(&self) -> Decimal {
        self.exact_low
    }
    pub fn exact_close(&self) -> Decimal {
        self.exact_close
    }
    pub fn exact_volume(&self) -> Decimal {
        self.exact_volume
    }
    pub fn exact_quote_volume(&self) -> Decimal {
        self.exact_quote_volume
    }
    pub fn exact_vwap(&self) -> Decimal {
        self.exact_vwap
    }
}

impl From<RawMarketOhlcv> for MarketOhlcv {
    fn from(value: RawMarketOhlcv) -> Self {
        let RawMarketOhlcv {
            open,
            high,
            low,
            close,
            volume,
            quote_volume,
            vwap,
        } = value;
        MarketOhlcv {
            exact_open: Decimal::from(open),
            exact_high: Decimal::from(high),
            exact_low: Decimal::from(low),
            exact_close: Decimal::from(close),
            exact_volume: Decimal::from(volume),
            exact_quote_volume: Decimal::from(quote_volume),
            exact_vwap: Decimal::from(vwap),
        }
    }
}
