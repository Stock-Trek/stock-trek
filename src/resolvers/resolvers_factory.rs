use crate::{
    order::order_request::OrderRequest,
    predicates::predicate::Predicate,
    resolvers::{
        if_resolver::IfResolver, list_resolver::ListResolver, no_op_resolver::NoOpResolver,
        place_order_resolver::PlaceOrderResolver, resolver::Resolver,
    },
    values::value::{ExchangeValue, NumberValue, TokenValue},
};

pub struct ResolversFactory;

impl ResolversFactory {
    pub fn if_else(&self, condition: Predicate, if_true: Resolver, if_false: Resolver) -> Resolver {
        IfResolver::new(condition, if_true, if_false)
    }
    pub fn list(&self, resolvers: Vec<Resolver>) -> Resolver {
        ListResolver::new(resolvers)
    }
    pub fn no_op(&self) -> Resolver {
        NoOpResolver::new()
    }
    pub fn place_order(
        &self,
        exchange_value: ExchangeValue,
        order_request: OrderRequest<TokenValue, NumberValue>,
    ) -> Resolver {
        PlaceOrderResolver::new(exchange_value, order_request)
    }
    // TODO
    // pub fn cancel_order(&self, exchange_value: ExchangeValue, order_id: OrderId) -> Resolver {
    //     CancelOrderResolver::new(exchange_value, order_id)
    // }
}
