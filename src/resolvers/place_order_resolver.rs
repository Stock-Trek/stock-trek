use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    order::order_request::OrderRequest,
    resolved_context::ResolvedContext,
    resolvers::resolver::{Resolver, ResolverTrait},
    values::value::{ExchangeValue, NumberValue, TokenValue},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlaceOrderResolver {
    exchange_value: ExchangeValue,
    order_request: OrderRequest<TokenValue, NumberValue>,
}

impl PlaceOrderResolver {
    pub fn new(
        exchange_value: ExchangeValue,
        order_request: OrderRequest<TokenValue, NumberValue>,
    ) -> Resolver {
        Box::new(Self {
            exchange_value,
            order_request,
        })
    }
}

#[typetag::serde]
impl ResolverTrait for PlaceOrderResolver {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        let exchange = self.exchange_value.exchange(c)?;
        if let Some(exchange) = c.exchanges.get(&exchange) {
            let order = exchange.place_order(&c.bot_id, &self.order_request)?;
            println!("{:?}", order);
            Ok(())
        } else {
            Err(StockTrekError::Value(ValueError::NotFound {
                name: "Exchange".to_string(),
                key: exchange.0,
            }))
        }
    }
}
