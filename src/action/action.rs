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

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
/// An action that an algorithm may request during strategy evaluation.
pub enum Action {
    /// Sends an order request to a CEX account.
    SendOrderRequest {
        /// The CEX on which the order should be placed.
        cex_id_value: CexIdValue,
        /// The account that should place the order.
        account_id_value: AccountIdValue,
        /// The unresolved order request to send.
        order_request: Box<OrderRequest<AssetIdValue, NumberValue>>,
    },
    /// Cancels all orders across all CEX accounts.
    CancelAllOrders {
        /// The optional tag of the orders to cancel.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX.
    CancelAllOrdersInCex {
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The optional tag of the orders to cancel.
        tag: Option<Tag>,
    },
    /// Cancels all orders in a CEX account.
    CancelAllOrdersInCexAccount {
        /// The CEX containing the account.
        cex_id_value: CexIdValue,
        /// The account whose orders should be cancelled.
        account_id_value: AccountIdValue,
        /// The optional tag of the orders to cancel.
        tag: Option<Tag>,
    },
}

impl Resolvable<ResolvedAction> for Action {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        match self {
            Action::SendOrderRequest {
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
            Action::SendOrderRequest { order_request, .. } => order_request.required_capabilities(),
            Action::CancelAllOrders { .. }
            | Action::CancelAllOrdersInCex { .. }
            | Action::CancelAllOrdersInCexAccount { .. } => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{portfolio::Portfolio, signal::signals::Signals};
    use std::collections::HashMap;
    use stock_trek_types::cex::{account_id::AccountId, cex_id::CexId};

    fn context() -> ResolvedContext {
        ResolvedContext {
            enqueue_action: Box::new(|_, _| Ok(())),
            portfolio: Portfolio::new(HashMap::new()),
            signals: Signals::new(),
        }
    }

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
    fn cancel_all_orders_resolves() {
        let resolved = Action::CancelAllOrders { tag: None }
            .try_resolve(&context())
            .unwrap();
        assert!(matches!(
            resolved,
            ResolvedAction::CancelAllOrders { tag: None }
        ));
    }

    #[test]
    fn cancel_all_orders_with_tag_resolves() {
        let tag = Tag::new("tag");
        let resolved = Action::CancelAllOrders {
            tag: Some(tag.clone()),
        }
        .try_resolve(&context())
        .unwrap();
        match resolved {
            ResolvedAction::CancelAllOrders { tag: resolved_tag } => {
                assert_eq!(resolved_tag, Some(tag));
            }
            _ => panic!("unexpected resolved action"),
        }
    }

    #[test]
    fn cancel_all_orders_in_cex_account_resolves() {
        let resolved = Action::CancelAllOrdersInCexAccount {
            cex_id_value: cex_id_value(),
            account_id_value: account_id_value(),
            tag: None,
        }
        .try_resolve(&context())
        .unwrap();
        match resolved {
            ResolvedAction::CancelAllOrdersInCexAccount {
                cex_id,
                account_id,
                tag: None,
            } => {
                assert_eq!(cex_id, CexId::Binance);
                assert_eq!(account_id, AccountId::new("account"));
            }
            _ => panic!("unexpected resolved action"),
        }
    }

    #[test]
    fn cancel_all_orders_in_cex_account_with_tag_resolves() {
        let tag = Tag::new("tag");
        let resolved = Action::CancelAllOrdersInCexAccount {
            cex_id_value: cex_id_value(),
            account_id_value: account_id_value(),
            tag: Some(tag.clone()),
        }
        .try_resolve(&context())
        .unwrap();
        match resolved {
            ResolvedAction::CancelAllOrdersInCexAccount {
                cex_id,
                account_id,
                tag: resolved_tag,
            } => {
                assert_eq!(cex_id, CexId::Binance);
                assert_eq!(account_id, AccountId::new("account"));
                assert_eq!(resolved_tag, Some(tag));
            }
            _ => panic!("unexpected resolved action"),
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
