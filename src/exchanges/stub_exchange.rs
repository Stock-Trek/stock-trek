use crate::{
    error::result::StockTrekResult,
    exchanges::{
        bot_id::BotId,
        exchange::{Exchange, ExchangeTrait},
    },
    order::{order_request::OrderRequest, order_response::OrderResponse},
    values::value::{NumberValue, TokenValue},
};

pub struct StubExchange;

impl From<StubExchange> for Exchange {
    fn from(value: StubExchange) -> Self {
        Box::new(value)
    }
}

impl ExchangeTrait for StubExchange {
    fn place_order(
        &self,
        _bot_id: &BotId,
        _order_request: &OrderRequest<TokenValue, NumberValue>,
    ) -> StockTrekResult<OrderResponse> {
        let response = OrderResponse {
            // TODO
            // id: OrderId(Uuid::new_v4().to_string()),
            // client_order_id: ClientOrderId::create(order_request),
        };
        Ok(response)
    }
    // TODO
    // fn cancel_order(&self, _bot_id: &BotId, _order_id: &OrderId) -> StockTrekResult<bool> {
    //     Ok(true)
    // }
}
