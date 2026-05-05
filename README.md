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
stock-trek = "0.4.12"
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
    key_market_exists: ScratchKey<bool>,
    key_satoshi_price: ScratchKey<f64>,
}

impl Default for CostAveraging {
    fn default() -> Self {
        Self {
            key_market_exists: ScratchKey::new_optional("MARKET_EXISTS", false),
            key_satoshi_price: ScratchKey::new_required("SATOSHI_PRICE"),
        }
    }
}

#[register_strategy(default)]
impl Strategy for CostAveraging {
    fn market_calculations(&self, c: &StrategyContext) -> StockTrekResult<ScratchPad> {
        let mut scratch_pad = ScratchPad::new();
        if let Some(binance) = c.exchanges.get(&ExchangeId::Binance) {
            let btc_usdt = c.symbol(BTC, USDT);
            let market_opt = binance.market_for(&btc_usdt)?;
            if let Some(market) = market_opt {
                scratch_pad.write(&self.key_market_exists, true);
                let satoshi_price = market.ticks.ticks[0].last.price / 1_000_000.0;
                scratch_pad.write(&self.key_satoshi_price, satoshi_price);
            }
        }
        Ok(scratch_pad)
    }
    fn on_action_error(&self) -> OnActionError {
        OnActionError::Halt
    }
    fn action_resolver(&self, c: &ResolverContext) -> StockTrekResult<Resolver> {
        Ok(c.resolvers.if_else(
            c.predicates.scratch_pad(&self.key_market_exists),
            c.resolvers.if_else(
                c.predicates.compare(
                    c.portfolio.asset_in_exchange(
                        c.literals.exchange(ExchangeId::Binance),
                        c.literals.asset(USDT),
                    ),
                    Ordering::Greater,
                    c.scratch_pad.number(&self.key_satoshi_price),
                ),
                c.resolvers.action(c.actions.order_request(
                    ExchangeId::Binance,
                    OrderRequest {
                        account_type: AccountType::Spot,
                        client_order_id: None,
                        order_type: OrderType::Market,
                        quantity: 1.0 / 1_000_000.0,
                        reduce_only: false,
                        side: OrderSide::Buy,
                        symbol: Symbol::new(BTC, USDT),
                        time_in_force: TimeInForce::Fok,
                    },
                )),
                c.resolvers.no_op(),
            ),
            c.resolvers.no_op(),
        ))
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
