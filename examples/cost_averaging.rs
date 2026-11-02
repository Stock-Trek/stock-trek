use std::cmp::Ordering;
use stock_trek::prelude::*;

/// The satoshi multiplier
const SATOSHI_SCALE: f64 = 1_000_000.0;
/// The base-asset quantity for one satoshi.
const SATOSHI_QUANTITY: f64 = 1.0 / SATOSHI_SCALE;
/// The CEX account that should place orders. TODO make this injectable and mapped by CexId
const CEX_ACCOUNT_ID: &str = "main";

pub struct CostAveraging {
    key_cex: SignalKey<CexId>,
    key_account: SignalKey<AccountId>,
    key_market_exists: SignalKey<bool>,
    key_cheapest_price: SignalKey<f64>,
    key_satoshi_quantity: SignalKey<f64>,
}

impl Default for CostAveraging {
    fn default() -> Self {
        Self {
            key_cex: SignalKey::new_required("CEX"),
            key_account: SignalKey::new_required("ACCOUNT"),
            key_market_exists: SignalKey::new_optional("MARKET_EXISTS", false),
            key_cheapest_price: SignalKey::new_required("CHEAPEST_PRICE"),
            key_satoshi_quantity: SignalKey::new_required("SATOSHI_QUANTITY"),
        }
    }
}

#[register_algorithm(default)]
impl Algorithm for CostAveraging {
    fn name(&self) -> &str {
        "CostAveraging"
    }
    fn description(&self) -> &str {
        "Finds the cheapest market BTC/USDT price, then places a Limit order at that price"
    }
    fn preferences(&self) -> Preferences {
        Preferences {
            cex: CexPreferences {
                max_network_delay_millis: 5000,
                rounding: CexRoundingPreferences {
                    activation_price_triggered_above: RoundingStrategy::AwayFromZero,
                    activation_price_triggered_below: RoundingStrategy::ToZero,
                    price: RoundingStrategy::ToZero,
                    quantity: RoundingStrategy::ToZero,
                    callback_rate_bps: RoundingStrategy::ToZero,
                },
            },
        }
    }
    fn signals(&self, c: &SignalContext) -> StockTrekResult<Signals> {
        let mut signals = Signals::new();
        signals.write(&self.key_satoshi_quantity, SATOSHI_QUANTITY);
        signals.write(&self.key_account, AccountId::new(CEX_ACCOUNT_ID));

        let cheapest_market = c
            .cex_markets_for(AssetId::Bitcoin, AssetId::TetherUSD)
            .filter_map(|(cex, market)| {
                let ask_price = market.ticks().latest()?.ask().price();
                (ask_price.is_finite() && ask_price > 0.0).then_some((cex, ask_price))
            })
            .min_by(|(a_cex, a_price), (b_cex, b_price)| {
                a_price
                    .total_cmp(b_price)
                    .then_with(|| a_cex.to_string().cmp(&b_cex.to_string()))
            });

        if let Some((cheapest_cex, cheapest_ask)) = cheapest_market {
            signals.write(&self.key_cex, cheapest_cex);
            signals.write(&self.key_market_exists, true);
            signals.write(&self.key_cheapest_price, cheapest_ask);
        }
        Ok(signals)
    }
    fn strategy(&self, c: &StrategyContext) -> Command {
        let cex = c.signals.cex_id(&self.key_cex);
        let account = c.signals.account_id(&self.key_account);
        let btc = c.literals.asset_id(AssetId::Bitcoin);
        let usdt = c.literals.asset_id(AssetId::TetherUSD);
        let cheapest_price = c.signals.number(&self.key_cheapest_price);
        let satoshi_quantity = c.signals.number(&self.key_satoshi_quantity);
        let tag = Tag::new("CostAveraging");
        c.commands.if_else(
            c.conditions.signal(&self.key_market_exists),
            c.commands.if_else(
                c.conditions.compare(
                    c.portfolio.asset_available_in_cex_account(
                        cex.clone(),
                        account.clone(),
                        usdt.clone(),
                    ),
                    Ordering::Greater,
                    cheapest_price.clone(),
                ),
                c.commands.plan(vec![c.actions.place_order(
                    cex.clone(),
                    account.clone(),
                    c.orders.limit(
                        btc,
                        usdt.clone(),
                        Side::Buy,
                        Activation::Immediate,
                        cheapest_price,
                        TimeInForce::ImmediateOrCancel,
                        Quantity::OfBase(satoshi_quantity),
                        tag,
                    ),
                    RecoveryPolicy::with_default_response(ActionErrorResponse::Stop).on_error(
                        ActionErrorCause::TemporaryCexRejection,
                        ActionErrorResponse::Retry { max_retries: 3 },
                    ),
                )]),
                c.commands.no_op(),
            ),
            c.commands.no_op(),
        )
    }
}

fn main() {
    let algorithm = CostAveraging::default();
    println!("{}: {}", algorithm.name(), algorithm.description());
}

#[cfg(test)]
mod tests {
    use super::*;
    use stock_trek::markets::{Market, MarketBuilder, TimestampMillis};

    fn market(asks: &[f64]) -> Market {
        let mut builder = MarketBuilder::new(0.000_001, 0.01, 0.000_1);
        for (index, ask) in asks.iter().enumerate() {
            builder = builder.tick(
                TimestampMillis::from(index as u64),
                (ask - 1.0, 1.0),
                (*ask, 1.0),
                (*ask, 1.0),
            );
        }
        builder.build()
    }

    fn context(markets: Vec<(CexId, Market)>) -> SignalContext {
        let mut builder = SignalContextBuilder::new();
        for (cex, market) in markets {
            builder.cex_market(cex, AssetId::Bitcoin, AssetId::TetherUSD, market);
        }
        builder.build()
    }

    #[test]
    fn writes_the_account_even_without_markets() {
        let algorithm = CostAveraging::default();
        let signals = algorithm.signals(&context(vec![])).unwrap();

        assert_eq!(
            signals.read(&algorithm.key_account).unwrap(),
            AccountId::new(CEX_ACCOUNT_ID)
        );
        assert!(!signals.read(&algorithm.key_market_exists).unwrap());
    }

    #[test]
    fn uses_the_latest_ask_and_handles_empty_markets() {
        let algorithm = CostAveraging::default();
        let latest = algorithm
            .signals(&context(vec![(CexId::Binance, market(&[300.0, 200.0]))]))
            .unwrap();
        let empty = algorithm
            .signals(&context(vec![(CexId::Binance, market(&[]))]))
            .unwrap();

        assert!(latest.read(&algorithm.key_market_exists).unwrap());
        assert_eq!(latest.read(&algorithm.key_cheapest_price).unwrap(), 200.0);
        assert!(!empty.read(&algorithm.key_market_exists).unwrap());
    }

    #[test]
    fn skips_non_finite_asks() {
        let algorithm = CostAveraging::default();
        let signals = algorithm
            .signals(&context(vec![(CexId::Binance, market(&[f64::NAN]))]))
            .unwrap();

        assert!(!signals.read(&algorithm.key_market_exists).unwrap());
    }
}
