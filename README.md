# stock-trek

A tool for running crypto bots on [stock-trek.com](https://stock-trek.com). Rust-native core with optional Python bindings (coming soon)

## Installation

Add to your Cargo.toml:

```rs
[dependencies]
stock-trek = "0.8.15"
```

## Python Bindings (coming soon)

stock-trek will also provide Python bindings in the future, available for installation via

`pip install stock-trek`

## Usage

Implement the `Algorithm` and `Default` traits for your algorithm and register it with the annotation `#[register_algorithm(default)]`.
An example implementing a cost averaging algorithm follows:

```rs
use stock_trek::prelude::*;
use std::cmp::Ordering;

pub struct CostAveraging {
    key_cex: SignalKey<CexId>,
    key_market_exists: SignalKey<bool>,
    key_satoshi_price: SignalKey<f64>,
    key_satoshi_quantity: SignalKey<f64>,
}

impl Default for CostAveraging {
    fn default() -> Self {
        Self {
            key_cex: SignalKey::new_required("CEX"),
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
            cex: CexPreferences {
                max_network_delay_millis: 5000,
                rounding: Rounding {
                    activation_price_triggered_above: RoundingStrategy::AwayFromZero,
                    activation_price_triggered_below: RoundingStrategy::ToZero,
                    price: RoundingStrategy::ToZero,
                    quantity: RoundingStrategy::ToZero,
                    callback_rate_bps: RoundingStrategy::ToZero,
                },
            },
        }
    }
    fn signals(&self, c: &SignalContext) -> Signals {
        let mut signals = Signals::new();
        let one_millionth = 1.0 / 1_000_000.0;
        signals.write(&self.key_satoshi_quantity, one_millionth);
        let iter = c.cex_markets_for(AssetId::bitcoin(), AssetId::tether_usd());
        let min_by_last_ask = iter.min_by(|(_a_exch, a_market), (_b_exch, b_market)| {
            let a_last_ask = a_market.ticks.ticks[0].ask.price;
            let b_last_ask = b_market.ticks.ticks[0].ask.price;
            a_last_ask.partial_cmp(&b_last_ask).unwrap()
        });
        if let Some((cheapest_cex_name, market)) = min_by_last_ask {
            signals.write(&self.key_cex, cheapest_cex_name);
            signals.write(&self.key_market_exists, true);
            let satoshi_price = market.ticks.ticks[0].ask.price / 1_000_000.0;
            signals.write(&self.key_satoshi_price, satoshi_price);
        }
        signals
    }
    fn strategy(&self, c: &StrategyContext) -> Command {
        let cex = c.signals.cex_id(&self.key_cex);
        let btc = c.literals.asset_id(AssetId::bitcoin());
        let usdt = c.literals.asset_id(AssetId::tether_usd());
        let satoshi_price = c.signals.number(&self.key_satoshi_price);
        let quantity = c.signals.number(&self.key_satoshi_quantity);
        c.commands.if_else(
            c.conditions.signal(&self.key_market_exists),
            c.commands.if_else(
                c.conditions.compare(
                    c.portfolio.asset_in_cex(cex.clone(), usdt.clone()),
                    Ordering::Greater,
                    satoshi_price,
                ),
                c.commands.plan(vec![c.actions.send_order_request(
                    cex.clone(),
                    c.orders.single(
                        btc,
                        usdt.clone(),
                        OrderSide::Buy,
                        OrderActivation::Immediate,
                        OrderPricing::Market,
                        OrderQuantity::OfQuote(quantity),
                        vec![OrderConstraint::FillPolicy {
                            allow_partial: true,
                        }],
                        OrderTag::new("CostAveraging"),
                    ),
                    RecoveryPolicy::with_default(ErrorResponse::Stop).on_error(
                        ErrorCause::TemporaryCexRejection,
                        ErrorResponse::Retry { max_retries: 3 },
                    ),
                )]),
                c.commands.no_op(),
            ),
            c.commands.no_op(),
        )
    }
}
```

Stock-Trek verifies code before running it and disallows certain syntax elements. To verify code locally, install the verifier [stock-trek-check](https://crates.io/crates/stock-trek-check) with

```sh
cargo install stock-trek-check
```

then run the verify command with

```sh
stock-trek-check verify --file ./path/algorithm.rs
```

## Status

This project is in early development (0.x). APIs may change.

## License

MIT
