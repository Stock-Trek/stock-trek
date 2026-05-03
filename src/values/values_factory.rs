use crate::{
    scratch::key::ScratchKey,
    values::{
        asset_in_exchange_value::AssetInExchangeValue,
        asset_total_value::AssetTotalValue,
        binary_calculation_value::{BinaryCalculationValue, BinaryOperator},
        literal_value::{
            LiteralAssetValue, LiteralExchangeValue, LiteralFlagValue, LiteralNumberValue,
        },
        unary_calculation_value::{UnaryCalculationValue, UnaryOperator},
        value::{AssetValue, ExchangeValue, FlagValue, NumberValue},
    },
};
use digdigdig3::{Asset, ExchangeId};

pub struct PortfolioValuesFactory;
pub struct CalculationValuesFactory;
pub struct LiteralValuesFactory;
pub struct ScratchPadValuesFactory;

impl PortfolioValuesFactory {
    pub fn asset_in_exchange(&self, exchange: ExchangeValue, asset: AssetValue) -> NumberValue {
        AssetInExchangeValue::new(exchange, asset)
    }
    pub fn asset_total(&self, asset: AssetValue) -> NumberValue {
        AssetTotalValue::new(asset)
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
    pub fn asset(&self, literal: impl AsRef<str>) -> AssetValue {
        LiteralAssetValue::new(literal.as_ref().to_string())
    }
    pub fn exchange(&self, literal: ExchangeId) -> ExchangeValue {
        LiteralExchangeValue::new(literal)
    }
    pub fn flag(&self, literal: bool) -> FlagValue {
        LiteralFlagValue::new(literal)
    }
    pub fn number(&self, literal: f64) -> NumberValue {
        LiteralNumberValue::new(literal)
    }
}

impl ScratchPadValuesFactory {
    pub fn asset(&self, key: &ScratchKey<Asset>) -> AssetValue {
        Box::new(key.clone())
    }
    pub fn exchange(&self, key: &ScratchKey<ExchangeId>) -> ExchangeValue {
        Box::new(key.clone())
    }
    pub fn flag(&self, key: &ScratchKey<bool>) -> FlagValue {
        Box::new(key.clone())
    }
    pub fn number(&self, key: &ScratchKey<f64>) -> NumberValue {
        Box::new(key.clone())
    }
}
