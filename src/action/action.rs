use crate::{
    action::resolved_action::ResolvedAction,
    cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult,
    resolvable::Resolvable,
    resolved_context::ResolvedContext,
    value::value::{AccountIdValue, AssetIdValue, CexIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{capability::CexCapability, order_request::OrderRequest, tag::Tag};
use strum::Display;

/// An action that an algorithm may request during strategy evaluation.
///
/// Actions are built with [`ActionFactory`](crate::actions::ActionFactory) and
/// carry unresolved values. They are resolved against a
/// [`ResolvedContext`](crate::ResolvedContext) immediately before the runtime
/// enqueues them. Prefer the factory methods over constructing variants
/// directly so the action is paired with a recovery policy.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Action {
    /// Places an order in a CEX account.
    PlaceOrder {
        /// The CEX on which the order should be placed.
        cex_id_value: CexIdValue,
        /// The account within the CEX that should place the order.
        account_id_value: AccountIdValue,
        /// The order to place, with unresolved asset identifiers and numbers.
        order_request: Box<OrderRequest<AssetIdValue, NumberValue>>,
    },
    /// Cancels all orders across all CEX accounts.
    CancelAllOrders {
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX.
    CancelAllOrdersInCex {
        /// The CEX whose orders should be cancelled.
        cex_id_value: CexIdValue,
        /// When set, only orders carrying this tag are cancelled.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX account.
    CancelAllOrdersInCexAccount {
        /// The CEX whose account should have its orders cancelled.
        cex_id_value: CexIdValue,
        /// The account whose orders should be cancelled.
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
