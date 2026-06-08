use crate::{
    cex::{asset_id::AssetId, cex_id::CexId},
    signal::key::SignalKey,
    values::{
        asset_in_cex_value::AssetInCexValue,
        asset_total_value::AssetTotalValue,
        binary_calculation_value::{BinaryCalculationValue, BinaryOperator},
        literal_value::{
            LiteralAssetIdValue, LiteralCexIdValue, LiteralFlagValue, LiteralNumberValue,
        },
        unary_calculation_value::{UnaryCalculationValue, UnaryOperator},
        value::{AssetIdValue, CexIdValue, FlagValue, NumberValue},
    },
};

pub struct PortfolioValuesFactory;
pub struct CalculationValuesFactory;
pub struct LiteralValuesFactory;
pub struct SignalValuesFactory;

impl PortfolioValuesFactory {
    pub fn asset_in_cex(
        &self,
        cex_id_value: CexIdValue,
        asset_id_value: AssetIdValue,
    ) -> NumberValue {
        AssetInCexValue::new(cex_id_value, asset_id_value)
    }
    pub fn asset_total(&self, asset_id_value: AssetIdValue) -> NumberValue {
        AssetTotalValue::new(asset_id_value)
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
    pub fn cex_id(&self, literal: CexId) -> CexIdValue {
        LiteralCexIdValue::new(literal)
    }
    pub fn asset_id(&self, literal: AssetId) -> AssetIdValue {
        LiteralAssetIdValue::new(literal)
    }
    pub fn flag(&self, literal: bool) -> FlagValue {
        LiteralFlagValue::new(literal)
    }
    pub fn number(&self, literal: f64) -> NumberValue {
        LiteralNumberValue::new(literal)
    }
}

impl SignalValuesFactory {
    pub fn cex_id(&self, key: &SignalKey<CexId>) -> CexIdValue {
        Box::new(key.clone())
    }
    pub fn asset_id(&self, key: &SignalKey<AssetId>) -> AssetIdValue {
        Box::new(key.clone())
    }
    pub fn flag(&self, key: &SignalKey<bool>) -> FlagValue {
        Box::new(key.clone())
    }
    pub fn number(&self, key: &SignalKey<f64>) -> NumberValue {
        Box::new(key.clone())
    }
}
