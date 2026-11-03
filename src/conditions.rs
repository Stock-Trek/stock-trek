use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    util::serde_ordering::OrderingSerde,
    value::value_types::NumberValue,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};
use strum::Display;

/// A condition that can be evaluated against a resolved context.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Condition {
    /// Compares two numeric values.
    Compare {
        /// The left-hand operand.
        left: Box<NumberValue>,
        /// The comparison applied to the operands.
        #[serde(with = "OrderingSerde")]
        comparison: Ordering,
        /// The right-hand operand.
        right: Box<NumberValue>,
    },
    /// Tests whether a CEX account exists.
    HasCexAccount {
        /// The CEX the account is on.
        cex_id: CexId,
        /// The account to test for.
        account_id: AccountId,
    },
    /// Negates another condition.
    Not {
        /// The condition to negate.
        condition: Box<Condition>,
    },
    /// Tests whether the portfolio owns any of an asset.
    OwnsAsset {
        /// The asset to test for.
        asset_id: AssetId,
    },
    /// Tests whether a CEX account owns any of an asset.
    OwnsAssetInCexAccount {
        /// The CEX the account is on.
        cex_id: CexId,
        /// The account to test.
        account_id: AccountId,
        /// The asset to test for.
        asset_id: AssetId,
    },
    /// Applies a quantity requirement to a list of conditions.
    QuantityOf {
        /// How many of the conditions must hold.
        quantity_of: QuantityOf,
        /// The conditions combined by the requirement.
        conditions: Vec<Condition>,
    },
    /// Tests the value of a boolean signal.
    Signal {
        /// The signal to read.
        signal: SignalKey<bool>,
    },
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
            Condition::OwnsAsset { asset_id } => Ok(c.portfolio.asset_total(asset_id) > 0.0),
            Condition::OwnsAssetInCexAccount {
                cex_id,
                account_id,
                asset_id,
            } => Ok(c
                .portfolio
                .asset_total_in_cex_account(cex_id, account_id, asset_id)
                > 0.0),
            Condition::QuantityOf {
                quantity_of,
                conditions,
            } => match quantity_of {
                QuantityOf::All => {
                    for condition in conditions {
                        if !condition.test(c)? {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                QuantityOf::AtLeastOne => {
                    for condition in conditions {
                        if condition.test(c)? {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
                QuantityOf::Partial => {
                    let mut has_true = false;
                    let mut has_false = false;
                    for condition in conditions {
                        if condition.test(c)? {
                            has_true = true;
                        } else {
                            has_false = true;
                        }
                        if has_true && has_false {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
                QuantityOf::None => {
                    for condition in conditions {
                        if condition.test(c)? {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                QuantityOf::Empty => Ok(conditions.is_empty()),
            },
            Condition::Signal { signal } => c.signals.read(signal),
        }
    }
}

/// Specifies how many of a set of conditions must be true.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum QuantityOf {
    /// All conditions must be true.
    All,
    /// At least one condition must be true.
    AtLeastOne,
    /// At least one and not all conditions must be true.
    Partial,
    /// No conditions may be true.
    None,
    /// The condition list must be empty.
    Empty,
}

/// Factory for constructing conditions.
///
/// Available as the `conditions` field of a [`StrategyContext`](crate::StrategyContext).
#[derive(Debug, Clone, Default)]
pub struct ConditionFactory;

impl ConditionFactory {
    /// Compares two numeric values with `comparison`, erroring when either
    /// value is NaN so the comparison has no result.
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
    /// Whether the CEX account exists in the portfolio.
    pub fn has_cex_account(&self, cex_id: CexId, account_id: AccountId) -> Condition {
        Condition::HasCexAccount { cex_id, account_id }
    }
    /// Negates `condition`.
    pub fn not(&self, condition: Condition) -> Condition {
        Condition::Not {
            condition: Box::new(condition),
        }
    }
    /// Whether the portfolio holds any amount of the asset, in any account.
    pub fn owns_asset(&self, asset_id: AssetId) -> Condition {
        Condition::OwnsAsset { asset_id }
    }
    /// Whether a specific CEX account holds any amount of the asset.
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
    /// Combines conditions with a [`QuantityOf`] requirement.
    pub fn quantity_of(&self, quantity_of: QuantityOf, conditions: Vec<Condition>) -> Condition {
        Condition::QuantityOf {
            quantity_of,
            conditions,
        }
    }
    /// Whether the boolean signal currently holds true.
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

    fn context() -> ResolvedContext {
        ResolvedContext {
            enqueue_action: Box::new(|_, _| Ok(())),
            portfolio: Portfolio::default(),
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

    fn literal_condition(value: bool) -> Condition {
        if value {
            ConditionFactory.compare(
                NumberValue::Literal { literal: 1.0 },
                Ordering::Less,
                NumberValue::Literal { literal: 2.0 },
            )
        } else {
            ConditionFactory.compare(
                NumberValue::Literal { literal: 2.0 },
                Ordering::Less,
                NumberValue::Literal { literal: 1.0 },
            )
        }
    }

    fn failing_condition() -> Condition {
        ConditionFactory.compare(
            NumberValue::Literal { literal: f64::NAN },
            Ordering::Equal,
            NumberValue::Literal { literal: 0.0 },
        )
    }

    fn quantity_of(quantity_of: QuantityOf, conditions: Vec<Condition>) -> StockTrekResult<bool> {
        ConditionFactory
            .quantity_of(quantity_of, conditions)
            .test(&context())
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

    #[test]
    fn quantity_of_all_short_circuits_after_false() {
        let conditions = vec![literal_condition(false), failing_condition()];

        assert!(!quantity_of(QuantityOf::All, conditions).unwrap());
    }

    #[test]
    fn quantity_of_none_short_circuits_after_true() {
        let conditions = vec![literal_condition(true), failing_condition()];

        assert!(!quantity_of(QuantityOf::None, conditions).unwrap());
    }

    #[test]
    fn quantity_of_at_least_one_short_circuits_after_true() {
        let conditions = vec![literal_condition(true), failing_condition()];

        assert!(quantity_of(QuantityOf::AtLeastOne, conditions).unwrap());
    }

    #[test]
    fn quantity_of_partial_short_circuits_after_true_and_false() {
        let conditions = vec![
            literal_condition(true),
            literal_condition(false),
            failing_condition(),
        ];

        assert!(quantity_of(QuantityOf::Partial, conditions).unwrap());
    }

    #[test]
    fn quantity_of_empty_does_not_evaluate_conditions() {
        assert!(quantity_of(QuantityOf::Empty, vec![]).unwrap());
        assert!(!quantity_of(QuantityOf::Empty, vec![failing_condition()]).unwrap());
    }

    #[test]
    fn quantity_of_propagates_errors_when_the_result_is_not_determined() {
        assert!(quantity_of(QuantityOf::All, vec![failing_condition()]).is_err());
        assert!(quantity_of(QuantityOf::None, vec![failing_condition()]).is_err());
        assert!(quantity_of(QuantityOf::AtLeastOne, vec![failing_condition()]).is_err());
        assert!(quantity_of(QuantityOf::Partial, vec![failing_condition()]).is_err());
    }

    #[test]
    fn quantity_of_returns_the_expected_result() {
        assert!(
            quantity_of(
                QuantityOf::All,
                vec![literal_condition(true), literal_condition(true)]
            )
            .unwrap()
        );
        assert!(
            !quantity_of(
                QuantityOf::All,
                vec![literal_condition(true), literal_condition(false)]
            )
            .unwrap()
        );
        assert!(
            quantity_of(
                QuantityOf::AtLeastOne,
                vec![literal_condition(true), literal_condition(false)]
            )
            .unwrap()
        );
        assert!(
            quantity_of(
                QuantityOf::AtLeastOne,
                vec![literal_condition(true), literal_condition(true)]
            )
            .unwrap()
        );
        assert!(
            !quantity_of(
                QuantityOf::AtLeastOne,
                vec![literal_condition(false), literal_condition(false)]
            )
            .unwrap()
        );
        assert!(!quantity_of(QuantityOf::AtLeastOne, vec![]).unwrap());
        assert!(
            quantity_of(
                QuantityOf::Partial,
                vec![literal_condition(true), literal_condition(false)]
            )
            .unwrap()
        );
        assert!(
            !quantity_of(
                QuantityOf::Partial,
                vec![literal_condition(true), literal_condition(true)]
            )
            .unwrap()
        );
        assert!(
            quantity_of(
                QuantityOf::None,
                vec![literal_condition(false), literal_condition(false)]
            )
            .unwrap()
        );
    }
}
