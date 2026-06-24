use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

use crate::{
    conditions::{
        compare_condition::CompareCondition,
        condition::Condition,
        has_account_in_cex_condition::HasAccountInCexCondition,
        not_condition::NotCondition,
        owns_asset_condition::OwnsAssetCondition,
        owns_asset_in_cex_condition::OwnsAssetInCexCondition,
        quantity_of_condition::{QuantityOf, QuantityOfCondition},
    },
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
    pub fn has_account_in_cex(&self, cex_id: CexId) -> Condition {
        HasAccountInCexCondition::new(cex_id)
    }
    pub fn not(&self, condition: Condition) -> Condition {
        NotCondition::new(condition)
    }
    pub fn owns_asset(&self, asset_id: AssetId) -> Condition {
        OwnsAssetCondition::new(asset_id)
    }
    pub fn owns_asset_in_cex(&self, asset_id: AssetId, cex_id: CexId) -> Condition {
        OwnsAssetInCexCondition::new(asset_id, cex_id)
    }
    pub fn quantity_of(&self, quantity_of: QuantityOf, conditions: Vec<Condition>) -> Condition {
        QuantityOfCondition::new(quantity_of, conditions)
    }
    pub fn signal(&self, key: &SignalKey<bool>) -> Condition {
        Box::new(key.clone())
    }
}
