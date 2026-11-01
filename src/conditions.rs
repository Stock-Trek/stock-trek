use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    util::serde_ordering,
    value::value::NumberValue,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Condition {
    Compare {
        left: Box<NumberValue>,
        #[serde(with = "serde_ordering")]
        comparison: Ordering,
        right: Box<NumberValue>,
    },
    HasCexAccount {
        cex_id: CexId,
        account_id: AccountId,
    },
    Not {
        condition: Box<Condition>,
    },
    OwnsAsset {
        asset_id: AssetId,
    },
    OwnsAssetInCexAccount {
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
    },
    QuantityOf {
        quantity_of: QuantityOf,
        conditions: Vec<Condition>,
    },
    Signal {
        signal: SignalKey<bool>,
    },
}

impl Condition {
    pub fn test(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        match self {
            Condition::Compare {
                left,
                comparison,
                right,
            } => {
                let left_value = left.number(c)?;
                let right_value = right.number(c)?;
                let ordering = left_value.partial_cmp(&right_value);
                match ordering {
                    Some(ordering) => Ok(ordering == *comparison),
                    None => Err(StockTrekError::Stats(StatsError::IncomparableValues {
                        left: left_value,
                        right: right_value,
                    })),
                }
            }
            Condition::HasCexAccount { cex_id, account_id } => {
                Ok(c.portfolio.has_cex_account(cex_id, account_id))
            }
            Condition::Not { condition } => {
                let test_result = condition.test(c)?;
                Ok(!test_result)
            }
            Condition::OwnsAsset { asset_id } => {
                Ok(c.portfolio.asset_total(asset_id) > Decimal::ZERO)
            }
            Condition::OwnsAssetInCexAccount {
                cex_id,
                account_id,
                asset_id,
            } => Ok(c
                .portfolio
                .asset_total_in_cex_account(asset_id, cex_id, account_id)
                > Decimal::ZERO),
            Condition::QuantityOf {
                quantity_of,
                conditions,
            } => {
                if conditions.is_empty() {
                    let empty_result = match quantity_of {
                        QuantityOf::All => true,
                        QuantityOf::Partial => false,
                        QuantityOf::None => true,
                        QuantityOf::Empty => true,
                    };
                    return Ok(empty_result);
                }
                let mut true_count = 0;
                let mut false_count = 0;
                for condition in conditions {
                    if condition.test(c)? {
                        true_count += 1;
                    } else {
                        false_count += 1;
                    }
                }
                let quantity = match quantity_of {
                    QuantityOf::All => false_count == 0,
                    QuantityOf::Partial => (true_count > 0) && (false_count > 0),
                    QuantityOf::None => true_count == 0,
                    QuantityOf::Empty => (true_count == 0) && (false_count == 0),
                };
                Ok(quantity)
            }
            Condition::Signal { signal } => c.signals.read(signal),
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum QuantityOf {
    All,
    Partial,
    None,
    Empty,
}

#[derive(Debug, Clone)]
pub struct ConditionFactory;

impl ConditionFactory {
    pub fn compare(
        &self,
        left: NumberValue,
        comparison: Ordering,
        right: NumberValue,
    ) -> Condition {
        Condition::Compare {
            left: Box::new(left),
            comparison,
            right: Box::new(right),
        }
    }
    pub fn has_cex_account(&self, cex_id: CexId, account_id: AccountId) -> Condition {
        Condition::HasCexAccount { cex_id, account_id }
    }
    pub fn not(&self, condition: Condition) -> Condition {
        Condition::Not {
            condition: Box::new(condition),
        }
    }
    pub fn owns_asset(&self, asset_id: AssetId) -> Condition {
        Condition::OwnsAsset { asset_id }
    }
    pub fn owns_asset_in_cex_account(
        &self,
        asset_id: AssetId,
        cex_id: CexId,
        account_id: AccountId,
    ) -> Condition {
        Condition::OwnsAssetInCexAccount {
            cex_id,
            account_id,
            asset_id,
        }
    }
    pub fn quantity_of(&self, quantity_of: QuantityOf, conditions: Vec<Condition>) -> Condition {
        Condition::QuantityOf {
            quantity_of,
            conditions,
        }
    }
    pub fn signal(&self, flag: &SignalKey<bool>) -> Condition {
        Condition::Signal {
            signal: flag.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{portfolio::Portfolio, signal::signals::Signals};
    use std::collections::HashMap;

    fn context() -> ResolvedContext {
        ResolvedContext {
            enqueue_action: Box::new(|_, _| Ok(())),
            portfolio: Portfolio::new(HashMap::new()),
            signals: Signals::new(),
        }
    }

    fn compare(left: f64, comparison: Ordering, right: f64) -> bool {
        ConditionFactory
            .compare(
                NumberValue::Literal { literal: left },
                comparison,
                NumberValue::Literal { literal: right },
            )
            .test(&context())
            .unwrap()
    }

    #[test]
    fn compare_matches_the_actual_ordering() {
        assert!(compare(1.0, Ordering::Less, 2.0));
        assert!(!compare(2.0, Ordering::Less, 1.0));
        assert!(!compare(1.0, Ordering::Less, 1.0));

        assert!(compare(2.0, Ordering::Equal, 2.0));
        assert!(!compare(2.0, Ordering::Equal, 1.0));
        assert!(!compare(1.0, Ordering::Equal, 2.0));

        assert!(compare(2.0, Ordering::Greater, 1.0));
        assert!(!compare(1.0, Ordering::Greater, 2.0));
        assert!(!compare(1.0, Ordering::Greater, 1.0));
    }
}
