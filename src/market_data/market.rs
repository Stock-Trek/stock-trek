use crate::{
    error::result::{StockTrekError, StockTrekResult},
    market_data::{
        aligned_window::AlignedWindow,
        market_aligned_window::MarketAlignedWindow,
        market_candle::MarketCandle,
        market_order_book::MarketOrderBook,
        market_quote::{IntoMarketQuote, MarketQuote},
        market_rolling_window::MarketRollingWindow,
        market_tick::MarketTick,
        market_ticks::MarketTicks,
        rolling_window::RollingWindow,
        timestamp::TimestampMillis,
        validation::Validation,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Market data for a single trading pair on a CEX.
///
/// The increment and notional fields describe exchange constraints. Ticks,
/// rolling candles, aligned candles, and the order book hold the recent market
/// history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MarketData")]
pub struct Market {
    base_increment: f64,
    quote_increment: f64,
    minimum_notional: f64,
    ticks: MarketTicks,
    rolling: MarketRollingWindow,
    aligned: MarketAlignedWindow,
    order_book: MarketOrderBook,
}

impl Market {
    fn validate_parameters(
        function: &'static str,
        base_increment: f64,
        quote_increment: f64,
        minimum_notional: f64,
    ) -> StockTrekResult<()> {
        Validation::positive(function, "base increment", base_increment)?;
        Validation::positive(function, "quote increment", quote_increment)?;
        Validation::non_negative(function, "minimum notional", minimum_notional)
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
        Self::validate_parameters(
            "Market",
            data.base_increment,
            data.quote_increment,
            data.minimum_notional,
        )?;
        Ok(Self {
            base_increment: data.base_increment,
            quote_increment: data.quote_increment,
            minimum_notional: data.minimum_notional,
            ticks: data.ticks,
            rolling: data.rolling,
            aligned: data.aligned,
            order_book: data.order_book,
        })
    }
}

/// Builder for constructing a [`Market`].
///
/// New markets start empty. Add ticks, candles, and order book quotes before
/// calling [`MarketBuilder::build`].
#[derive(Debug, Clone)]
pub struct MarketBuilder {
    base_increment: f64,
    quote_increment: f64,
    minimum_notional: f64,
    ticks: Vec<MarketTick>,
    rolling: HashMap<RollingWindow, MarketCandle>,
    aligned: HashMap<AlignedWindow, Vec<MarketCandle>>,
    order_book_asks: Vec<MarketQuote>,
    order_book_bids: Vec<MarketQuote>,
    error: Option<StockTrekError>,
}

impl MarketBuilder {
    /// Creates a builder with the market's trading increments and minimum notional.
    pub fn new(base_increment: f64, quote_increment: f64, minimum_notional: f64) -> Self {
        Self {
            base_increment,
            quote_increment,
            minimum_notional,
            ticks: Vec::new(),
            rolling: HashMap::new(),
            aligned: HashMap::new(),
            order_book_bids: Vec::new(),
            order_book_asks: Vec::new(),
            error: None,
        }
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

    /// Returns the ticks added to the builder.
    pub fn ticks(&self) -> &[MarketTick] {
        &self.ticks
    }

    /// Returns the rolling-window candles added to the builder.
    pub fn rolling(&self) -> &HashMap<RollingWindow, MarketCandle> {
        &self.rolling
    }

    /// Returns the aligned-window candles added to the builder.
    pub fn aligned(&self) -> &HashMap<AlignedWindow, Vec<MarketCandle>> {
        &self.aligned
    }

    /// Returns the bid quotes added to the builder.
    pub fn order_book_bids(&self) -> &[MarketQuote] {
        &self.order_book_bids
    }

    /// Returns the ask quotes added to the builder.
    pub fn order_book_asks(&self) -> &[MarketQuote] {
        &self.order_book_asks
    }

    /// Adds a market tick with the given bid, ask, and last quotes.
    ///
    /// Each quote may be provided as a validated [`MarketQuote`] or as a
    /// `(price, quantity)` pair, which is validated before it is added.
    pub fn tick(
        mut self,
        timestamp_millis: TimestampMillis,
        bid: impl IntoMarketQuote,
        ask: impl IntoMarketQuote,
        last: impl IntoMarketQuote,
    ) -> Self {
        let tick = bid.into_market_quote().and_then(|bid| {
            Ok(MarketTick::new(
                timestamp_millis,
                bid,
                ask.into_market_quote()?,
                last.into_market_quote()?,
            ))
        });
        match tick {
            Ok(tick) => self.ticks.push(tick),
            Err(error) => self.record_error(error),
        }
        self
    }
    /// Sets the candle for a fixed-length rolling window.
    pub fn rolling_set_candle(mut self, window: RollingWindow, candle: MarketCandle) -> Self {
        self.rolling.insert(window, candle);
        self
    }
    /// Appends a candle to an aligned window.
    pub fn aligned_push_candle(mut self, window: AlignedWindow, candle: MarketCandle) -> Self {
        self.aligned.entry(window).or_default().push(candle);
        self
    }
    /// Adds a bid quote to the order book.
    pub fn order_book_bid(mut self, price: f64, quantity: f64) -> Self {
        match MarketQuote::new(price, quantity) {
            Ok(quote) => self.order_book_bids.push(quote),
            Err(error) => self.record_error(error),
        }
        self
    }
    /// Adds an ask quote to the order book.
    pub fn order_book_ask(mut self, price: f64, quantity: f64) -> Self {
        match MarketQuote::new(price, quantity) {
            Ok(quote) => self.order_book_asks.push(quote),
            Err(error) => self.record_error(error),
        }
        self
    }
    /// Builds the [`Market`] from the collected data.
    ///
    /// Returns an error if the increments, minimum notional, ticks, or order
    /// book contain invalid values.
    pub fn build(self) -> StockTrekResult<Market> {
        if let Some(error) = self.error {
            return Err(error);
        }
        Market::validate_parameters(
            "MarketBuilder::build",
            self.base_increment,
            self.quote_increment,
            self.minimum_notional,
        )?;
        let order_book = MarketOrderBook::new(self.order_book_bids, self.order_book_asks)?;
        Ok(Market {
            base_increment: self.base_increment,
            quote_increment: self.quote_increment,
            minimum_notional: self.minimum_notional,
            ticks: MarketTicks::new(self.ticks),
            rolling: MarketRollingWindow::new(self.rolling),
            aligned: MarketAlignedWindow::new(self.aligned),
            order_book,
        })
    }

    fn record_error(&mut self, error: StockTrekError) {
        if self.error.is_none() {
            self.error = Some(error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::market_ohlcv::MarketOhlcv;

    fn market() -> Market {
        MarketBuilder::new(0.000_001, 0.01, 1.0).build().unwrap()
    }

    fn candle(start_millis: u64) -> MarketCandle {
        MarketCandle::new(
            TimestampMillis::new(start_millis),
            TimestampMillis::new(start_millis + 1_000),
            TimestampMillis::new(1_000),
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
    fn builder_methods_can_be_chained() {
        let market = MarketBuilder::new(0.000_001, 0.01, 1.0)
            .tick(
                TimestampMillis::new(1),
                (100.0, 1.0),
                (101.0, 2.0),
                (100.5, 3.0),
            )
            .rolling_set_candle(RollingWindow::Minutes1, candle(1_000))
            .aligned_push_candle(AlignedWindow::Minutes1, candle(1_000))
            .order_book_bid(100.0, 1.0)
            .order_book_ask(101.0, 2.0)
            .build()
            .unwrap();

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
            assert!(MarketBuilder::new(value, 0.01, 1.0).build().is_err());
            assert!(MarketBuilder::new(0.000_001, value, 1.0).build().is_err());
        }
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
            assert!(MarketBuilder::new(0.000_001, 0.01, value).build().is_err());
        }
    }

    #[test]
    fn accepts_quotes_or_pairs_when_building_ticks() {
        let market = MarketBuilder::new(0.000_001, 0.01, 1.0)
            .tick(
                TimestampMillis::new(1),
                MarketQuote::new(100.0, 1.0).unwrap(),
                (101.0, 2.0),
                MarketQuote::new(100.5, 3.0).unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(market.ticks().ticks().len(), 1);
    }

    #[test]
    fn rejects_invalid_tick_quotes() {
        let builder = MarketBuilder::new(0.000_001, 0.01, 1.0).tick(
            TimestampMillis::new(1),
            (f64::NAN, 1.0),
            (101.0, 1.0),
            (100.0, 1.0),
        );

        assert!(builder.build().is_err());
    }

    #[test]
    fn rejects_invalid_order_book_quotes() {
        let builder = MarketBuilder::new(0.000_001, 0.01, 1.0)
            .order_book_bid(f64::NAN, 1.0)
            .order_book_ask(101.0, f64::INFINITY);

        assert!(builder.build().is_err());
    }

    #[test]
    fn serde_json_round_trip_preserves_the_market() {
        let market = MarketBuilder::new(0.000_001, 0.01, 1.0)
            .order_book_bid(100.0, 1.0)
            .order_book_ask(101.0, 1.0)
            .build()
            .unwrap();

        let json = serde_json::to_string(&market).unwrap();
        let deserialized: Market = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.base_increment(), market.base_increment());
        assert_eq!(
            deserialized.order_book().bid_price_quantities(),
            vec![(100.0, 1.0)]
        );
    }

    #[test]
    fn deserialization_rejects_invalid_parameters() {
        let mut value = serde_json::to_value(market()).unwrap();
        value["base_increment"] = serde_json::json!(-1.0);

        assert!(serde_json::from_value::<Market>(value).is_err());
    }

    #[test]
    fn deserialization_rejects_invalid_nested_quotes() {
        let market = MarketBuilder::new(0.000_001, 0.01, 1.0)
            .order_book_bid(100.0, 1.0)
            .build()
            .unwrap();
        let mut value = serde_json::to_value(market).unwrap();
        value["order_book"]["bids"][0]["quantity"] = serde_json::json!(0.0);

        assert!(serde_json::from_value::<Market>(value).is_err());
    }
}
