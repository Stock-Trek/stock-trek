use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{NumberValue, NumberValueTrait, TokenValue},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenTotalValue {
    token_value: TokenValue,
}

impl TokenTotalValue {
    pub fn new(token_value: TokenValue) -> NumberValue {
        Box::new(Self { token_value })
    }
}

#[typetag::serde]
impl NumberValueTrait for TokenTotalValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let token = self.token_value.token(c)?;
        Ok(c.portfolio.token_total(&token))
    }
}
