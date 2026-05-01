use crate::prelude::*;
use digdigdig3::{
    core::{OrderRequest, TimeInForce},
    AccountType, ExchangeId, OrderSide, OrderType, Symbol,
};
use std::cmp::Ordering;
use strum::{Display, EnumString};

const BTC: &str = "BTC";
const USDT: &str = "USDT";

#[derive(Display, EnumString)]
pub enum ScratchPadValues {
    SatoshiPrice,
}

pub struct Dca {}

impl Default for Dca {
    fn default() -> Self {
        Self {}
    }
}

#[register_strategy(default)]
impl Strategy for Dca {
    fn market_calculations(&self, context: StrategyContext) -> anyhow::Result<ScratchPad> {
        let mut scratch_pad = ScratchPad::new();
        if let Some(binance) = context.exchanges.get(&ExchangeId::Binance) {
            let btc_usdt = Symbol::new(BTC, USDT);
            let market_opt = binance.market_for(&btc_usdt)?;
            match market_opt {
                Some(market) => {
                    let satoshi_price = market.ticks.ticks[0].last.price / 1_000_000.0;
                    scratch_pad.write(
                        ScratchPadValues::SatoshiPrice.to_string(),
                        ScratchValue::Number(satoshi_price),
                    );
                }
                None => {}
            }
        }
        Ok(scratch_pad)
    }
    fn portfolio_response(&self) -> anyhow::Result<Assembler> {
        let exchange = ExchangeId::Binance;
        let satoshi_price = Values::scratch_pad_number(ScratchPadValues::SatoshiPrice.to_string());
        let usdt = Values::asset_in_exchange(Values::exchange(exchange), Values::asset(USDT));
        let can_buy = Predicates::compare(usdt, Ordering::Greater, satoshi_price);
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
        let buy_one_satoshi = Assemblers::action(Actions::order_request(exchange, order_request));
        let no_op = Assemblers::no_op();
        let assembler = Assemblers::if_(can_buy, buy_one_satoshi, no_op);
        Ok(assembler)
    }
}
