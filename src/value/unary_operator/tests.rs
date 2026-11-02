use super::*;

const OPERATORS: [UnaryOperator; 26] = [
    UnaryOperator::Abs,
    UnaryOperator::Neg,
    UnaryOperator::Floor,
    UnaryOperator::Ceil,
    UnaryOperator::RoundAwayFrom0,
    UnaryOperator::RoundTiesEven,
    UnaryOperator::Trunc,
    UnaryOperator::Frac,
    UnaryOperator::Sqrt,
    UnaryOperator::Exp,
    UnaryOperator::Exp2,
    UnaryOperator::Log2,
    UnaryOperator::LogE,
    UnaryOperator::Log10,
    UnaryOperator::Sin,
    UnaryOperator::Cos,
    UnaryOperator::Tan,
    UnaryOperator::Asin,
    UnaryOperator::Acos,
    UnaryOperator::Atan,
    UnaryOperator::Sinh,
    UnaryOperator::Cosh,
    UnaryOperator::Tanh,
    UnaryOperator::Asinh,
    UnaryOperator::Acosh,
    UnaryOperator::Atanh,
];

#[test]
fn rejects_non_finite_inputs() {
    for operator in OPERATORS {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(operator.calculate(value).is_err(), "{operator}");
        }
    }
}

#[test]
fn rejects_non_finite_results() {
    assert!(UnaryOperator::Exp.calculate(1_000.0).is_err());
    assert!(UnaryOperator::Exp2.calculate(2_000.0).is_err());
    assert!(UnaryOperator::Sinh.calculate(1_000.0).is_err());
    assert!(UnaryOperator::Cosh.calculate(1_000.0).is_err());
}

#[test]
fn accepts_finite_values() {
    assert_eq!(UnaryOperator::Sqrt.calculate(9.0).unwrap(), 3.0);
    assert_eq!(UnaryOperator::LogE.calculate(1.0).unwrap(), 0.0);
}
