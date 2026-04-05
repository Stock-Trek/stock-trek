use crate::{
    dto::raw_market_candle::RawMarketCandle, market_data::market_ohlcv::MarketOhlcv,
    prelude::TimestampMillis,
};

#[derive(Debug)]
pub struct MarketCandle {
    start_time_millis_inc: TimestampMillis,
    end_time_millis_exc: TimestampMillis,
    duration_millis: TimestampMillis,
    is_candle_closed: bool,
    ohlcv: MarketOhlcv,
    trade_count: u64,
}

impl MarketCandle {
    pub fn start_time_millis_inc(&self) -> TimestampMillis {
        self.start_time_millis_inc
    }
    pub fn end_time_millis_exc(&self) -> TimestampMillis {
        self.end_time_millis_exc
    }
    pub fn duration_millis(&self) -> TimestampMillis {
        self.duration_millis
    }
    pub fn is_candle_closed(&self) -> bool {
        self.is_candle_closed
    }
    pub fn ohlcv(&self) -> &MarketOhlcv {
        &self.ohlcv
    }
    pub fn trade_count(&self) -> u64 {
        self.trade_count
    }
}

impl From<RawMarketCandle> for MarketCandle {
    fn from(value: RawMarketCandle) -> Self {
        let RawMarketCandle {
            start_time_millis_inc,
            end_time_millis_exc,
            duration_millis,
            is_candle_closed,
            ohlcv,
            trade_count,
        } = value;
        MarketCandle {
            start_time_millis_inc,
            end_time_millis_exc,
            duration_millis,
            is_candle_closed,
            ohlcv: MarketOhlcv::from(ohlcv),
            trade_count,
        }
    }
}
