use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    validation::Validation,
    value::{binary_operator::BinaryOperator, unary_operator::UnaryOperator},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};
use strum::Display;

/// A value that resolves to a CEX account identifier.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AccountIdValue {
    /// A literal account identifier.
    Literal {
        /// The account identifier.
        literal: AccountId,
    },
    /// An account identifier read from a signal.
    Signal {
        /// The key of the signal holding the account identifier.
        signal: SignalKey<AccountId>,
    },
}

impl AccountIdValue {
    /// Resolves the account identifier against the resolved context.
    pub fn account_id(&self, c: &ResolvedContext) -> StockTrekResult<AccountId> {
        match self {
            Self::Literal { literal } => Ok(literal.clone()),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

/// A value that resolves to a CEX identifier.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CexIdValue {
    /// A literal CEX identifier.
    Literal {
        /// The CEX identifier.
        literal: CexId,
    },
    /// A CEX identifier read from a signal.
    Signal {
        /// The key of the signal holding the CEX identifier.
        signal: SignalKey<CexId>,
    },
}

impl CexIdValue {
    /// Resolves the CEX identifier against the resolved context.
    pub fn cex_id(&self, c: &ResolvedContext) -> StockTrekResult<CexId> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

/// A value that resolves to an asset identifier.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AssetIdValue {
    /// A literal asset identifier.
    Literal {
        /// The asset identifier.
        literal: AssetId,
    },
    /// An asset identifier read from a signal.
    Signal {
        /// The key of the signal holding the asset identifier.
        signal: SignalKey<AssetId>,
    },
}

impl AssetIdValue {
    /// Resolves the asset identifier against the resolved context.
    pub fn asset_id(&self, c: &ResolvedContext) -> StockTrekResult<AssetId> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

/// A value that resolves to a boolean flag.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum FlagValue {
    /// A literal boolean flag.
    Literal {
        /// The flag.
        literal: bool,
    },
    /// A boolean flag read from a signal.
    Signal {
        /// The key of the signal holding the flag.
        signal: SignalKey<bool>,
    },
}

impl FlagValue {
    /// Resolves the flag against the resolved context.
    pub fn flag(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

/// A value that resolves to a number.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum NumberValue {
    /// A literal number.
    Literal {
        /// The number.
        literal: f64,
    },
    /// A number read from a signal.
    Signal {
        /// The key of the signal holding the number.
        signal: SignalKey<f64>,
    },
    /// The number of pending orders across all CEX accounts.
    PendingOrders,
    /// The number of pending orders with the tag, across all CEX accounts.
    PendingOrdersWithTag {
        /// Only orders carrying this tag are counted.
        tag: Tag,
    },
    /// The number of pending orders in a CEX account.
    PendingOrdersInCexAccount {
        /// The CEX the account is on.
        cex_id_value: CexIdValue,
        /// The account to count pending orders in.
        account_id_value: AccountIdValue,
    },
    /// The number of pending orders with the tag in a CEX account.
    PendingOrdersInCexAccountWithTag {
        /// The CEX the account is on.
        cex_id_value: CexIdValue,
        /// The account to count pending orders in.
        account_id_value: AccountIdValue,
        /// Only orders carrying this tag are counted.
        tag: Tag,
    },
    /// The amount of an asset held across all CEX accounts, available and
    /// reserved.
    AssetTotal {
        /// The asset to sum.
        asset_id_value: AssetIdValue,
    },
    /// The amount of an asset held in a CEX account, available and reserved.
    AssetTotalInCexAccount {
        /// The asset to sum.
        asset_id_value: AssetIdValue,
        /// The CEX the account is on.
        cex_id_value: CexIdValue,
        /// The account to sum the balance of.
        account_id_value: AccountIdValue,
    },
    /// The amount of an asset across all CEX accounts that is not reserved
    /// for pending orders.
    AssetAvailable {
        /// The asset to measure.
        asset_id_value: AssetIdValue,
    },
    /// The amount of an asset in a CEX account that is not reserved for
    /// pending orders.
    AssetAvailableInCexAccount {
        /// The asset to measure.
        asset_id_value: AssetIdValue,
        /// The CEX the account is on.
        cex_id_value: CexIdValue,
        /// The account to measure the balance of.
        account_id_value: AccountIdValue,
    },
    /// The amount of an asset across all CEX accounts that is reserved for
    /// pending orders.
    AssetReserved {
        /// The asset to measure.
        asset_id_value: AssetIdValue,
    },
    /// The amount of an asset in a CEX account that is reserved for pending
    /// orders.
    AssetReservedInCexAccount {
        /// The asset to measure.
        asset_id_value: AssetIdValue,
        /// The CEX the account is on.
        cex_id_value: CexIdValue,
        /// The account to measure the reservations of.
        account_id_value: AccountIdValue,
    },
    /// `left` combined with `right` by the operator.
    BinaryCalculation {
        /// The left-hand operand.
        left: Box<NumberValue>,
        /// The operation applied to the operands.
        operator: BinaryOperator,
        /// The right-hand operand.
        right: Box<NumberValue>,
    },
    /// `number` transformed by the operator.
    UnaryCalculation {
        /// The operation applied to the value.
        operator: UnaryOperator,
        /// The value transformed by the operation.
        number: Box<NumberValue>,
    },
}

impl NumberValue {
    /// Creates a literal number value.
    ///
    /// Returns an error unless `literal` is finite.
    pub fn new(literal: f64) -> StockTrekResult<Self> {
        Validation::finite("NumberValue::new", "literal", literal)?;
        Ok(Self::Literal { literal })
    }

    /// Resolves the number against the resolved context.
    pub fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        match self {
            Self::Literal { literal } => {
                Self::finite_number("NumberValue::Literal", "literal", *literal)
            }
            Self::Signal { signal } => {
                Self::finite_number("NumberValue::Signal", "signal value", signal.read(c)?)
            }
            Self::PendingOrders => Ok(c.portfolio.pending_orders_count() as f64),
            Self::PendingOrdersWithTag { tag } => {
                Ok(c.portfolio.pending_orders_with_tag_count(tag) as f64)
            }
            Self::PendingOrdersInCexAccount {
                cex_id_value,
                account_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio
                    .pending_orders_in_cex_account_count(&cex_id, &account_id)
                    as f64)
            }
            Self::PendingOrdersInCexAccountWithTag {
                cex_id_value,
                account_id_value,
                tag,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio.pending_orders_in_cex_account_with_tag_count(
                    &cex_id,
                    &account_id,
                    tag,
                ) as f64)
            }
            Self::AssetTotal { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_total(&asset_id))
            }
            Self::AssetTotalInCexAccount {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio
                    .asset_total_in_cex_account(&cex_id, &account_id, &asset_id))
            }
            Self::AssetAvailable { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_available(&asset_id))
            }
            Self::AssetAvailableInCexAccount {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio
                    .asset_available_in_cex_account(&cex_id, &account_id, &asset_id))
            }
            Self::AssetReserved { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_reserved(&asset_id))
            }
            Self::AssetReservedInCexAccount {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio
                    .asset_reserved_in_cex_account(&cex_id, &account_id, &asset_id))
            }
            Self::BinaryCalculation {
                left,
                operator,
                right,
            } => {
                let left_value = left.number(c)?;
                let right_value = right.number(c)?;
                operator.calculate(left_value, right_value)
            }
            Self::UnaryCalculation { number, operator } => {
                let value = number.number(c)?;
                operator.calculate(value)
            }
        }
    }

    fn finite_number(
        function: &'static str,
        field: &'static str,
        value: f64,
    ) -> StockTrekResult<f64> {
        Validation::finite(function, field, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_accepts_finite_numbers() {
        for value in [0.0, -1.0, 1.5] {
            assert!(matches!(
                NumberValue::new(value).unwrap(),
                NumberValue::Literal { literal } if literal == value
            ));
        }
    }

    #[test]
    fn literal_rejects_non_finite_numbers() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(NumberValue::new(value).is_err());
        }
    }
}
