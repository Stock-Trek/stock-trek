use rust_decimal::Decimal;

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
    pub fn new(
        exact_open: Decimal,
        exact_high: Decimal,
        exact_low: Decimal,
        exact_close: Decimal,
        exact_volume: Decimal,
        exact_quote_volume: Decimal,
        exact_vwap: Decimal,
    ) -> Self {
        Self {
            exact_open,
            exact_high,
            exact_low,
            exact_close,
            exact_volume,
            exact_quote_volume,
            exact_vwap,
        }
    }
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
