use crate::{
    error::result::{StockTrekError, StockTrekResult},
    market_data::{
        market_aligned_window::MarketAlignedWindow, market_order_book::MarketOrderBook,
        market_rolling_window::MarketRollingWindow, market_ticks::MarketTicks,
    },
    validation::Validation,
};
use serde::{Deserialize, Serialize};

pub use crate::market_data::market_builder::MarketBuilder;

/// Market data for a single trading pair on a CEX.
///
/// The increment and notional fields describe exchange constraints. Ticks,
/// rolling candles, aligned candles, and the order book hold the recent market
/// history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MarketData")]
pub struct Market {
    pub(super) base_increment: f64,
    pub(super) quote_increment: f64,
    pub(super) minimum_notional: f64,
    pub(super) ticks: MarketTicks,
    pub(super) rolling: MarketRollingWindow,
    pub(super) aligned: MarketAlignedWindow,
    pub(super) order_book: MarketOrderBook,
}

impl Market {
    pub(super) fn validate_parameters(
        function: &'static str,
        base_increment: f64,
        quote_increment: f64,
        minimum_notional: f64,
    ) -> StockTrekResult<()> {
        Validation::positive(function, "base increment", base_increment)?;
        Validation::positive(function, "quote increment", quote_increment)?;
        Validation::non_negative(function, "minimum notional", minimum_notional)?;
        Ok(())
    }

    /// Returns the minimum increment for the base asset quantity.
    pub fn base_increment(&self) -> f64 {
        self.base_increment
    }

    /// Returns the minimum increment for the quote asset price.
    pub fn quote_increment(&self) -> f64 {
        self.quote_increment
    }

    /// Returns the minimum notional value for an order.
    pub fn minimum_notional(&self) -> f64 {
        self.minimum_notional
    }

    /// Returns the recent market ticks.
    pub fn ticks(&self) -> &MarketTicks {
        &self.ticks
    }

    /// Returns the candles for fixed-length rolling windows.
    pub fn rolling(&self) -> &MarketRollingWindow {
        &self.rolling
    }

    /// Returns the candles for aligned windows.
    pub fn aligned(&self) -> &MarketAlignedWindow {
        &self.aligned
    }

    /// Returns the current order book.
    pub fn order_book(&self) -> &MarketOrderBook {
        &self.order_book
    }
}

#[derive(Deserialize)]
struct MarketData {
    base_increment: f64,
    quote_increment: f64,
    minimum_notional: f64,
    ticks: MarketTicks,
    rolling: MarketRollingWindow,
    aligned: MarketAlignedWindow,
    order_book: MarketOrderBook,
}

impl TryFrom<MarketData> for Market {
    type Error = StockTrekError;

    fn try_from(data: MarketData) -> Result<Self, Self::Error> {
        let mut builder = MarketBuilder::new(
            data.base_increment,
            data.quote_increment,
            data.minimum_notional,
        )?;

        for tick in data.ticks.ticks() {
            builder.tick(
                tick.timestamp_millis(),
                tick.bid().clone(),
                tick.ask().clone(),
                tick.last().clone(),
            )?;
        }
        for (window, candle) in data.rolling.candles() {
            builder.rolling_set_candle(*window, candle.clone())?;
        }
        for (window, candles) in data.aligned.candles() {
            for candle in candles {
                builder.aligned_push_candle(*window, candle.clone())?;
            }
        }
        for quote in data.order_book.bids() {
            builder.order_book_bid(quote.clone())?;
        }
        for quote in data.order_book.asks() {
            builder.order_book_ask(quote.clone())?;
        }

        Ok(builder.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::{
        aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
        market_quote::MarketQuote, rolling_window::RollingWindow, timestamp::TimestampMillis,
    };

    fn market() -> Market {
        MarketBuilder::new(0.000_001, 0.01, 1.0).unwrap().build()
    }

    fn candle(start_millis: u64, duration_millis: u64) -> MarketCandle {
        MarketCandle::new(
            TimestampMillis::new(start_millis),
            TimestampMillis::new(start_millis + duration_millis),
            true,
            MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4).unwrap(),
            3,
        )
        .unwrap()
    }

    #[test]
    fn builds_a_market_with_valid_parameters() {
        let market = market();

        assert_eq!(market.base_increment(), 0.000_001);
        assert_eq!(market.quote_increment(), 0.01);
        assert_eq!(market.minimum_notional(), 1.0);
    }

    #[test]
    fn builder_methods_can_be_combined() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 1.0).unwrap();
        builder
            .tick(
                TimestampMillis::new(1),
                MarketQuote::new(100.0, 1.0).unwrap(),
                MarketQuote::new(101.0, 2.0).unwrap(),
                MarketQuote::new(100.5, 3.0).unwrap(),
            )
            .unwrap()
            .rolling_set_candle(RollingWindow::Minutes1, candle(0, 60_000))
            .unwrap()
            .aligned_push_candle(AlignedWindow::Minutes1, candle(0, 60_000))
            .unwrap()
            .order_book_bid(MarketQuote::new(100.0, 1.0).unwrap())
            .unwrap()
            .order_book_ask(MarketQuote::new(101.0, 2.0).unwrap())
            .unwrap();
        let market = builder.build();

