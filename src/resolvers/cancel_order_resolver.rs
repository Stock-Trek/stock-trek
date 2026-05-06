// TODO
// use crate::{
//     error::{
//         result::{StockTrekError, StockTrekResult},
//         value::ValueError,
//     },
//     order::order_id::OrderId,
//     resolved_context::ResolvedContext,
//     resolvers::resolver::{Resolver, ResolverTrait},
//     values::value::ExchangeValue,
// };
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// pub struct CancelOrderResolver {
//     exchange_value: ExchangeValue,
//     order_id: OrderId,
// }

// impl CancelOrderResolver {
//     pub fn new(exchange_value: ExchangeValue, order_id: OrderId) -> Resolver {
//         Box::new(Self {
//             exchange_value,
//             order_id,
//         })
//     }
// }

// #[typetag::serde]
// impl ResolverTrait for CancelOrderResolver {
//     fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()> {
//         let exchange = self.exchange_value.exchange(c)?;
//         if let Some(exchange) = c.exchanges.get(&exchange) {
//             let cancelled = exchange.cancel_order(&c.bot_id, &self.order_id)?;
//             println!("cancelled {:?}", cancelled);
//             Ok(())
//         } else {
//             Err(StockTrekError::Value(ValueError::NotFound {
//                 name: "Exchange".to_string(),
//                 key: exchange.0,
//             }))
//         }
//     }
// }
