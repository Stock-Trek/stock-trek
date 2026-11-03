use super::validation::*;

use crate::error::{domain::DomainError, result::StockTrekError};

#[test]
fn finite_accepts_finite_values_and_rejects_nan_and_infinity() {
    assert_eq!(Validation::finite("test", "value", 1.5).unwrap(), 1.5);
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Validation::finite("test", "value", value).is_err());
    }
}

#[test]
fn positive_rejects_zero_and_negative_values() {
    assert_eq!(Validation::positive("test", "value", 1.5).unwrap(), 1.5);
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(Validation::positive("test", "value", value).is_err());
    }
}

#[test]
fn non_negative_accepts_zero_and_rejects_negative_values() {
    assert_eq!(Validation::non_negative("test", "value", 0.0).unwrap(), 0.0);
    for value in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(Validation::non_negative("test", "value", value).is_err());
    }
}

#[test]
fn bid_below_ask_rejects_crossed_and_equal_prices() {
    assert!(Validation::bid_below_ask("test", 99.0, 100.0).is_ok());
    for (bid_price, ask_price) in [(100.0, 100.0), (101.0, 100.0)] {
        assert!(Validation::bid_below_ask("test", bid_price, ask_price).is_err());
    }
}

#[test]
fn failures_produce_a_domain_error() {
    let error = Validation::positive("test", "value", 0.0).unwrap_err();
    assert!(matches!(error, StockTrekError::Domain(DomainError { .. })));
}
