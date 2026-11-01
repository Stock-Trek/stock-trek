use crate::{
    action::resolved_action::ResolvedAction,
    cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult,
    resolvable::Resolvable,
    resolved_context::ResolvedContext,
    value::value::{AccountIdValue, AssetIdValue, CexIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{capability::CexCapability, order_request::OrderRequest};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Action {
    SendOrderRequest {
        cex_id_value: CexIdValue,
        account_id_value: AccountIdValue,
        order_request: Box<OrderRequest<AssetIdValue, NumberValue>>,
    },
    // TODO
    // CancelAllOrders,
    // CancelAllOrdersWithTag {
    //     tag: Tag,
    // },
    // CancelAllOrdersInCexAccount {
    //     cex_id_value: CexIdValue,
    //     account_id_value: AccountIdValue,
    // },
    // CancelAllOrdersInCexAccountWithTag {
    //     cex_id_value: CexIdValue,
    //     account_id_value: AccountIdValue,
    //     tag: Tag,
    // },
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
            // TODO
            // Action::CancelAllOrders => Ok(ResolvedAction::CancelAllOrders),
            // Action::CancelAllOrdersWithTag { tag } => {
            //     Ok(ResolvedAction::CancelAllOrdersWithTag { tag: tag.clone() })
            // }
            // Action::CancelAllOrdersInCexAccount { cex_id_value, account_id_value } => {
            //     Ok(ResolvedAction::CancelAllOrdersInCexAccount {
            //         cex_id: cex_id_value.cex_id(c)?,
            //         account_id: account_id_value.account_id(c)?,
            //     })
            // }
            // Action::CancelAllOrdersInCexAccountWithTag { cex_id_value, account_id_value, tag } => {
            //     Ok(ResolvedAction::CancelAllOrdersInCexAccountWithTag {
            //         cex_id: cex_id_value.cex_id(c)?,
            //         account_id: account_id_value.account_id(c)?,
            //         tag: tag.clone(),
            //     })
            // }
        }
    }
}

impl HasRequiredCapabilities for Action {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        match self {
            Action::SendOrderRequest { order_request, .. } => order_request.required_capabilities(),
            // TODO
            // Action::CancelAllOrders
            // | Action::CancelAllOrdersWithTag { .. }
            // | Action::CancelAllOrdersInCexAccount { .. }
            // | Action::CancelAllOrdersInCexAccountWithTag { .. } => vec![],
        }
    }
}
