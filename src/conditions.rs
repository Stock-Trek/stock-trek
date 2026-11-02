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
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};
use strum::Display;

/// A condition that can be evaluated against a resolved context.
///
/// Conditions read signals and portfolio state; they do not mutate anything.
/// Build them with [`ConditionFactory`] and combine them with
/// [`QuantityOf`] when a strategy needs "all", "none", or "some" semantics.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Condition {
    /// Compares two numeric values.
    Compare {
        /// The left-hand value.
        left: Box<NumberValue>,
        /// The comparison to apply.
        #[serde(with = "OrderingSerde")]
        comparison: Ordering,
        /// The right-hand value.
        right: Box<NumberValue>,
    },
    /// Tests whether a CEX account exists in the portfolio.
    HasCexAccount {
        /// The CEX to check.
        cex_id: CexId,
        /// The account to check within the CEX.
        account_id: AccountId,
    },
    /// Negates another condition.
    Not {
        /// The condition to negate.
        condition: Box<Condition>,
    },
    /// Tests whether the portfolio owns any of an asset.
    OwnsAsset {
        /// The asset to check for across all accounts.
        asset_id: AssetId,
    },
    /// Tests whether a CEX account owns any of an asset.
    OwnsAssetInCexAccount {
        /// The CEX to check.
        cex_id: CexId,
        /// The account to check within the CEX.
        account_id: AccountId,
        /// The asset to check for in the account.
        asset_id: AssetId,
    },
    /// Applies a quantity requirement to a list of conditions.
    QuantityOf {
        /// How many of the nested conditions must be true.
        quantity_of: QuantityOf,
        /// The conditions to evaluate.
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
    ///
    /// Returns an error when a referenced signal is missing or has the wrong
    /// type, or when numeric values cannot be compared (for example, NaN).
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
///
/// This is used by [`Condition::QuantityOf`] to express group requirements
/// without nesting many `Not` conditions.
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

impl ConditionFactory {
    /// Compares two values using `comparison`.
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
    /// Tests whether a CEX account exists in the portfolio.
    pub fn has_cex_account(&self, cex_id: CexId, account_id: AccountId) -> Condition {
        Condition::HasCexAccount { cex_id, account_id }
    }
    /// Negates a condition.
    pub fn not(&self, condition: Condition) -> Condition {
        Condition::Not {
            condition: Box::new(condition),
        }
    }
    /// Tests whether any account holds the asset.
    pub fn owns_asset(&self, asset_id: AssetId) -> Condition {
        Condition::OwnsAsset { asset_id }
    }
    /// Tests whether a specific CEX account holds the asset.
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
    /// Requires a specific number of the given conditions to be true.
    pub fn quantity_of(&self, quantity_of: QuantityOf, conditions: Vec<Condition>) -> Condition {
        Condition::QuantityOf {
            quantity_of,
            conditions,
        }
    }
    /// Tests the value of a boolean signal.
    pub fn signal(&self, flag: &SignalKey<bool>) -> Condition {
        Condition::Signal {
            signal: flag.clone(),
        }
    }
}

#[cfg(test)]
mod tests;
