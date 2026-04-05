use rust_decimal::Decimal;

#[derive(Debug)]
pub struct RawDecimal {
    pub lo: u32,
    pub mid: u32,
    pub hi: u32,
    pub negative: bool,
    pub scale: u32,
}

impl From<RawDecimal> for Decimal {
    fn from(value: RawDecimal) -> Self {
        let RawDecimal {
            lo,
            mid,
            hi,
            negative,
            scale,
        } = value;
        Decimal::from_parts(lo, mid, hi, negative, scale)
    }
}
