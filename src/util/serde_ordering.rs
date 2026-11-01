use serde::{
    Deserializer, Serializer,
    de::{Error, Unexpected, Visitor},
};
use std::{cmp::Ordering, fmt};

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
    deserializer.deserialize_any(OrderingVisitor)
}

struct OrderingVisitor;

impl Visitor<'_> for OrderingVisitor {
    type Value = Ordering;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("`less`, `equal`, or `greater` (or -1, 0, 1)")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "less" => Ok(Ordering::Less),
            "equal" => Ok(Ordering::Equal),
            "greater" => Ok(Ordering::Greater),
            _ => Err(E::invalid_value(Unexpected::Str(value), &self)),
        }
    }

    fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
        match value {
            -1 => Ok(Ordering::Less),
            0 => Ok(Ordering::Equal),
            1 => Ok(Ordering::Greater),
            _ => Err(E::invalid_value(Unexpected::Signed(value), &self)),
        }
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
        self.visit_i64(i64::try_from(value).map_err(|_| {
            E::invalid_value(
                Unexpected::Unsigned(value),
                &"an integer in the range of -1i8..=1i8",
            )
        })?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::serde_ordering;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Wrapper {
        #[serde(with = "serde_ordering")]
        ordering: Ordering,
    }

    #[test]
    fn serializes_as_snake_case_strings() {
        let cases = [
            (Ordering::Less, "\"less\""),
            (Ordering::Equal, "\"equal\""),
            (Ordering::Greater, "\"greater\""),
        ];

        for (ordering, expected) in cases {
            assert_eq!(
                serde_json::to_string(&Wrapper { ordering }).unwrap(),
                format!("{{\"ordering\":{expected}}}")
            );
        }
    }

    #[test]
    fn deserializes_snake_case_strings() {
        for (json, expected) in [
            ("{\"ordering\":\"less\"}", Ordering::Less),
            ("{\"ordering\":\"equal\"}", Ordering::Equal),
            ("{\"ordering\":\"greater\"}", Ordering::Greater),
        ] {
            let wrapper: Wrapper = serde_json::from_str(json).unwrap();
            assert_eq!(wrapper.ordering, expected);
        }
    }

    #[test]
    fn deserializes_legacy_integers() {
        for (json, expected) in [
            ("{\"ordering\":-1}", Ordering::Less),
            ("{\"ordering\":0}", Ordering::Equal),
            ("{\"ordering\":1}", Ordering::Greater),
        ] {
            let wrapper: Wrapper = serde_json::from_str(json).unwrap();
            assert_eq!(wrapper.ordering, expected);
        }
    }

    #[test]
    fn rejects_invalid_values() {
        assert!(serde_json::from_str::<Wrapper>("{\"ordering\":2}").is_err());
        assert!(serde_json::from_str::<Wrapper>("{\"ordering\":\"invalid\"}").is_err());
    }
}
