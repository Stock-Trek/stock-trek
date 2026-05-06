use crate::{
    error::result::StockTrekResult,
    exchanges::{
        bot_id::BotId,
        exchange::{Exchange, ExchangeTrait},
    },
    order::{
        client_order_id::ClientOrderId, order_request::OrderRequest, order_response::OrderResponse,
    },
    values::value::{NumberValue, TokenValue},
};
use uuid::Uuid;

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
        order_request: &OrderRequest<TokenValue, NumberValue>,
    ) -> StockTrekResult<OrderResponse> {
        let client_order_id = ClientOrderId::create(order_request);
        let response = OrderResponse {
            id: Uuid::new_v4().to_string(),
            client_order_ids: vec![client_order_id],
        };
        Ok(response)
    }
}
