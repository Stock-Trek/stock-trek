use super::conditions::*;

use crate::{
    error::result::StockTrekResult, resolved_context::ResolvedContext,
    value::value_types::NumberValue,
};
use std::cmp::Ordering;

use crate::{portfolio::Portfolio, signal::signals::Signals};

fn context() -> ResolvedContext {
    ResolvedContext::new(
        Box::new(|_, _| Ok(())),
        Portfolio::default(),
        Signals::new(),
    )
}

fn compare(left: f64, comparison: Ordering, right: f64) -> bool {
    ConditionFactory
        .compare(
            NumberValue::Literal { literal: left },
            comparison,
            NumberValue::Literal { literal: right },
        )
        .test(&context())
        .unwrap()
}

fn literal_condition(value: bool) -> Condition {
    if value {
        ConditionFactory.compare(
            NumberValue::Literal { literal: 1.0 },
            Ordering::Less,
            NumberValue::Literal { literal: 2.0 },
        )
    } else {
        ConditionFactory.compare(
            NumberValue::Literal { literal: 2.0 },
            Ordering::Less,
            NumberValue::Literal { literal: 1.0 },
        )
    }
}

fn failing_condition() -> Condition {
    ConditionFactory.compare(
        NumberValue::Literal { literal: f64::NAN },
        Ordering::Equal,
        NumberValue::Literal { literal: 0.0 },
    )
}

fn quantity_of(quantity_of: QuantityOf, conditions: Vec<Condition>) -> StockTrekResult<bool> {
    ConditionFactory
        .quantity_of(quantity_of, conditions)
        .test(&context())
}

#[test]
fn compare_serializes_ordering_as_snake_case() {
    let condition = ConditionFactory.compare(
        NumberValue::Literal { literal: 1.0 },
        Ordering::Less,
        NumberValue::Literal { literal: 2.0 },
    );

    let value = serde_json::to_value(&condition).unwrap();
    assert_eq!(value["comparison"], "less");

    let deserialized: Condition = serde_json::from_value(value).unwrap();
    assert!(deserialized.test(&context()).unwrap());
}

#[test]
fn compare_matches_the_actual_ordering() {
    assert!(compare(1.0, Ordering::Less, 2.0));
    assert!(!compare(2.0, Ordering::Less, 1.0));
    assert!(!compare(1.0, Ordering::Less, 1.0));

    assert!(compare(2.0, Ordering::Equal, 2.0));
    assert!(!compare(2.0, Ordering::Equal, 1.0));
    assert!(!compare(1.0, Ordering::Equal, 2.0));

    assert!(compare(2.0, Ordering::Greater, 1.0));
    assert!(!compare(1.0, Ordering::Greater, 2.0));
    assert!(!compare(1.0, Ordering::Greater, 1.0));
}

#[test]
fn quantity_of_all_short_circuits_after_false() {
    let conditions = vec![literal_condition(false), failing_condition()];

    assert!(!quantity_of(QuantityOf::All, conditions).unwrap());
}

#[test]
fn quantity_of_none_short_circuits_after_true() {
    let conditions = vec![literal_condition(true), failing_condition()];

    assert!(!quantity_of(QuantityOf::None, conditions).unwrap());
}

#[test]
fn quantity_of_at_least_one_short_circuits_after_true() {
    let conditions = vec![literal_condition(true), failing_condition()];

    assert!(quantity_of(QuantityOf::AtLeastOne, conditions).unwrap());
}

#[test]
fn quantity_of_partial_short_circuits_after_true_and_false() {
    let conditions = vec![
        literal_condition(true),
        literal_condition(false),
        failing_condition(),
    ];

    assert!(quantity_of(QuantityOf::Partial, conditions).unwrap());
}

#[test]
fn quantity_of_empty_does_not_evaluate_conditions() {
    assert!(quantity_of(QuantityOf::Empty, vec![]).unwrap());
    assert!(!quantity_of(QuantityOf::Empty, vec![failing_condition()]).unwrap());
}

#[test]
fn quantity_of_propagates_errors_when_the_result_is_not_determined() {
    assert!(quantity_of(QuantityOf::All, vec![failing_condition()]).is_err());
    assert!(quantity_of(QuantityOf::None, vec![failing_condition()]).is_err());
    assert!(quantity_of(QuantityOf::AtLeastOne, vec![failing_condition()]).is_err());
    assert!(quantity_of(QuantityOf::Partial, vec![failing_condition()]).is_err());
}

#[test]
fn quantity_of_returns_the_expected_result() {
    assert!(
        quantity_of(
            QuantityOf::All,
            vec![literal_condition(true), literal_condition(true)]
        )
        .unwrap()
    );
    assert!(
        !quantity_of(
            QuantityOf::All,
            vec![literal_condition(true), literal_condition(false)]
        )
        .unwrap()
    );
    assert!(
        quantity_of(
            QuantityOf::AtLeastOne,
            vec![literal_condition(true), literal_condition(false)]
        )
        .unwrap()
    );
    assert!(
        quantity_of(
            QuantityOf::AtLeastOne,
            vec![literal_condition(true), literal_condition(true)]
        )
        .unwrap()
    );
    assert!(
        !quantity_of(
            QuantityOf::AtLeastOne,
            vec![literal_condition(false), literal_condition(false)]
        )
        .unwrap()
    );
    assert!(!quantity_of(QuantityOf::AtLeastOne, vec![]).unwrap());
    assert!(
        quantity_of(
            QuantityOf::Partial,
            vec![literal_condition(true), literal_condition(false)]
        )
        .unwrap()
    );
    assert!(
        !quantity_of(
            QuantityOf::Partial,
            vec![literal_condition(true), literal_condition(true)]
        )
        .unwrap()
    );
    assert!(
        quantity_of(
            QuantityOf::None,
            vec![literal_condition(false), literal_condition(false)]
        )
        .unwrap()
    );
}
