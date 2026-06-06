use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Unexpected},
};
use std::cmp::Ordering;

pub fn serialize<S: Serializer>(ordering: &Ordering, serializer: S) -> Result<S::Ok, S::Error> {
    (*ordering as i8).serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Ordering, D::Error>
where
    D: Deserializer<'de>,
{
    i8::deserialize(deserializer).and_then(|i| match i {
        -1 => Ok(Ordering::Less),
        0 => Ok(Ordering::Equal),
        1 => Ok(Ordering::Greater),
        _ => Err(D::Error::invalid_value(
            Unexpected::Signed(i.into()),
            &"an integer in the range of -1i8..=1i8",
        )),
    })
}
