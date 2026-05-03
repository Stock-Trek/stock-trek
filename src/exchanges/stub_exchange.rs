use crate::{
    error::result::StockTrekResult,
    exchanges::{
        exchange::{Exchange, ExchangeTrait},
        order_capability::OrderCapability,
    },
    market_data::{
        aligned_window::AlignedWindow, market::Market, market_aligned_window::MarketAlignedWindow,
        market_candle::MarketCandle, market_ohlcv::MarketOhlcv, market_order_book::MarketOrderBook,
        market_quote::MarketQuote, market_rolling_window::MarketRollingWindow,
        market_tick::MarketTick, market_ticks::MarketTicks, rolling_window::RollingWindow,
    },
};
use chrono::Utc;
use digdigdig3::{core::OrderRequest, Order, OrderStatus, OrderType, Symbol};
use std::{collections::HashMap, vec};
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StubExchange {
    market_stub: Market,
}

impl StubExchange {
    pub fn new() -> Self {
        let tick = MarketTick {
            bid: MarketQuote {
                price: 1.5,
                quantity: 10.0,
            },
            ask: MarketQuote {
                price: 1.6,
                quantity: 20.0,
            },
            last: MarketQuote {
                price: 1.55,
                quantity: 10.0,
            },
            timestamp_millis: 1800,
        };
        let ticks = MarketTicks::new(vec![tick]);
        let ohlcv = MarketOhlcv::new(1.5, 2.1, 1.2, 1.8, 1_000.0, 200.0, 1.6);
        let candle = MarketCandle::new(0, 3600, 3600, true, ohlcv, 50);
        let mut aligned_candles = HashMap::new();
        for window in AlignedWindow::iter() {
            aligned_candles.insert(window, vec![candle.clone()]);
        }
        let aligned = MarketAlignedWindow::new(aligned_candles);
        let mut rolling_candles = HashMap::new();
        for window in RollingWindow::iter() {
            rolling_candles.insert(window, candle.clone());
        }
        let rolling = MarketRollingWindow::new(rolling_candles);
        let bids = vec![MarketQuote {
            price: 1.49,
            quantity: 5.0,
        }];
        let asks = vec![MarketQuote {
            price: 1.51,
            quantity: 20.0,
        }];
        let order_book = MarketOrderBook::new(bids, asks);
        let market_stub = Market {
            base_increment: 0.0,
            quote_increment: 0.0,
            minimum_notional: 0.0,
            ticks,
            aligned,
            rolling,
            order_book,
        };
        Self { market_stub }
    }
}

impl Default for StubExchange {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StubExchange> for Exchange {
    fn from(value: StubExchange) -> Self {
        Box::new(value)
    }
}

impl ExchangeTrait for StubExchange {
    fn has_capability(&self, _capability: &OrderCapability) -> StockTrekResult<bool> {
        Ok(true)
    }
    fn market_for(&self, _symbol: &Symbol) -> StockTrekResult<Option<&Market>> {
        Ok(Some(&self.market_stub))
    }
    fn place_order(&self, request: &OrderRequest) -> StockTrekResult<Order> {
        let price = match request.order_type {
            OrderType::Bracket {
                price,
                take_profit: _,
                stop_loss: _,
            } => price,
            OrderType::Fok { price } => Some(price),
            OrderType::Iceberg {
                price,
                display_quantity: _,
            } => Some(price),
            OrderType::Ioc { price } => price,
            OrderType::Limit { price } => Some(price),
            OrderType::Oco {
                price,
                stop_price: _,
                stop_limit_price: _,
            } => Some(price),
            OrderType::PostOnly { price } => Some(price),
            OrderType::ReduceOnly { price } => price,
            _ => None,
        };
        let stop_price = match request.order_type {
            OrderType::Oco {
                price: _,
                stop_price,
                stop_limit_price: _,
            } => Some(stop_price),
            OrderType::StopLimit {
                stop_price,
                limit_price: _,
            } => Some(stop_price),
            OrderType::StopMarket { stop_price } => Some(stop_price),
            _ => None,
        };
        let order = Order {
            // copied
            client_order_id: request.client_order_id.clone(),
            order_type: request.order_type.clone(),
            quantity: request.quantity,
            side: request.side,
            symbol: request.symbol.to_underscore(),
            time_in_force: request.time_in_force,
            // matched
            price,
            stop_price,
            // new
            average_price: None,
            commission: None,
            commission_asset: None,
            created_at: Utc::now().timestamp_millis(),
            filled_quantity: 0.0,
            id: Uuid::new_v4().to_string(),
            status: OrderStatus::New,
            updated_at: None,
        };
        Ok(order)
    }
}
