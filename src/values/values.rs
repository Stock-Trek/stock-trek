use crate::values::{
    asset_in_exchange_value::AssetInExchangeValue,
    asset_total_value::AssetTotalValue,
    binary_calculation_value::{BinaryCalculationValue, BinaryOperator},
    cash_in_exchange_value::CashInExchangeValue,
    cash_total_value::CashTotalValue,
    literal_value::{
        LiteralAssetValue, LiteralExchangeValue, LiteralFlagValue, LiteralNumberValue,
    },
    scratch_pad_value::{
        ScratchPadAssetValue, ScratchPadExchangeValue, ScratchPadFlagValue, ScratchPadNumberValue,
    },
    unary_calculation_value::{UnaryCalculationValue, UnaryOperator},
    value::{AssetValue, ExchangeValue, FlagValue, NumberValue},
};
use digdigdig3::ExchangeId;

pub struct Values {}

impl Values {
    pub fn asset_in_exchange(exchange: ExchangeValue, asset: AssetValue) -> NumberValue {
        AssetInExchangeValue::new(exchange, asset)
    }
    pub fn asset_total(asset: AssetValue) -> NumberValue {
        AssetTotalValue::new(asset)
    }
    pub fn cash_in_exchange(exchange: ExchangeValue) -> NumberValue {
        CashInExchangeValue::new(exchange)
    }
    pub fn cash_total() -> NumberValue {
        CashTotalValue::new()
    }
    pub fn binary(left: NumberValue, operator: BinaryOperator, right: NumberValue) -> NumberValue {
        BinaryCalculationValue::new(left, operator, right)
    }
    pub fn unary(number: NumberValue, operator: UnaryOperator) -> NumberValue {
        UnaryCalculationValue::new(number, operator)
    }
    pub fn asset(literal: impl AsRef<str>) -> AssetValue {
        LiteralAssetValue::new(literal.as_ref().to_string())
    }
    pub fn exchange(literal: ExchangeId) -> ExchangeValue {
        LiteralExchangeValue::new(literal)
    }
    pub fn flag(literal: bool) -> FlagValue {
        LiteralFlagValue::new(literal)
    }
    pub fn number(literal: f64) -> NumberValue {
        LiteralNumberValue::new(literal)
    }
    pub fn scratch_pad_asset(key: impl AsRef<str>) -> AssetValue {
        ScratchPadAssetValue::new(key.as_ref().to_string())
    }
    pub fn scratch_pad_exchange(key: impl AsRef<str>) -> ExchangeValue {
        ScratchPadExchangeValue::new(key.as_ref().to_string())
    }
    pub fn scratch_pad_flag(key: impl AsRef<str>) -> FlagValue {
        ScratchPadFlagValue::new(key.as_ref().to_string())
    }
    pub fn scratch_pad_number(key: impl AsRef<str>) -> NumberValue {
        ScratchPadNumberValue::new(key.as_ref().to_string())
    }
}
