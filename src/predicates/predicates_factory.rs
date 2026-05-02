use crate::{
    predicates::{
        compare_predicate::ComparePredicate,
        has_account_in_exchange_predicate::HasAccountInExchangePredicate,
        not_predicate::NotPredicate,
        owns_asset_in_exchange_predicate::OwnsAssetInExchangePredicate,
        owns_asset_predicate::OwnsAssetPredicate,
        predicate::Predicate,
        quantity_of_predicate::{QuantityOf, QuantityOfPredicate},
        scratch_pad_predicate::ScratchPadPredicate,
    },
    prelude::ScratchKey,
    values::value::NumberValue,
};
use digdigdig3::{Asset, ExchangeId};
use std::cmp::Ordering;

pub struct PredicatesFactory {}

impl PredicatesFactory {
    pub fn compare(
        &self,
        left: NumberValue,
        comparison: Ordering,
        right: NumberValue,
    ) -> Predicate {
        ComparePredicate::new(left, comparison, right)
    }
    pub fn has_account_in_exchange(&self, exchange: ExchangeId) -> Predicate {
        HasAccountInExchangePredicate::new(exchange)
    }
    pub fn not(&self, predicate: Predicate) -> Predicate {
        NotPredicate::new(predicate)
    }
    pub fn owns_asset_in_exchange(&self, asset: Asset, exchange: ExchangeId) -> Predicate {
        OwnsAssetInExchangePredicate::new(asset, exchange)
    }
    pub fn owns_asset(&self, asset: Asset) -> Predicate {
        OwnsAssetPredicate::new(asset)
    }
    pub fn quantity_of(&self, quantity_of: QuantityOf, predicates: Vec<Predicate>) -> Predicate {
        QuantityOfPredicate::new(quantity_of, predicates)
    }
    pub fn scratch_pad(&self, key: ScratchKey<bool>) -> Predicate {
        ScratchPadPredicate::new(key.key())
    }
}
