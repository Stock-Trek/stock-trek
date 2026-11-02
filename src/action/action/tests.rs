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
