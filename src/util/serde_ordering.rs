use serde::{
    Deserializer, Serializer,
    de::{Error, Unexpected, Visitor},
};
use std::{cmp::Ordering, fmt};

/// Serializes an [`Ordering`] as a snake_case string.
pub struct OrderingSerde;

impl OrderingSerde {
    pub fn serialize<S: Serializer>(ordering: &Ordering, serializer: S) -> Result<S::Ok, S::Error> {
        let value = match *ordering {
            Ordering::Less => "less",
            Ordering::Equal => "equal",
            Ordering::Greater => "greater",
        };
        serializer.serialize_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Ordering, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OrderingVisitor)
    }
}

struct OrderingVisitor;

impl Visitor<'_> for OrderingVisitor {
    type Value = Ordering;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("`less`, `equal`, or `greater`")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "less" => Ok(Ordering::Less),
            "equal" => Ok(Ordering::Equal),
            "greater" => Ok(Ordering::Greater),
            _ => Err(E::invalid_value(Unexpected::Str(value), &self)),
        }
    }
}

#[cfg(test)]
mod tests;
