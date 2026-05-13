pub struct Preferences {
    pub multi_leg: MultiLeg,
}

pub struct MultiLeg {
    pub if_different_symbol_unsupported: OnDifferent,
    pub if_different_price_unsupported: OnDifferent,
}

#[derive(Debug, Clone, Copy)]
pub enum OnDifferent {
    PreferPrimary,
    SkipOrder,
    CancelAll,
}
