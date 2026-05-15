use crate::order::order_price_basis::OrderPriceBasis;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderConstraint {
    NotionalCap(Decimal),
    PriceDeviationCap {
        basis: OrderPriceBasis,
        max_deviation_bps: Decimal,
    },
    SlippageCap {
        max_slippage_bps: Decimal,
    },
    FillPolicy {
        allow_partial: bool,
    },
}
