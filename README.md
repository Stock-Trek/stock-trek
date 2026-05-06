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
stock-trek = "0.5.0"
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
    exchange_binance: ExchangeName,
    token_btc: TokenName,
    token_usdt: TokenName,
    key_exchange: ScratchKey<ExchangeName>,
    key_market_exists: ScratchKey<bool>,
    key_satoshi_price: ScratchKey<f64>,
    key_satoshi_quantity: ScratchKey<f64>,
}

impl Default for CostAveraging {
    fn default() -> Self {
        Self {
            exchange_binance: ExchangeName("Binance".into()),
            token_btc: TokenName(BTC.into()),
            token_usdt: TokenName(USDT.into()),
            key_exchange: ScratchKey::new_required("EXCHANGE"),
            key_market_exists: ScratchKey::new_optional("MARKET_EXISTS", false),
            key_satoshi_price: ScratchKey::new_required("SATOSHI_PRICE"),
            key_satoshi_quantity: ScratchKey::new_required("SATOSHI_QUANTITY"),
        }
    }
}

#[register_strategy(default)]
impl Strategy for CostAveraging {
    fn market_calculations(&self, c: &StrategyContext) -> StockTrekResult<ScratchPad> {
        let mut scratch_pad = ScratchPad::new();
        let one_millionth = 1.0 / 1_000_000.0;
        scratch_pad.write(&self.key_satoshi_quantity, one_millionth);
        if let Some(binance_bt_usdt) =
            c.market_for(&self.exchange_binance, &self.token_btc, &self.token_usdt)
        {
            scratch_pad.write(&self.key_exchange, self.exchange_binance.clone());
            scratch_pad.write(&self.key_market_exists, true);
            let satoshi_price = binance_bt_usdt.ticks.ticks[0].last.price / 1_000_000.0;
            scratch_pad.write(&self.key_satoshi_price, satoshi_price);
        }
        Ok(scratch_pad)
    }
    fn resolver(&self, c: &ResolverContext) -> StockTrekResult<Resolver> {
        let exchange = c.scratch_pad.exchange(&self.key_exchange);
        let btc = c.literals.token(self.token_btc.clone());
        let usdt = c.literals.token(self.token_usdt.clone());
        let satoshi_price = c.scratch_pad.number(&self.key_satoshi_price);
        let quantity = c.scratch_pad.number(&self.key_satoshi_quantity);
        Ok(c.resolvers.if_else(
            c.predicates.scratch_pad(&self.key_market_exists),
            c.resolvers.if_else(
                c.predicates.compare(
                    c.portfolio
                        .token_in_exchange(exchange.clone(), usdt.clone()),
                    Ordering::Greater,
                    satoshi_price,
                ),
                c.resolvers.place_order(
                    exchange.clone(),
                    c.orders.market(
                        btc,
                        usdt.clone(),
                        OrderIntent::Open,
                        OrderSide::Buy,
                        OrderQuantity::OfQuote(quantity),
                    ),
                ),
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
