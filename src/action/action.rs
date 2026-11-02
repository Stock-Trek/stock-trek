use crate::{
    action::resolved_action::ResolvedAction,
    cex::capability::HasRequiredCapabilities,
    error::stock_trek_error::StockTrekResult,
    resolvable::Resolvable,
    resolved_context::ResolvedContext,
    value::number_value::{AccountIdValue, AssetIdValue, CexIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{capability::CexCapability, order_request::OrderRequest, tag::Tag};
use strum::Display;

/// An action that an algorithm may request during strategy evaluation.
///
/// Actions are enqueued by a [`crate::Command`] and executed after
/// interpretation. CEX and account targets are [`CexIdValue`]s and
/// [`AccountIdValue`]s so they can come from signals as well as literals.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Action {
    /// Places an order in a CEX account.
    PlaceOrder {
        /// The CEX to place the order on.
        cex_id_value: CexIdValue,
        /// The account to place the order in.
        account_id_value: AccountIdValue,
        /// The order to place, with any quantity, price, and asset
        /// identifier left unresolved.
        order_request: Box<OrderRequest<AssetIdValue, NumberValue>>,
    },
    /// Cancels all orders across all CEX accounts.
    CancelAllOrders {
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX.
    CancelAllOrdersInCex {
        /// The CEX whose orders are cancelled.
        cex_id_value: CexIdValue,
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX account.
    CancelAllOrdersInCexAccount {
        /// The CEX the account is on.
        cex_id_value: CexIdValue,
        /// The account whose orders are cancelled.
        account_id_value: AccountIdValue,
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
}

impl Resolvable<ResolvedAction> for Action {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        match self {
            Action::PlaceOrder {
                cex_id_value,
                account_id_value,
                order_request,
            } => Ok(ResolvedAction::PlaceOrder {
                cex_id: cex_id_value.cex_id(c)?,
                account_id: account_id_value.account_id(c)?,
                order_request: order_request.try_resolve(c)?,
            }),
            Action::CancelAllOrders { tag } => {
                Ok(ResolvedAction::CancelAllOrders { tag: tag.clone() })
            }
            Action::CancelAllOrdersInCex { cex_id_value, tag } => {
                Ok(ResolvedAction::CancelAllOrdersInCex {
                    cex_id: cex_id_value.cex_id(c)?,
                    tag: tag.clone(),
                })
            }
            Action::CancelAllOrdersInCexAccount {
                cex_id_value,
                account_id_value,
                tag,
            } => Ok(ResolvedAction::CancelAllOrdersInCexAccount {
                cex_id: cex_id_value.cex_id(c)?,
                account_id: account_id_value.account_id(c)?,
                tag: tag.clone(),
            }),
        }
    }
}

impl HasRequiredCapabilities for Action {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        match self {
            Action::PlaceOrder { order_request, .. } => order_request.required_capabilities(),
            Action::CancelAllOrders { .. }
            | Action::CancelAllOrdersInCex { .. }
            | Action::CancelAllOrdersInCexAccount { .. } => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stock_trek_types::cex::{account_id::AccountId, cex_id::CexId};

    fn cex_id_value() -> CexIdValue {
        CexIdValue::Literal {
            literal: CexId::Binance,
        }
    }

    fn account_id_value() -> AccountIdValue {
        AccountIdValue::Literal {
            literal: AccountId::new("account"),
        }
    }

    #[test]
    fn cancel_actions_require_no_capabilities() {
        assert!(
            Action::CancelAllOrders { tag: None }
                .required_capabilities()
                .is_empty()
        );
        assert!(
            Action::CancelAllOrders {
                tag: Some(Tag::new("tag"))
            }
            .required_capabilities()
            .is_empty()
        );
        assert!(
            Action::CancelAllOrdersInCexAccount {
                cex_id_value: cex_id_value(),
                account_id_value: account_id_value(),
                tag: None
            }
            .required_capabilities()
            .is_empty()
        );
        assert!(
            Action::CancelAllOrdersInCexAccount {
                cex_id_value: cex_id_value(),
                account_id_value: account_id_value(),
                tag: Some(Tag::new("tag")),
            }
            .required_capabilities()
            .is_empty()
        );
    }
}
