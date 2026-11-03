use std::cmp::Ordering;
use stock_trek::{prelude::*, values::BinaryOperator};

/// The number of satoshis in one bitcoin.
const SATOSHIS_PER_BITCOIN: f64 = 100_000_000.0;
/// The base-asset quantity bought per order, expressed in satoshis. This matches the
/// example market's minimum base increment.
const ORDER_QUANTITY_SATOSHIS: f64 = 100.0;
/// The base-asset quantity bought per order (100 satoshis = 0.000001 BTC).
const ORDER_QUANTITY: f64 = ORDER_QUANTITY_SATOSHIS / SATOSHIS_PER_BITCOIN;
/// The quote-currency buffer added to the order notional to cover fees and rounding.
const FEE_BUFFER_RATE: f64 = 0.001;
/// The CEX account that should place orders.
const CEX_ACCOUNT_ID: &str = "main";

pub struct CostAveraging {
    key_cex: SignalKey<CexId>,
    key_account: SignalKey<AccountId>,
    key_market_exists: SignalKey<bool>,
    key_cheapest_price: SignalKey<f64>,
    key_order_quantity: SignalKey<f64>,
}

impl Default for CostAveraging {
    fn default() -> Self {
        Self {
            key_cex: SignalKey::new_required("CEX"),
            key_account: SignalKey::new_required("ACCOUNT"),
            key_market_exists: SignalKey::new_optional("MARKET_EXISTS", false),
            key_cheapest_price: SignalKey::new_required("CHEAPEST_PRICE"),
            key_order_quantity: SignalKey::new_required("ORDER_QUANTITY"),
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
        signals.write(&self.key_order_quantity, ORDER_QUANTITY)?;
        signals.write(&self.key_account, AccountId::new(CEX_ACCOUNT_ID))?;

        let cheapest_market = c
            .cex_markets_for(AssetId::Bitcoin, AssetId::TetherUSD)
            .filter_map(|(cex, market)| {
                let ask_price = market.ticks().latest()?.ask().price();
                (ask_price.is_finite() && ask_price > 0.0).then_some((cex, ask_price))
            })
            .min_by(|(_, a_price), (_, b_price)| a_price.total_cmp(b_price));

        if let Some((cheapest_cex, cheapest_ask)) = cheapest_market {
            signals.write(&self.key_cex, cheapest_cex)?;
            signals.write(&self.key_market_exists, true)?;
            signals.write(&self.key_cheapest_price, cheapest_ask)?;
        }
        Ok(signals)
    }
    fn strategy(&self, c: &StrategyContext) -> Command {
        let cex = c.signals.cex_id(&self.key_cex);
        let account = c.signals.account_id(&self.key_account);
        let btc = c.literals.asset_id(AssetId::Bitcoin);
        let usdt = c.literals.asset_id(AssetId::TetherUSD);
        let cheapest_price = c.signals.number(&self.key_cheapest_price);
        let order_quantity = c.signals.number(&self.key_order_quantity);
        let notional = c.calculations.binary(
            cheapest_price.clone(),
            BinaryOperator::Mul,
            order_quantity.clone(),
        );
        let required_quote = c.calculations.binary(
            notional,
            BinaryOperator::Mul,
            c.literals.number(1.0 + FEE_BUFFER_RATE),
        );
        let available_quote =
            c.portfolio
                .asset_available_in_cex_account(cex.clone(), account.clone(), usdt.clone());
        let tag = Tag::new("CostAveraging");
        c.commands.if_else(
            c.conditions.signal(&self.key_market_exists),
            c.commands.if_else(
                c.conditions.not(c.conditions.compare(
                    available_quote,
                    Ordering::Less,
                    required_quote,
                )),
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
                        Quantity::OfBase(order_quantity),
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
    use std::{cell::RefCell, rc::Rc};
    use stock_trek::{
        PortfolioBuilder, ResolvedContext,
        actions::ResolvedAction,
        markets::{Market, MarketBuilder, MarketQuote, TimestampMillis},
        types::OrderRequest,
    };

    const ASK: f64 = 200.0;

    fn market(asks: &[f64]) -> Market {
        let mut builder = MarketBuilder::new(ORDER_QUANTITY, 0.01, 0.000_1);
        for (index, ask) in asks.iter().enumerate() {
            builder.tick(
                TimestampMillis::from(index as u64),
                MarketQuote::new(ask - 1.0, 1.0).unwrap(),
                MarketQuote::new(*ask, 1.0).unwrap(),
                MarketQuote::new(*ask, 1.0).unwrap(),
            );
        }
        builder.build().unwrap()
    }

    fn context(markets: Vec<(CexId, Market)>) -> SignalContext {
        let mut builder = SignalContextBuilder::new();
        for (cex, market) in markets {
            builder.cex_market(cex, AssetId::Bitcoin, AssetId::TetherUSD, market);
        }
        builder.build().unwrap()
    }

    fn execute(
        algorithm: &CostAveraging,
        signals: Signals,
        available_usdt: f64,
    ) -> Vec<ResolvedAction> {
        let actions = Rc::new(RefCell::new(Vec::new()));
        let enqueued_actions = Rc::clone(&actions);
        let mut portfolio = PortfolioBuilder::new();
        portfolio.asset_count(
            CexId::Binance,
            AccountId::new(CEX_ACCOUNT_ID),
            AssetId::TetherUSD,
            available_usdt,
        );
        let mut resolved = ResolvedContext::new(
            Box::new(move |action, _| {
                enqueued_actions.borrow_mut().push(action.clone());
                Ok(())
            }),
            portfolio.build().unwrap(),
            signals,
        );
        algorithm
            .strategy(&StrategyContext::new())
            .execute(&mut resolved)
            .unwrap();
        actions.borrow().clone()
    }

    fn assert_places_expected_order(actions: &[ResolvedAction]) {
        assert_eq!(actions.len(), 1);
        match &actions[0] {
            ResolvedAction::PlaceOrder {
                cex_id,
                account_id,
                order_request,
            } => {
                assert_eq!(*cex_id, CexId::Binance);
                assert_eq!(account_id, &AccountId::new(CEX_ACCOUNT_ID));
                match order_request {
                    OrderRequest::Limit {
                        base,
                        quote,
                        side,
                        limit_price,
                        time_in_force,
                        quantity,
                        ..
                    } => {
                        assert_eq!(*base, AssetId::Bitcoin);
                        assert_eq!(*quote, AssetId::TetherUSD);
                        assert_eq!(*side, Side::Buy);
                        assert_eq!(*time_in_force, TimeInForce::ImmediateOrCancel);
                        assert_eq!(*limit_price, ASK);
                        match quantity {
                            Quantity::OfBase(quantity) => {
                                assert_eq!(*quantity, ORDER_QUANTITY);
                            }
                            Quantity::OfQuote(_) => panic!("expected a base-asset quantity"),
                        }
                    }
                    other => panic!("expected a limit order, got {other:?}"),
                }
            }
            other => panic!("expected a place-order action, got {other:?}"),
        }
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
    fn places_an_order_when_available_quote_covers_notional_and_fees() {
        let algorithm = CostAveraging::default();
        let signals = algorithm
            .signals(&context(vec![(CexId::Binance, market(&[ASK]))]))
            .unwrap();

        assert_places_expected_order(&execute(&algorithm, signals, 1.0));
    }

    #[test]
    fn does_not_place_an_order_without_a_market() {
        let algorithm = CostAveraging::default();
        let signals = algorithm.signals(&context(vec![])).unwrap();

        assert!(execute(&algorithm, signals, 1.0).is_empty());
    }

    #[test]
    fn does_not_place_an_order_when_available_quote_covers_only_the_notional() {
        let algorithm = CostAveraging::default();
        let signals = algorithm
            .signals(&context(vec![(CexId::Binance, market(&[ASK]))]))
            .unwrap();

        assert!(execute(&algorithm, signals, ASK * ORDER_QUANTITY).is_empty());
    }

    #[test]
    fn does_not_place_an_order_when_quote_is_insufficient() {
        let algorithm = CostAveraging::default();
        let signals = algorithm
            .signals(&context(vec![(CexId::Binance, market(&[ASK]))]))
            .unwrap();

        assert!(execute(&algorithm, signals, 0.0001).is_empty());
    }
}
