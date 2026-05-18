use rust_decimal::RoundingStrategy;
use serde::de::Error;
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use strum::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub rounding: Rounding,
    pub multi_leg: MultiLeg,
}

#[derive(Debug, Clone)]
pub struct Rounding {
    pub price: RoundingStrategy,
    pub quantity: RoundingStrategy,
    pub callback_rate: RoundingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Serialize for Rounding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Rounding {
            price,
            quantity,
            callback_rate,
        } = self;
        let mut state = serializer.serialize_struct("Rounding", 3)?;
        state.serialize_field("price", &serialize_rounding(price))?;
        state.serialize_field("quantity", &serialize_rounding(quantity))?;
        state.serialize_field("callback_rate", &serialize_rounding(callback_rate))?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Rounding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RoundingHelper {
            price: String,
            quantity: String,
            callback_rate: String,
        }

        let helper = RoundingHelper::deserialize(deserializer)?;

        Ok(Rounding {
            price: deserialize_rounding(&helper.price)
                .map_err(|e| D::Error::custom(format!("Invalid price rounding: {}", e)))?,
            quantity: deserialize_rounding(&helper.quantity)
                .map_err(|e| D::Error::custom(format!("Invalid quantity rounding: {}", e)))?,
            callback_rate: deserialize_rounding(&helper.callback_rate)
                .map_err(|e| D::Error::custom(format!("Invalid callback_rate rounding: {}", e)))?,
        })
    }
}

#[allow(deprecated)]
fn deserialize_rounding(text: &str) -> Result<RoundingStrategy, String> {
    match text {
        "AwayFromZero" => Ok(RoundingStrategy::AwayFromZero),
        "MidpointAwayFromZero" => Ok(RoundingStrategy::MidpointAwayFromZero),
        "MidpointNearestEven" => Ok(RoundingStrategy::MidpointNearestEven),
        "MidpointTowardZero" => Ok(RoundingStrategy::MidpointTowardZero),
        "ToNegativeInfinity" => Ok(RoundingStrategy::ToNegativeInfinity),
        "ToPositiveInfinity" => Ok(RoundingStrategy::ToPositiveInfinity),
        "ToZero" => Ok(RoundingStrategy::ToZero),
        "BankersRounding" => Ok(RoundingStrategy::BankersRounding),
        "RoundDown" => Ok(RoundingStrategy::RoundDown),
        "RoundHalfDown" => Ok(RoundingStrategy::RoundHalfDown),
        "RoundHalfUp" => Ok(RoundingStrategy::RoundHalfUp),
        "RoundUp" => Ok(RoundingStrategy::RoundUp),
        _ => Err(text.to_string()),
    }
}

#[allow(deprecated)]
fn serialize_rounding(strategy: &RoundingStrategy) -> &str {
    match *strategy {
        RoundingStrategy::AwayFromZero => "AwayFromZero",
        RoundingStrategy::MidpointAwayFromZero => "MidpointAwayFromZero",
        RoundingStrategy::MidpointNearestEven => "MidpointNearestEven",
        RoundingStrategy::MidpointTowardZero => "MidpointTowardZero",
        RoundingStrategy::ToNegativeInfinity => "ToNegativeInfinity",
        RoundingStrategy::ToPositiveInfinity => "ToPositiveInfinity",
        RoundingStrategy::ToZero => "ToZero",
        RoundingStrategy::BankersRounding => "BankersRounding",
        RoundingStrategy::RoundDown => "RoundDown",
        RoundingStrategy::RoundHalfDown => "RoundHalfDown",
        RoundingStrategy::RoundHalfUp => "RoundHalfUp",
        RoundingStrategy::RoundUp => "RoundUp",
    }
}