        assert_eq!(market.base_increment(), 0.000_001);
        assert_eq!(market.quote_increment(), 0.01);
        assert_eq!(market.minimum_notional(), 1.0);
        assert_eq!(market.ticks().ticks().len(), 1);
        assert_eq!(market.rolling().candles().len(), 1);
        assert_eq!(market.aligned().candles().len(), 1);
        assert_eq!(market.order_book().bids().len(), 1);
        assert_eq!(market.order_book().asks().len(), 1);
    }

    #[test]
    fn rejects_non_finite_and_non_positive_parameters() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -1.0] {
            assert!(MarketBuilder::new(value, 0.01, 1.0).is_err());
            assert!(MarketBuilder::new(0.000_001, value, 1.0).is_err());
        }
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
            assert!(MarketBuilder::new(0.000_001, 0.01, value).is_err());
        }
    }

    #[test]
    fn order_book_rejections_leave_the_builder_usable() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 1.0).unwrap();
        builder
            .order_book_bid(MarketQuote::new(100.0, 1.0).unwrap())
            .unwrap();

        assert!(
            builder
                .order_book_ask(MarketQuote::new(99.0, 1.0).unwrap())
                .is_err()
        );
        builder
            .order_book_ask(MarketQuote::new(101.0, 1.0).unwrap())
            .unwrap();

        let market = builder.build();
        assert_eq!(market.order_book().bids().len(), 1);
        assert_eq!(market.order_book().asks().len(), 1);
    }

    #[test]
    fn rejects_quotes_that_do_not_respect_increments() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.0).unwrap();

        assert!(
            builder
                .order_book_bid(MarketQuote::new(100.005, 1.0).unwrap())
                .is_err()
        );
        assert!(
            builder
                .order_book_bid(MarketQuote::new(100.0, 0.000_000_5).unwrap())
                .is_err()
        );
        assert!(
            builder
                .tick(
                    TimestampMillis::new(1),
                    MarketQuote::new(100.005, 1.0).unwrap(),
                    MarketQuote::new(101.0, 1.0).unwrap(),
                    MarketQuote::new(100.5, 1.0).unwrap(),
                )
                .is_err()
        );
    }

    #[test]
    fn rejects_off_increment_quotes_at_large_magnitudes() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.0).unwrap();

        assert!(
            builder
                .order_book_bid(MarketQuote::new(1_000_000_000.005, 1.0).unwrap())
                .is_err()
        );
        assert!(
            builder
                .order_book_bid(MarketQuote::new(1_000_000_000.0, 1_000_000.000_000_5).unwrap())
                .is_err()
        );
        assert!(
            builder
                .order_book_bid(MarketQuote::new(5_347_718_528.99, 1.0).unwrap())
                .is_ok()
        );
    }

    #[test]
    fn rejects_quotes_below_the_minimum_notional() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 10.0).unwrap();

        assert!(
            builder
                .order_book_bid(MarketQuote::new(100.0, 0.01).unwrap())
                .is_err()
        );
        assert!(
            builder
                .order_book_bid(MarketQuote::new(100.0, 1.0).unwrap())
                .is_ok()
        );
    }

    #[test]
    fn accepts_last_trades_below_the_minimum_notional() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 10.0).unwrap();

        assert!(
            builder
                .clone()
                .tick(
                    TimestampMillis::new(1),
                    MarketQuote::new(100.0, 0.01).unwrap(),
                    MarketQuote::new(101.0, 1.0).unwrap(),
                    MarketQuote::new(100.5, 1.0).unwrap(),
                )
                .is_err()
        );
        assert!(
            builder
                .clone()
                .tick(
                    TimestampMillis::new(1),
                    MarketQuote::new(100.0, 1.0).unwrap(),
                    MarketQuote::new(101.0, 0.01).unwrap(),
                    MarketQuote::new(100.5, 1.0).unwrap(),
                )
                .is_err()
        );

        builder
            .tick(
                TimestampMillis::new(1),
                MarketQuote::new(100.0, 1.0).unwrap(),
                MarketQuote::new(101.0, 1.0).unwrap(),
                MarketQuote::new(100.5, 0.01).unwrap(),
            )
            .unwrap();

        let market = builder.build();
        assert_eq!(market.ticks().ticks()[0].last().quantity(), 0.01);
    }

    #[test]
    fn rejects_crossed_order_books() {
        let builder = || MarketBuilder::new(0.000_001, 0.01, 0.0).unwrap();

        assert!(
            builder()
                .order_book_bid(MarketQuote::new(101.0, 1.0).unwrap())
                .unwrap()
                .order_book_ask(MarketQuote::new(101.0, 1.0).unwrap())
                .is_err()
        );
        assert!(
            builder()
                .order_book_bid(MarketQuote::new(102.0, 1.0).unwrap())
                .unwrap()
                .order_book_ask(MarketQuote::new(101.0, 1.0).unwrap())
                .is_err()
        );
        assert!(
            builder()
                .order_book_ask(MarketQuote::new(100.0, 1.0).unwrap())
                .unwrap()
                .order_book_bid(MarketQuote::new(100.0, 1.0).unwrap())
                .is_err()
        );
    }

    #[test]
    fn rejects_ticks_with_a_crossed_bid_and_ask() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.0).unwrap();

        assert!(
            builder
                .tick(
                    TimestampMillis::new(1),
                    MarketQuote::new(101.0, 1.0).unwrap(),
                    MarketQuote::new(100.0, 1.0).unwrap(),
                    MarketQuote::new(100.5, 1.0).unwrap(),
                )
                .is_err()
        );
    }

    #[test]
    fn rejects_candles_that_do_not_match_their_window() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.0).unwrap();

        assert!(
            builder
                .clone()
                .rolling_set_candle(RollingWindow::Minutes1, candle(0, 1_000))
                .is_err()
        );
        assert!(
            builder
                .clone()
                .aligned_push_candle(AlignedWindow::Minutes1, candle(1_000, 59_000))
                .is_err()
        );
        assert!(
            builder
                .aligned_push_candle(AlignedWindow::Minutes1, candle(0, 60_000))
                .is_ok()
        );
    }

    #[test]
    fn rejects_duplicate_aligned_candle_intervals() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.0).unwrap();
        builder
            .aligned_push_candle(AlignedWindow::Minutes1, candle(0, 60_000))
            .unwrap();

        assert!(
            builder
                .aligned_push_candle(AlignedWindow::Minutes1, candle(0, 60_000))
                .is_err()
        );

        let market = builder.build();
        assert_eq!(
            market
                .aligned()
                .candles_for(AlignedWindow::Minutes1)
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn accepts_calendar_aligned_candles_that_match_their_bounds() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.0).unwrap();
        let bounds = AlignedWindow::Months1
            .bounds_for(TimestampMillis::new(0))
            .unwrap();
        let duration = bounds.upper().as_millis() - bounds.lower().as_millis();
        let candle = candle(bounds.lower().as_millis(), duration);

        assert!(
            builder
                .aligned_push_candle(AlignedWindow::Months1, candle)
                .is_ok()
        );
    }

    #[test]
    fn serde_json_round_trip_preserves_the_market() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 1.0).unwrap();
        builder
            .order_book_bid(MarketQuote::new(100.0, 1.0).unwrap())
            .unwrap()
            .order_book_ask(MarketQuote::new(101.0, 1.0).unwrap())
            .unwrap();
        let market = builder.build();

        let json = serde_json::to_string(&market).unwrap();
        let deserialized: Market = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.base_increment(), market.base_increment());
        assert_eq!(deserialized.order_book().bids(), market.order_book().bids());
    }

    #[test]
    fn deserialization_rejects_invalid_parameters() {
        let mut value = serde_json::to_value(market()).unwrap();
        value["base_increment"] = serde_json::json!(-1.0);

        assert!(serde_json::from_value::<Market>(value).is_err());
    }

    #[test]
    fn deserialization_rejects_invalid_nested_quotes() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 1.0).unwrap();
        builder
            .order_book_bid(MarketQuote::new(100.0, 1.0).unwrap())
            .unwrap();
        let market = builder.build();
        let mut value = serde_json::to_value(market).unwrap();
        value["order_book"]["bids"][0]["quantity"] = serde_json::json!(0.0);

        assert!(serde_json::from_value::<Market>(value).is_err());
    }

    #[test]
    fn deserialization_rejects_semantically_inconsistent_data() {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.0).unwrap();
        builder
            .order_book_bid(MarketQuote::new(100.0, 1.0).unwrap())
            .unwrap()
            .order_book_ask(MarketQuote::new(101.0, 1.0).unwrap())
            .unwrap();
        let market = builder.build();
        let value = serde_json::to_value(market).unwrap();

        let mut crossed = value.clone();
        crossed["order_book"]["bids"][0]["price"] = serde_json::json!(102.0);
        assert!(serde_json::from_value::<Market>(crossed).is_err());

        let mut off_increment = value.clone();
        off_increment["order_book"]["bids"][0]["price"] = serde_json::json!(100.005);
        assert!(serde_json::from_value::<Market>(off_increment).is_err());

        let mut wrong_candle = value;
        wrong_candle["rolling"] = serde_json::json!({
            "candles": {
                "1m": candle(0, 1_000),
            }
        });
        assert!(serde_json::from_value::<Market>(wrong_candle).is_err());
    }
}
