use crate::{
    scratch::key::{ExchangeName, ScratchKey, TokenName},
    values::{
        binary_calculation_value::{BinaryCalculationValue, BinaryOperator},
        literal_value::{
            LiteralExchangeValue, LiteralFlagValue, LiteralNumberValue, LiteralTokenValue,
        },
        token_in_exchange_value::TokenInExchangeValue,
        token_total_value::TokenTotalValue,
        unary_calculation_value::{UnaryCalculationValue, UnaryOperator},
        value::{ExchangeValue, FlagValue, NumberValue, TokenValue},
    },
};

pub struct PortfolioValuesFactory;
pub struct CalculationValuesFactory;
pub struct LiteralValuesFactory;
pub struct ScratchPadValuesFactory;

impl PortfolioValuesFactory {
    pub fn token_in_exchange(&self, exchange: ExchangeValue, token: TokenValue) -> NumberValue {
        TokenInExchangeValue::new(exchange, token)
    }
    pub fn token_total(&self, token: TokenValue) -> NumberValue {
        TokenTotalValue::new(token)
    }
}

impl CalculationValuesFactory {
    pub fn binary(
        &self,
        left: NumberValue,
        operator: BinaryOperator,
        right: NumberValue,
    ) -> NumberValue {
        BinaryCalculationValue::new(left, operator, right)
    }
    pub fn unary(&self, number: NumberValue, operator: UnaryOperator) -> NumberValue {
        UnaryCalculationValue::new(number, operator)
    }
}

impl LiteralValuesFactory {
    pub fn exchange(&self, literal: ExchangeName) -> ExchangeValue {
        LiteralExchangeValue::new(literal)
    }
    pub fn token(&self, literal: TokenName) -> TokenValue {
        LiteralTokenValue::new(literal)
    }
    pub fn flag(&self, literal: bool) -> FlagValue {
        LiteralFlagValue::new(literal)
    }
    pub fn number(&self, literal: f64) -> NumberValue {
        LiteralNumberValue::new(literal)
    }
}

impl ScratchPadValuesFactory {
    pub fn exchange(&self, key: &ScratchKey<ExchangeName>) -> ExchangeValue {
        Box::new(key.clone())
    }
    pub fn token(&self, key: &ScratchKey<TokenName>) -> TokenValue {
        Box::new(key.clone())
    }
    pub fn flag(&self, key: &ScratchKey<bool>) -> FlagValue {
        Box::new(key.clone())
    }
    pub fn number(&self, key: &ScratchKey<f64>) -> NumberValue {
        Box::new(key.clone())
    }
}
