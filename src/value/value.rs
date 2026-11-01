use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    value::{binary_operator::BinaryOperator, unary_operator::UnaryOperator},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId, tag::Tag};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AccountIdValue {
    Literal { literal: AccountId },
    Signal { signal: SignalKey<AccountId> },
}

impl AccountIdValue {
    pub fn account_id(&self, c: &ResolvedContext) -> StockTrekResult<AccountId> {
        match self {
            Self::Literal { literal } => Ok(literal.clone()),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CexIdValue {
    Literal { literal: CexId },
    Signal { signal: SignalKey<CexId> },
}

impl CexIdValue {
    pub fn cex_id(&self, c: &ResolvedContext) -> StockTrekResult<CexId> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AssetIdValue {
    Literal { literal: AssetId },
    Signal { signal: SignalKey<AssetId> },
}

impl AssetIdValue {
    pub fn asset_id(&self, c: &ResolvedContext) -> StockTrekResult<AssetId> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum FlagValue {
    Literal { literal: bool },
    Signal { signal: SignalKey<bool> },
}

impl FlagValue {
    pub fn flag(&self, c: &ResolvedContext) -> StockTrekResult<bool> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
        }
    }
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum NumberValue {
    Literal {
        literal: f64,
    },
    Signal {
        signal: SignalKey<f64>,
    },
    PendingOrders,
    PendingOrdersWithTag {
        tag: Tag,
    },
    PendingOrdersInCexAccount {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    },
    PendingOrdersInCexAccountWithTag {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        tag: Tag,
    },
    AssetTotal {
        asset_id_value: AssetIdValue,
    },
    AssetTotalInCexAccount {
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    },
    AssetAvailable {
        asset_id_value: AssetIdValue,
    },
    AssetAvailableInCexAccount {
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    },
    AssetReserved {
        asset_id_value: AssetIdValue,
    },
    AssetReservedInCexAccount {
        asset_id_value: AssetIdValue,
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
    },
    BinaryCalculation {
        left: Box<NumberValue>,
        operator: BinaryOperator,
        right: Box<NumberValue>,
    },
    UnaryCalculation {
        operator: UnaryOperator,
        number: Box<NumberValue>,
    },
}

impl NumberValue {
    pub fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        match self {
            Self::Literal { literal } => Ok(*literal),
            Self::Signal { signal } => signal.read(c),
            Self::PendingOrders => Ok(c.portfolio.pending_orders()),
            Self::PendingOrdersWithTag { tag } => Ok(c.portfolio.pending_orders_with_tag(tag)),
            Self::PendingOrdersInCexAccount {
                cex_id_value,
                account_id_value,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio
                    .pending_orders_in_cex_account(&cex_id, &account_id))
            }
            Self::PendingOrdersInCexAccountWithTag {
                cex_id_value,
                account_id_value,
                tag,
            } => {
                let cex_id = cex_id_value.cex_id(c)?;
                let account_id = account_id_value.account_id(c)?;
                Ok(c.portfolio
                    .pending_orders_in_cex_account_with_tag(&cex_id, &account_id, tag))
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
