use crate::{
    error::result::StockTrekResult,
    exchanges::bot_id::BotId,
    order::{order_request::OrderRequest, order_response::OrderResponse},
    values::value::{NumberValue, TokenValue},
};

pub type Exchange = Box<dyn ExchangeTrait>;

pub trait ExchangeTrait: Send + Sync {
    fn place_order(
        &self,
        bot_id: &BotId,
        request: &OrderRequest<TokenValue, NumberValue>,
    ) -> StockTrekResult<OrderResponse>;
    // TODO
    // fn cancel_order(&self, bot_id: &BotId, order_id: &OrderId) -> StockTrekResult<bool>;
}
