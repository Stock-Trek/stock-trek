use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{ExchangeValue, NumberValue, NumberValueTrait, TokenValue},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenInExchangeValue {
    exchange_value: ExchangeValue,
    token_value: TokenValue,
}

impl TokenInExchangeValue {
    pub fn new(exchange_value: ExchangeValue, token_value: TokenValue) -> NumberValue {
        Box::new(Self {
            exchange_value,
            token_value,
        })
    }
}

#[typetag::serde]
impl NumberValueTrait for TokenInExchangeValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let exchange = self.exchange_value.exchange(c)?;
        let token = self.token_value.token(c)?;
        Ok(c.portfolio.token_in_exchange(&token, &exchange))
    }
}
