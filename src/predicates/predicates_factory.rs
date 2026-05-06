use crate::{
    predicates::{
        compare_predicate::ComparePredicate,
        has_account_in_exchange_predicate::HasAccountInExchangePredicate,
        not_predicate::NotPredicate,
        owns_token_in_exchange_predicate::OwnsTokenInExchangePredicate,
        owns_token_predicate::OwnsTokenPredicate,
        predicate::Predicate,
        quantity_of_predicate::{QuantityOf, QuantityOfPredicate},
    },
    scratch::key::{ExchangeName, ScratchKey, TokenName},
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
    pub fn has_account_in_exchange(&self, exchange: ExchangeName) -> Predicate {
        HasAccountInExchangePredicate::new(exchange)
    }
    pub fn not(&self, predicate: Predicate) -> Predicate {
        NotPredicate::new(predicate)
    }
    pub fn owns_token(&self, token: TokenName) -> Predicate {
        OwnsTokenPredicate::new(token)
    }
    pub fn owns_token_in_exchange(&self, token: TokenName, exchange: ExchangeName) -> Predicate {
        OwnsTokenInExchangePredicate::new(token, exchange)
    }
    pub fn quantity_of(&self, quantity_of: QuantityOf, predicates: Vec<Predicate>) -> Predicate {
        QuantityOfPredicate::new(quantity_of, predicates)
    }
    pub fn scratch_pad(&self, key: &ScratchKey<bool>) -> Predicate {
        Box::new(key.clone())
    }
}
