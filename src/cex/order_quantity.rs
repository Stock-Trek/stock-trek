use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    resolvable::Resolvable,
    resolved_context::ResolvedContext,
    value::value::NumberValue,
};
use stock_trek_types::cex::quantity::Quantity;

impl Resolvable<Quantity<f64>> for Quantity<NumberValue> {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<Quantity<f64>> {
        match self {
            Quantity::OfBase(q) => Ok(Quantity::OfBase(positive_quantity(
                "Quantity::OfBase",
                "base quantity",
                q.number(context)?,
            )?)),
            Quantity::OfQuote(q) => Ok(Quantity::OfQuote(positive_quantity(
                "Quantity::OfQuote",
                "quote quantity",
                q.number(context)?,
            )?)),
        }
    }
}

pub(crate) fn positive_quantity(
    function: &'static str,
    field: &'static str,
    value: f64,
) -> StockTrekResult<f64> {
    if !value.is_finite() {
        return Err(StockTrekError::Stats(StatsError::DomainError {
            function,
            message: format!("{field} {value} is not finite (NaN or infinity)"),
        }));
    }
    if value <= 0.0 {
        return Err(StockTrekError::Stats(StatsError::DomainError {
            function,
            message: format!("{field} {value} must be positive"),
        }));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{portfolio::Portfolio, signal::signals::Signals};
    use std::collections::HashMap;

    fn context() -> ResolvedContext {
        ResolvedContext {
            enqueue_action: Box::new(|_, _| Ok(())),
            portfolio: Portfolio::new(HashMap::new()),
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
}
