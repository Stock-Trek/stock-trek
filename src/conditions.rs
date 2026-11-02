use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    util::serde_ordering::OrderingSerde,
    value::value::NumberValue,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};
use strum::Display;

/// A condition that can be evaluated against a resolved context.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[strum(serialize_all = "snake_case")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Condition {
    /// Compares two numeric values.
    Compare {
        left: Box<NumberValue>,
        #[serde(with = "OrderingSerde")]
        comparison: Ordering,
        right: Box<NumberValue>,
    },
    /// Tests whether a CEX account exists.
    HasCexAccount {
        cex_id: CexId,
        account_id: AccountId,
    },
    /// Negates another condition.
    Not { condition: Box<Condition> },
    /// Tests whether the portfolio owns any of an asset.
    OwnsAsset { asset_id: AssetId },
    /// Tests whether a CEX account owns any of an asset.
    OwnsAssetInCexAccount {
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
    },
    /// Applies a quantity requirement to a list of conditions.
    QuantityOf {
        quantity_of: QuantityOf,
        conditions: Vec<Condition>,
    },
    /// Tests the value of a boolean signal.
    Signal { signal: SignalKey<bool> },
}

impl Condition {
    /// Evaluates the condition against the resolved context.
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
                .asset_total_in_cex_account(cex_id, account_id, asset_id)
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

/// Specifies how many of a set of conditions must be true.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum QuantityOf {
    /// All conditions must be true.
    All,
    /// At least one and not all conditions must be true.
    Partial,
    /// No conditions may be true.
    None,
    /// The condition list must be empty.
    Empty,
}

/// Factory for constructing conditions.
#[derive(Debug, Clone, Default)]
pub struct ConditionFactory;

#[allow(missing_docs)]
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
        cex_id: CexId,
        account_id: AccountId,
        asset_id: AssetId,
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
    fn compare_serializes_ordering_as_snake_case() {
        let condition = ConditionFactory.compare(
            NumberValue::Literal { literal: 1.0 },
            Ordering::Less,
            NumberValue::Literal { literal: 2.0 },
        );

        let value = serde_json::to_value(&condition).unwrap();
        assert_eq!(value["comparison"], "less");

        let deserialized: Condition = serde_json::from_value(value).unwrap();
        assert!(deserialized.test(&context()).unwrap());
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
