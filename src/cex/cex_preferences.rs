use crate::util::serde_rounding_strategy;
use rust_decimal::RoundingStrategy;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CexPreferences {
    pub max_network_delay_millis: u32,
    pub rounding: Rounding,
    // pub multi_leg: MultiLeg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rounding {
    #[serde(with = "serde_rounding_strategy")]
    pub activation_price_triggered_above: RoundingStrategy,
    #[serde(with = "serde_rounding_strategy")]
    pub activation_price_triggered_below: RoundingStrategy,
    #[serde(with = "serde_rounding_strategy")]
    pub price: RoundingStrategy,
    #[serde(with = "serde_rounding_strategy")]
    pub quantity: RoundingStrategy,
    #[serde(with = "serde_rounding_strategy")]
    pub callback_rate_bps: RoundingStrategy,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MultiLeg {
//     pub if_different_symbol_unsupported: OnDifferent,
//     pub if_different_price_unsupported: OnDifferent,
// }

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OnDifferent {
    UseDataFromPrimary,
    SkipThisOrder,
    CancelEntireIteration,
}
