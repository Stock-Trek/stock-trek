use crate::prelude::*;
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
    fn market_calculations(&self, context: StrategyContext) -> StockTrekResult<ScratchPad> {
        let mut scratch_pad = ScratchPad::new();
        if let Some(binance) = context.exchanges.get(&ExchangeId::Binance) {
            let btc_usdt = context.symbol(BTC, USDT);
            let market_opt = binance.market_for(&btc_usdt)?;
            if let Some(market) = market_opt {
                scratch_pad.write(&self.key_market_exists, true);
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
        let market_exists = context.predicates.scratch_pad(&self.key_market_exists);
        let buy_if_possible = context.resolvers.if_else(can_buy, buy_one_satoshi, no_op);
        let do_nothing = context.resolvers.no_op();
        let resolver = context
            .resolvers
            .if_else(market_exists, buy_if_possible, do_nothing);
        Ok(resolver)
    }
}
