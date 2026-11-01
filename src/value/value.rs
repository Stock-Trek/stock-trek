use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
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
        /// The signal key containing the account identifier.
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
        /// The signal key containing the CEX identifier.
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
        /// The signal key containing the asset identifier.
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
        /// The boolean value.
        literal: bool,
    },
    /// A boolean flag read from a signal.
    Signal {
        /// The signal key containing the flag.
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
        /// The numeric value.
        literal: f64,
    },
    /// A number read from a signal.
    Signal {
        /// The signal key containing the number.
        signal: SignalKey<f64>,
    },
    /// The total number of pending orders.
    PendingOrders,
    /// The number of pending orders with a specific tag.
    PendingOrdersWithTag {
        /// The tag to count.
        tag: Tag,
    },
    /// The number of pending orders in a CEX account.
    PendingOrdersInCexAccount {
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account to inspect.
        account_id_value: AccountIdValue,
    },
    /// The number of pending orders with a specific tag in a CEX account.
    PendingOrdersInCexAccountWithTag {
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account to inspect.
        account_id_value: AccountIdValue,
        /// The tag to count.
        tag: Tag,
    },
    /// The total amount of an asset.
    AssetTotal {
        /// The asset to sum.
        asset_id_value: AssetIdValue,
    },
    /// The total amount of an asset in a CEX account.
    AssetTotalInCexAccount {
        /// The asset to sum.
        asset_id_value: AssetIdValue,
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account to inspect.
        account_id_value: AccountIdValue,
    },
    /// The available amount of an asset.
    AssetAvailable {
        /// The asset to inspect.
        asset_id_value: AssetIdValue,
    },
    /// The available amount of an asset in a CEX account.
    AssetAvailableInCexAccount {
        /// The asset to inspect.
        asset_id_value: AssetIdValue,
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account to inspect.
        account_id_value: AccountIdValue,
    },
    /// The reserved amount of an asset.
    AssetReserved {
        /// The asset to inspect.
        asset_id_value: AssetIdValue,
    },
    /// The reserved amount of an asset in a CEX account.
    AssetReservedInCexAccount {
        /// The asset to inspect.
        asset_id_value: AssetIdValue,
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account to inspect.
        account_id_value: AccountIdValue,
    },
    /// A binary calculation.
    BinaryCalculation {
        /// The left-hand value.
        left: Box<NumberValue>,
        /// The operator to apply.
        operator: BinaryOperator,
        /// The right-hand value.
        right: Box<NumberValue>,
    },
    /// A unary calculation.
    UnaryCalculation {
        /// The operator to apply.
        operator: UnaryOperator,
        /// The value to operate on.
        number: Box<NumberValue>,
    },
}

impl NumberValue {
    /// Resolves the number against the resolved context.
    pub fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
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
                Ok(c.portfolio.asset_total(&asset_id).as_f64())
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
                    .asset_total_in_cex_account(&asset_id, &cex_id, &account_id)
                    .as_f64())
            }
            Self::AssetAvailable { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_available(&asset_id).as_f64())
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
                    .asset_available_in_cex_account(&asset_id, &cex_id, &account_id)
                    .as_f64())
            }
            Self::AssetReserved { asset_id_value } => {
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(c.portfolio.asset_reserved(&asset_id).as_f64())
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
                    .asset_reserved_in_cex_account(&asset_id, &cex_id, &account_id)
                    .as_f64())
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
}
