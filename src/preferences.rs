use serde::{Deserialize, Serialize};
use strum::Display;

pub struct Preferences {
    pub multi_leg: MultiLeg,
}

pub struct MultiLeg {
    pub if_different_symbol_unsupported: OnDifferent,
    pub if_different_price_unsupported: OnDifferent,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OnDifferent {
    UseDataFromPrimary,
    SkipThisOrder,
    CancelEntireIteration,
}
