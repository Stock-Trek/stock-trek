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
        /// The concrete account identifier.
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
        /// The concrete CEX identifier.
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
        /// The concrete asset identifier.
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
        /// The concrete flag value.
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
///
/// Numbers may be literals, signal values, portfolio quantities, or
/// calculations. Portfolio quantities are evaluated against the portfolio of
/// the current evaluation, so the same `NumberValue` can produce different
/// results over time.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum NumberValue {
    /// A literal number.
    Literal {
        /// The concrete number.
        literal: f64,
    },
    /// A number read from a signal.
    Signal {
        /// The key of the signal holding the number.
        signal: SignalKey<f64>,
    },
    /// The total number of pending orders.
    PendingOrders,
    /// The number of pending orders with a specific tag.
    PendingOrdersWithTag {
        /// Only orders carrying this tag are counted.
        tag: Tag,
    },
    /// The number of pending orders in a CEX account.
    PendingOrdersInCexAccount {
        /// The CEX to count orders in.
        cex_id_value: CexIdValue,
        /// The account to count orders in.
        account_id_value: AccountIdValue,
    },
    /// The number of pending orders with a specific tag in a CEX account.
    PendingOrdersInCexAccountWithTag {
        /// The CEX to count orders in.
        cex_id_value: CexIdValue,
        /// The account to count orders in.
        account_id_value: AccountIdValue,
        /// Only orders carrying this tag are counted.
        tag: Tag,
    },
    /// The total amount of an asset.
    AssetTotal {
        /// The asset whose total balance should be read.
        asset_id_value: AssetIdValue,
    },
    /// The total amount of an asset in a CEX account.
    AssetTotalInCexAccount {
        /// The asset whose total balance should be read.
        asset_id_value: AssetIdValue,
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account whose balance should be read.
        account_id_value: AccountIdValue,
    },
    /// The available amount of an asset.
    AssetAvailable {
        /// The asset whose available balance should be read.
        asset_id_value: AssetIdValue,
    },
    /// The available amount of an asset in a CEX account.
    AssetAvailableInCexAccount {
        /// The asset whose available balance should be read.
        asset_id_value: AssetIdValue,
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account whose balance should be read.
        account_id_value: AccountIdValue,
    },
    /// The reserved amount of an asset.
    AssetReserved {
        /// The asset whose reserved balance should be read.
        asset_id_value: AssetIdValue,
    },
    /// The reserved amount of an asset in a CEX account.
    AssetReservedInCexAccount {
        /// The asset whose reserved balance should be read.
        asset_id_value: AssetIdValue,
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account whose balance should be read.
        account_id_value: AccountIdValue,
    },
    /// A binary calculation.
    BinaryCalculation {
        /// The left-hand operand.
        left: Box<NumberValue>,
        /// The operation to apply.
        operator: BinaryOperator,
        /// The right-hand operand.
        right: Box<NumberValue>,
    },
    /// A unary calculation.
    UnaryCalculation {
        /// The operation to apply.
        operator: UnaryOperator,
        /// The operand.
        number: Box<NumberValue>,
    },
}

impl NumberValue {
    /// Resolves the number against the resolved context.
    ///
    /// Literal and signal values must be finite. Calculations propagate
    /// numeric errors such as division by zero or a non-finite result. Asset
    /// totals include both available and reserved amounts; available amounts
    /// are clamped to zero if pending orders reserve more than the balance.
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
