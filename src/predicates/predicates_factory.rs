use crate::{
    asset_id::AssetId,
    exchange_id::ExchangeId,
    predicates::{
        compare_predicate::ComparePredicate,
        has_account_in_exchange_predicate::HasAccountInExchangePredicate,
        not_predicate::NotPredicate,
        owns_asset_in_exchange_predicate::OwnsAssetInExchangePredicate,
        owns_asset_predicate::OwnsAssetPredicate,
        predicate::Predicate,
        quantity_of_predicate::{QuantityOf, QuantityOfPredicate},
    },
    signal::key::SignalKey,
    values::value::NumberValue,
};
use std::cmp::Ordering;

pub struct PredicatesFactory;

impl PredicatesFactory {
    pub fn compare(
        &self,
        left: NumberValue,
        comparison: Ordering,
        right: NumberValue,
    ) -> Predicate {
        ComparePredicate::new(left, comparison, right)
    }
    pub fn has_account_in_exchange(&self, exchange_id: ExchangeId) -> Predicate {
        HasAccountInExchangePredicate::new(exchange_id)
    }
    pub fn not(&self, predicate: Predicate) -> Predicate {
        NotPredicate::new(predicate)
    }
    pub fn owns_asset(&self, asset_id: AssetId) -> Predicate {
        OwnsAssetPredicate::new(asset_id)
    }
    pub fn owns_asset_in_exchange(&self, asset_id: AssetId, exchange_id: ExchangeId) -> Predicate {
        OwnsAssetInExchangePredicate::new(asset_id, exchange_id)
    }
    pub fn quantity_of(&self, quantity_of: QuantityOf, predicates: Vec<Predicate>) -> Predicate {
        QuantityOfPredicate::new(quantity_of, predicates)
    }
    pub fn signal(&self, key: &SignalKey<bool>) -> Predicate {
        Box::new(key.clone())
    }
}
