use crate::{
    cex::{asset_id::AssetId, cex_id::CexId, order_tag::OrderTag},
    signal::key::SignalKey,
    values::{
        active_orders_in_cex_value::ActiveOrdersInCexValue,
        active_orders_in_cex_with_tag_value::ActiveOrdersInCexWithTagValue,
        active_orders_value::ActiveOrdersValue,
        active_orders_with_tag_value::ActiveOrdersWithTagValue,
        allocation_for_asset_in_cex_value::AllocationForAssetInCexValue,
        allocation_for_asset_total_value::AllocationForAssetTotalValue,
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

pub struct AllocationValuesFactory;
pub struct PortfolioValuesFactory;
pub struct CalculationValuesFactory;
pub struct LiteralValuesFactory;
pub struct SignalValuesFactory;

impl AllocationValuesFactory {
    pub fn allocation_for_asset_in_cex(
        &self,
        cex_id_value: CexIdValue,
        asset_id_value: AssetIdValue,
    ) -> NumberValue {
        AllocationForAssetInCexValue::new(cex_id_value, asset_id_value)
    }
    pub fn asset_total(&self, asset_id_value: AssetIdValue) -> NumberValue {
        AllocationForAssetTotalValue::new(asset_id_value)
    }
}

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
    pub fn active_orders_in_cex(&self, cex_id_value: CexIdValue) -> NumberValue {
        ActiveOrdersInCexValue::new(cex_id_value)
    }
    pub fn active_orders_in_cex_with_tag(
        &self,
        cex_id_value: CexIdValue,
        order_tag: OrderTag,
    ) -> NumberValue {
        ActiveOrdersInCexWithTagValue::new(cex_id_value, order_tag)
    }
    pub fn active_orders(&self) -> NumberValue {
        ActiveOrdersValue::new()
    }
    pub fn active_orders_with_tag(&self, order_tag: OrderTag) -> NumberValue {
        ActiveOrdersWithTagValue::new(order_tag)
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
