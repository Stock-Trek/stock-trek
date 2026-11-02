//! Exact numeric values used at the Decimal/f64 boundary.
//!
//! Accounting, orders, and persisted values are represented by [`Decimal`].
//! Signals, indicators, and operator math are represented by `f64`. The
//! newtypes in this module make that boundary explicit: calculated values are
//! converted and quantized once, validated, and then carried as exact values.

use crate::error::{numeric::NumericError, result::StockTrekResult};
use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use std::fmt;

/// An exact, positive price.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Price(Decimal);

/// An exact, positive quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Quantity(Decimal);

/// An exact, positive notional value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Notional(Decimal);

macro_rules! numeric_newtype {
    ($name:ident, $kind:literal) => {
        impl $name {
            /// Creates the value from an already exact [`Decimal`].
            pub fn try_from_decimal(value: Decimal) -> StockTrekResult<Self> {
                if value <= Decimal::ZERO {
                    return Err(NumericError::NotPositive { kind: $kind, value }.into());
                }
                Ok(Self(value))
            }

            /// Converts a calculated `f64` into an exact value without
            /// quantizing it.
            pub fn try_from_calc(value: f64) -> StockTrekResult<Self> {
                let value = decimal_from_calc($kind, value)?;
                Self::try_from_decimal(value)
            }

            /// Converts a calculated `f64` into an exact value quantized to
            /// `increment` using `rounding`.
            pub fn try_from_calc_quantized(
                value: f64,
                increment: f64,
                rounding: RoundingStrategy,
            ) -> StockTrekResult<Self> {
                let value = decimal_from_calc($kind, value)?;
                let increment = decimal_from_calc("increment", increment)?;
                let quantized = quantize(value, increment, rounding)?;
                Self::try_from_decimal(quantized)
            }

            /// Returns the exact [`Decimal`] value.
            pub fn to_decimal(self) -> Decimal {
                self.0
            }

            /// Returns a lossy `f64` view for calculations.
            pub fn to_calc(self) -> f64 {
                decimal_to_calc(self.0)
            }

            /// Returns a lossy `f64` view for calculations.
            pub fn to_f64(self) -> f64 {
                self.to_calc()
            }
        }

        impl TryFrom<Decimal> for $name {
            type Error = crate::error::result::StockTrekError;

            fn try_from(value: Decimal) -> Result<Self, Self::Error> {
                Self::try_from_decimal(value)
            }
        }

        impl TryFrom<f64> for $name {
            type Error = crate::error::result::StockTrekError;

            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Self::try_from_calc(value)
            }
        }

        impl From<$name> for Decimal {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

numeric_newtype!(Price, "price");
numeric_newtype!(Quantity, "quantity");
numeric_newtype!(Notional, "notional");

/// Converts a calculated value into [`Decimal`], rejecting NaN and infinity.
pub fn decimal_from_calc(kind: &'static str, value: f64) -> StockTrekResult<Decimal> {
    if !value.is_finite() {
        return Err(NumericError::NonFinite { kind, value }.into());
    }
    Decimal::try_from(value).map_err(|_| NumericError::NonFinite { kind, value }.into())
}

/// Returns a lossy `f64` view of an exact [`Decimal`] for calculations.
pub fn decimal_to_calc(value: Decimal) -> f64 {
    value.to_string().parse::<f64>().unwrap_or(f64::NAN)
}

/// Quantizes `value` to a multiple of `increment`.
///
/// The quotient is rounded to an integer with `rounding` before being
/// multiplied back by the increment, so the result is always an exact multiple
/// of the increment.
pub fn quantize(
    value: Decimal,
    increment: Decimal,
    rounding: RoundingStrategy,
) -> StockTrekResult<Decimal> {
    if increment <= Decimal::ZERO {
        return Err(NumericError::InvalidIncrement {
            kind: "increment",
            value: increment,
        }
        .into());
    }

    let steps = value.checked_div(increment).ok_or(NumericError::Overflow {
        operation: "quantize division",
    })?;
    let rounded_steps = steps.round_dp_with_strategy(0, rounding);
    rounded_steps
        .checked_mul(increment)
        .ok_or(NumericError::Overflow {
            operation: "quantize multiplication",
        })
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn quantized(value: &str, increment: &str, rounding: RoundingStrategy) -> String {
        quantize(
            Decimal::from_str_exact(value).unwrap(),
            Decimal::from_str_exact(increment).unwrap(),
            rounding,
        )
        .unwrap()
        .to_string()
    }

    #[test]
    fn quantizes_with_each_rounding_strategy() {
        assert_eq!(quantized("1.235", "0.01", RoundingStrategy::ToZero), "1.23");
        assert_eq!(
            quantized("1.235", "0.01", RoundingStrategy::AwayFromZero),
            "1.24"
        );
        assert_eq!(
            quantized("1.235", "0.01", RoundingStrategy::ToNegativeInfinity),
            "1.23"
        );
        assert_eq!(
            quantized("1.235", "0.01", RoundingStrategy::ToPositiveInfinity),
            "1.24"
        );
        assert_eq!(
            quantized("1.235", "0.01", RoundingStrategy::MidpointNearestEven),
            "1.24"
        );
        assert_eq!(
            quantized("1.235", "0.01", RoundingStrategy::MidpointAwayFromZero),
            "1.24"
        );
        assert_eq!(
            quantized("1.235", "0.01", RoundingStrategy::MidpointTowardZero),
            "1.23"
        );
    }

    #[test]
    fn decimal_f64_decimal_round_trip_is_exact_for_quantized_ticks() {
        let price =
            Price::try_from_calc_quantized(0.012_345, 0.001, RoundingStrategy::ToZero).unwrap();

        assert_eq!(
            price.to_decimal(),
            Decimal::from_str_exact("0.012").unwrap()
        );
        assert_eq!(
            Price::try_from_calc_quantized(price.to_calc(), 0.001, RoundingStrategy::ToZero)
                .unwrap(),
            price
        );
    }

    #[test]
    fn rejects_non_finite_calculated_values() {
        assert!(Price::try_from_calc(f64::NAN).is_err());
        assert!(Quantity::try_from_calc(f64::INFINITY).is_err());
        assert!(Notional::try_from_calc(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn rejects_non_positive_values_and_increments() {
        assert!(Price::try_from_calc(0.0).is_err());
        assert!(Price::try_from_calc(-1.0).is_err());
        assert!(Quantity::try_from_calc_quantized(1.0, 0.0, RoundingStrategy::ToZero).is_err());
        assert!(Notional::try_from_calc_quantized(1.0, -0.01, RoundingStrategy::ToZero).is_err());
    }
}
