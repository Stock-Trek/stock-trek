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

/// Declares a value enum that resolves a literal or a signal-backed value.
///
/// `resolve` selects how a literal is read: `copy` dereferences it and `clone`
/// clones it. The remaining parameters provide the documentation for the
/// generated items.
macro_rules! value_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            method: $method:ident,
            ty: $ty:ty,
            resolve: $resolve:ident,
            docs: {
                literal: $literal_doc:literal,
                literal_field: $literal_field_doc:literal,
                signal: $signal_doc:literal,
                signal_field: $signal_field_doc:literal,
                method: $method_doc:literal $(,)?
            } $(,)?
        } $(,)?
    ) => {
        $(#[$meta])*
        #[derive(Debug, Display, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        #[serde(tag = "type")]
        #[non_exhaustive]
        pub enum $name {
            #[doc = $literal_doc]
            Literal {
                #[doc = $literal_field_doc]
                literal: $ty,
            },
            #[doc = $signal_doc]
            Signal {
                #[doc = $signal_field_doc]
                signal: SignalKey<$ty>,
            },
        }

        impl $name {
            #[doc = $method_doc]
            pub fn $method(&self, c: &ResolvedContext) -> StockTrekResult<$ty> {
                match self {
                    Self::Literal { literal } => Ok(value_enum!(@resolve $resolve literal)),
                    Self::Signal { signal } => signal.read(c),
                }
            }
        }
    };
    (@resolve copy $literal:ident) => {
        *$literal
    };
    (@resolve clone $literal:ident) => {
        $literal.clone()
    };
}

value_enum! {
    /// A value that resolves to a CEX account identifier.
    AccountIdValue {
        method: account_id,
        ty: AccountId,
        resolve: clone,
        docs: {
            literal: "A literal account identifier.",
            literal_field: "The account identifier.",
            signal: "An account identifier read from a signal.",
            signal_field: "The key of the signal holding the account identifier.",
            method: "Resolves the account identifier against the resolved context.",
        },
    }
}

value_enum! {
    /// A value that resolves to a CEX identifier.
    CexIdValue {
        method: cex_id,
        ty: CexId,
        resolve: copy,
        docs: {
            literal: "A literal CEX identifier.",
            literal_field: "The CEX identifier.",
            signal: "A CEX identifier read from a signal.",
            signal_field: "The key of the signal holding the CEX identifier.",
            method: "Resolves the CEX identifier against the resolved context.",
        },
    }
}

value_enum! {
    /// A value that resolves to an asset identifier.
    AssetIdValue {
        method: asset_id,
        ty: AssetId,
        resolve: copy,
        docs: {
            literal: "A literal asset identifier.",
            literal_field: "The asset identifier.",
            signal: "An asset identifier read from a signal.",
            signal_field: "The key of the signal holding the asset identifier.",
            method: "Resolves the asset identifier against the resolved context.",
        },
    }
}

value_enum! {
    /// A value that resolves to a boolean flag.
    FlagValue {
        method: flag,
        ty: bool,
        resolve: copy,
        docs: {
            literal: "A literal boolean flag.",
            literal_field: "The flag.",
            signal: "A boolean flag read from a signal.",
            signal_field: "The key of the signal holding the flag.",
            method: "Resolves the flag against the resolved context.",
        },
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
    /// Resolves the number against the resolved context.
    pub fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let portfolio = c.portfolio();
        match self {
            Self::Literal { literal } => {
                Validation::finite("NumberValue::Literal", "literal", *literal)
            }
            Self::Signal { signal } => {
                Validation::finite("NumberValue::Signal", "signal value", signal.read(c)?)
            }
            Self::PendingOrders => Ok(portfolio.pending_orders_count() as f64),
            Self::PendingOrdersWithTag { tag } => {
                Ok(portfolio.pending_orders_with_tag_count(tag) as f64)
            }
            Self::PendingOrdersInCexAccount {
                cex_id_value,
                account_id_value,
            } => {
                let (cex_id, account_id) =
                    Self::resolve_cex_account(c, cex_id_value, account_id_value)?;
                Ok(portfolio.pending_orders_in_cex_account_count(&cex_id, &account_id) as f64)
            }
            Self::PendingOrdersInCexAccountWithTag {
                cex_id_value,
                account_id_value,
                tag,
            } => {
                let (cex_id, account_id) =
                    Self::resolve_cex_account(c, cex_id_value, account_id_value)?;
                Ok(
                    portfolio.pending_orders_in_cex_account_with_tag_count(
                        &cex_id,
                        &account_id,
                        tag,
                    ) as f64,
                )
            }
            Self::AssetTotal { asset_id_value } => {
                Ok(portfolio.asset_total(&asset_id_value.asset_id(c)?))
            }
            Self::AssetTotalInCexAccount {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let (cex_id, account_id) =
                    Self::resolve_cex_account(c, cex_id_value, account_id_value)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(portfolio.asset_total_in_cex_account(&cex_id, &account_id, &asset_id))
            }
            Self::AssetAvailable { asset_id_value } => {
                Ok(portfolio.asset_available(&asset_id_value.asset_id(c)?))
            }
            Self::AssetAvailableInCexAccount {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let (cex_id, account_id) =
                    Self::resolve_cex_account(c, cex_id_value, account_id_value)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(portfolio.asset_available_in_cex_account(&cex_id, &account_id, &asset_id))
            }
            Self::AssetReserved { asset_id_value } => {
                Ok(portfolio.asset_reserved(&asset_id_value.asset_id(c)?))
            }
            Self::AssetReservedInCexAccount {
                cex_id_value,
                account_id_value,
                asset_id_value,
            } => {
                let (cex_id, account_id) =
                    Self::resolve_cex_account(c, cex_id_value, account_id_value)?;
                let asset_id = asset_id_value.asset_id(c)?;
                Ok(portfolio.asset_reserved_in_cex_account(&cex_id, &account_id, &asset_id))
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

    /// Resolves the CEX and account identifiers held by a [`NumberValue`].
    fn resolve_cex_account(
        c: &ResolvedContext,
        cex_id_value: &CexIdValue,
        account_id_value: &AccountIdValue,
    ) -> StockTrekResult<(CexId, AccountId)> {
        Ok((cex_id_value.cex_id(c)?, account_id_value.account_id(c)?))
    }
}
