use crate::{
    resolvable::Resolvable, resolved_context::ResolvedContext, value::value_types::NumberValue,
};
use stock_trek_types::cex::quantity::Quantity;

use crate::{portfolio::Portfolio, signal::signals::Signals};

fn context() -> ResolvedContext {
    ResolvedContext {
        enqueue_action: Box::new(|_, _| Ok(())),
        portfolio: Portfolio::default(),
        signals: Signals::new(),
    }
}

#[test]
fn resolves_positive_quantities() {
    let base = Quantity::OfBase(NumberValue::Literal { literal: 1.5 });
    let quote = Quantity::OfQuote(NumberValue::Literal { literal: 100.0 });

    assert!(matches!(
        base.try_resolve(&context()).unwrap(),
        Quantity::OfBase(value) if value == 1.5
    ));
    assert!(matches!(
        quote.try_resolve(&context()).unwrap(),
        Quantity::OfQuote(value) if value == 100.0
    ));
}

#[test]
fn rejects_non_finite_and_non_positive_quantities() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -1.0] {
        assert!(
            Quantity::OfBase(NumberValue::Literal { literal: value })
                .try_resolve(&context())
                .is_err()
        );
        assert!(
            Quantity::OfQuote(NumberValue::Literal { literal: value })
                .try_resolve(&context())
                .is_err()
        );
    }
}
