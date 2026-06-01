use crate::{
    asset_id::AssetId,
    conditions::{
        compare_condition::CompareCondition,
        condition::Condition,
        has_account_in_exchange_condition::HasAccountInExchangeCondition,
        not_condition::NotCondition,
        owns_asset_condition::OwnsAssetCondition,
        owns_asset_in_exchange_condition::OwnsAssetInExchangeCondition,
        quantity_of_condition::{QuantityOf, QuantityOfCondition},
    },
    exchange_id::ExchangeId,
    signal::key::SignalKey,
    values::value::NumberValue,
};
use std::cmp::Ordering;

pub struct ConditionFactory;

impl ConditionFactory {
    pub fn compare(
        &self,
        left: NumberValue,
        comparison: Ordering,
        right: NumberValue,
    ) -> Condition {
        CompareCondition::new(left, comparison, right)
    }
    pub fn has_account_in_exchange(&self, exchange_id: ExchangeId) -> Condition {
        HasAccountInExchangeCondition::new(exchange_id)
    }
    pub fn not(&self, condition: Condition) -> Condition {
        NotCondition::new(condition)
    }
    pub fn owns_asset(&self, asset_id: AssetId) -> Condition {
        OwnsAssetCondition::new(asset_id)
    }
    pub fn owns_asset_in_exchange(&self, asset_id: AssetId, exchange_id: ExchangeId) -> Condition {
        OwnsAssetInExchangeCondition::new(asset_id, exchange_id)
    }
    pub fn quantity_of(&self, quantity_of: QuantityOf, conditions: Vec<Condition>) -> Condition {
        QuantityOfCondition::new(quantity_of, conditions)
    }
    pub fn signal(&self, key: &SignalKey<bool>) -> Condition {
        Box::new(key.clone())
    }
}
