use crate::{
    commands::{
        command::Command, enqueue_order_command::EnqueueOrderCommand, if_command::IfCommand,
        list_command::ListCommand, no_op_command::NoOpCommand,
    },
    order::order_request::OrderRequest,
    predicates::predicate::Predicate,
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue},
};

pub struct CommandFactory;

impl CommandFactory {
    pub fn if_else(&self, condition: Predicate, if_true: Command, if_false: Command) -> Command {
        IfCommand::new(condition, if_true, if_false)
    }
    pub fn list(&self, resolvers: Vec<Command>) -> Command {
        ListCommand::new(resolvers)
    }
    pub fn no_op(&self) -> Command {
        NoOpCommand::new()
    }
    pub fn enqueue_order(
        &self,
        exchange_id_value: ExchangeIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
    ) -> Command {
        EnqueueOrderCommand::new(exchange_id_value, order_request)
    }
    // TODO
    // pub fn cancel_order(&self, exchange_id_value: ExchangeIdValue, order_id: OrderId) -> Resolver {
    //     CancelOrderResolver::new(exchange_id_value, order_id)
    // }
}
