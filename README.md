# stock-trek

A lightweight, composable time series and statistical toolkit designed for running crypto bots on [stock-trek.com](https://stock-trek.com). Rust-native core with optional Python bindings

## Overview

stock-trek provides core abstractions and utilities for working with market data, including:

- Order books
- Aligned/Rolling windows
- Ticks
- Statistical and analytical functions

## Installation

Add to your Cargo.toml:

```rs
[dependencies]
stock-trek = "0.4.3"
```

## Python Bindings (coming soon)

stock-trek will also provide Python bindings in the future, available for installation via

`pip install stock-trek`

## Usage

Implement the `Strategy` and `Default` traits for your algorithm and register it with the annotation `#[register_strategy(default)]`.
An example implementing a cost averaging strategy follows:

```rs
use stock_trek::prelude::*;
use std::cmp::Ordering;

const BTC: &str = "BTC";
const USDT: &str = "USDT";

pub struct CostAveraging {
    key_satoshi_price: ScratchKey<f64>,
}

impl Default for CostAveraging {
    fn default() -> Self {
        Self {
            key_satoshi_price: ScratchKey::new("SATOSHI_PRICE"),
        }
    }
}

#[register_strategy(default)]
impl Strategy for CostAveraging {
    fn market_calculations(&self, context: StrategyContext) -> StockTrekResult<ScratchPad> {
        let mut scratch_pad = ScratchPad::new();
        if let Some(binance) = context.exchanges.get(&ExchangeId::Binance) {
            let btc_usdt = context.symbol(BTC, USDT);
            let market_opt = binance.market_for(&btc_usdt)?;
            if let Some(market) = market_opt {
                let satoshi_price = market.ticks.ticks[0].last.price / 1_000_000.0;
                scratch_pad.write(&self.key_satoshi_price, satoshi_price);
            }
        }
        Ok(scratch_pad)
    }
    fn action_resolver(&self, context: ResolverContext) -> StockTrekResult<Resolver> {
        let exchange = ExchangeId::Binance;
        let satoshi_price = context.scratch_pad.number(&self.key_satoshi_price);
        let usdt = context.portfolio.asset_in_exchange(
            context.literals.exchange(exchange),
            context.literals.asset(USDT),
        );
        let can_buy = context
            .predicates
            .compare(usdt, Ordering::Greater, satoshi_price);
        let order_request = OrderRequest {
            account_type: AccountType::Spot,
            client_order_id: None,
            order_type: OrderType::Market,
            quantity: 1.0 / 1_000_000.0,
            reduce_only: false,
            side: OrderSide::Buy,
            symbol: Symbol::new(BTC, USDT),
            time_in_force: TimeInForce::Fok,
        };
        let buy_one_satoshi_action = context.actions.order_request(exchange, order_request);
        let buy_one_satoshi = context.resolvers.action(buy_one_satoshi_action);
        let no_op = context.resolvers.no_op();
        let resolver = context.resolvers.if_else(can_buy, buy_one_satoshi, no_op);
        Ok(resolver)
    }
}
```

Stock-Trek verifies code before running it and disallows certain syntax elements. To verify code locally, install it with

```sh
cargo install stock-trek
```

then run the verify command with

```sh
stock-trek verify --file ./path/strategy.rs
```

## Roadmap

Planned features include:

- Technical indicators (EMA, RSI, MACD, etc.)
- Backtesting and simulation utilities

## Status

This project is in early development (0.x). APIs may change.

## License

MIT
