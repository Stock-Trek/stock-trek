use crate::{
    resolvable::Resolvable, resolved_context::ResolvedContext, value::value_types::NumberValue,
};
use stock_trek_types::cex::activation::Activation;

use crate::{portfolio::Portfolio, signal::signals::Signals};
use stock_trek_types::cex::{
    price_basis::PriceBasis, trigger_direction::TriggerDirection, trigger_mode::TriggerMode,
};

fn context() -> ResolvedContext {
    ResolvedContext {
        enqueue_action: Box::new(|_, _| Ok(())),
        portfolio: Portfolio::default(),
        signals: Signals::new(),
    }
}

fn price_triggered(activation_price: f64) -> Activation<NumberValue> {
    Activation::PriceTriggered {
        activation_price: NumberValue::Literal {
            literal: activation_price,
        },
        basis: PriceBasis::Last,
        direction: TriggerDirection::Above,
        mode: TriggerMode::Touch,
    }
}

fn trailing(activation_price: f64) -> Activation<NumberValue> {
    Activation::Trailing {
        activation_price: NumberValue::Literal {
            literal: activation_price,
        },
        basis: PriceBasis::Last,
        callback_rate_bps: 100,
        direction: TriggerDirection::Below,
    }
}

#[test]
fn resolves_positive_activation_prices() {
    assert!(matches!(
        price_triggered(100.0).try_resolve(&context()).unwrap(),
        Activation::PriceTriggered {
            activation_price: value,
            ..
        } if value == 100.0
    ));
    assert!(matches!(
        trailing(100.0).try_resolve(&context()).unwrap(),
        Activation::Trailing {
            activation_price: value,
            ..
        } if value == 100.0
    ));
}

#[test]
fn rejects_invalid_activation_prices() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(price_triggered(value).try_resolve(&context()).is_err());
        assert!(trailing(value).try_resolve(&context()).is_err());
    }
}
