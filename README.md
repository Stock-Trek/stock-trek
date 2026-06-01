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
stock-trek = "0.7.5"
```

## Python Bindings (coming soon)

stock-trek will also provide Python bindings in the future, available for installation via

`pip install stock-trek`

## Usage

Implement the `Strategy` and `Default` traits for your algorithm and register it with the annotation `#[register_algorithm(default)]`.
An example implementing a cost averaging strategy follows:

```rs
use stock_trek::prelude::*;
use std::cmp::Ordering;

pub struct CostAveraging {
    key_exchange: SignalKey<ExchangeId>,
    key_market_exists: SignalKey<bool>,
    key_satoshi_price: SignalKey<f64>,
    key_satoshi_quantity: SignalKey<f64>,
}

impl Default for CostAveraging {
    fn default() -> Self {
        Self {
            key_exchange: SignalKey::new_required("EXCHANGE"),
            key_market_exists: SignalKey::new_optional("MARKET_EXISTS", false),
            key_satoshi_price: SignalKey::new_required("SATOSHI_PRICE"),
            key_satoshi_quantity: SignalKey::new_required("SATOSHI_QUANTITY"),
        }
    }
}

#[register_algorithm(default)]
impl Algorithm for CostAveraging {
    fn preferences(&self) -> Preferences {
        Preferences {
            rounding: Rounding {
                activation_price_triggered_above: RoundingStrategy::AwayFromZero,
                activation_price_triggered_below: RoundingStrategy::ToZero,
                price: RoundingStrategy::ToZero,
                quantity: RoundingStrategy::ToZero,
                callback_rate_bps: RoundingStrategy::ToZero,
            },
            multi_leg: MultiLeg {
                if_different_price_unsupported: OnDifferent::UseDataFromPrimary,
                if_different_symbol_unsupported: OnDifferent::UseDataFromPrimary,
            },
        }
    }
    fn signals(&self, c: &SignalContext) -> Signals {
        let mut signals = Signals::new();
        let one_millionth = 1.0 / 1_000_000.0;
        signals.write(&self.key_satoshi_quantity, one_millionth);
        let iter = c.exchange_markets_for(AssetId::bitcoin_native(), AssetId::ethereum_usdt());
        let min_by_last_ask = iter.min_by(|(_a_exch, a_market), (_b_exch, b_market)| {
            let a_last_ask = a_market.ticks.ticks[0].ask.price;
            let b_last_ask = b_market.ticks.ticks[0].ask.price;
            a_last_ask.partial_cmp(&b_last_ask).unwrap()
        });
        if let Some((cheapest_exchange_name, market)) = min_by_last_ask {
            signals.write(&self.key_exchange, cheapest_exchange_name);
            signals.write(&self.key_market_exists, true);
            let satoshi_price = market.ticks.ticks[0].ask.price / 1_000_000.0;
            signals.write(&self.key_satoshi_price, satoshi_price);
        }
        signals
    }
    fn strategy(&self, c: &StrategyContext) -> Command {
        let exchange = c.signals.exchange_id(&self.key_exchange);
        let btc = c.literals.asset_id(AssetId::bitcoin_native());
        let usdt = c.literals.asset_id(AssetId::ethereum_usdt());
        let satoshi_price = c.signals.number(&self.key_satoshi_price);
        let quantity = c.signals.number(&self.key_satoshi_quantity);
        c.commands.if_else(
            c.predicates.signal(&self.key_market_exists),
            c.commands.if_else(
                c.predicates.compare(
                    c.portfolio
                        .asset_in_exchange(exchange.clone(), usdt.clone()),
                    Ordering::Greater,
                    satoshi_price,
                ),
                c.commands.enqueue_order(
                    exchange.clone(),
                    c.orders.single(
                        btc,
                        usdt.clone(),
                        OrderIntent::Open,
                        OrderSide::Buy,
                        OrderActivation::Immediate,
                        OrderPricing::Market,
                        OrderQuantity::OfQuote(quantity),
                        vec![OrderConstraint::FillPolicy {
                            allow_partial: true,
                        }],
                    ),
                ),
                c.commands.no_op(),
            ),
            c.commands.no_op(),
        )
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
