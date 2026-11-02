use super::*;

const OPERATORS: [BinaryOperator; 8] = [
    BinaryOperator::Add,
    BinaryOperator::Sub,
    BinaryOperator::Mul,
    BinaryOperator::Div,
    BinaryOperator::Mod,
    BinaryOperator::Pow,
    BinaryOperator::Log,
    BinaryOperator::Atan2,
];

#[test]
fn rejects_nan_operands() {
    for operator in OPERATORS {
        assert!(operator.calculate(f64::NAN, 2.0).is_err(), "{operator}");
        assert!(operator.calculate(2.0, f64::NAN).is_err(), "{operator}");
    }
}

#[test]
fn rejects_infinite_operands() {
    for operator in OPERATORS {
        for value in [f64::INFINITY, f64::NEG_INFINITY] {
            assert!(operator.calculate(value, 2.0).is_err(), "{operator}");
            assert!(operator.calculate(2.0, value).is_err(), "{operator}");
        }
    }
}

#[test]
fn rejects_non_finite_results() {
    assert!(BinaryOperator::Add.calculate(f64::MAX, f64::MAX).is_err());
    assert!(BinaryOperator::Sub.calculate(f64::MIN, f64::MAX).is_err());
    assert!(BinaryOperator::Mul.calculate(f64::MAX, 2.0).is_err());
    assert!(BinaryOperator::Div.calculate(f64::MAX, 0.5).is_err());
    assert!(BinaryOperator::Pow.calculate(2.0, 1024.0).is_err());
}
