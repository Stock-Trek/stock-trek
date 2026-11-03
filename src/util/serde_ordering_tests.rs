use std::cmp::Ordering;

use crate::util::serde_ordering::OrderingSerde;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Wrapper {
    #[serde(with = "OrderingSerde")]
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
fn rejects_invalid_strings() {
    assert!(serde_json::from_str::<Wrapper>("{\"ordering\":\"invalid\"}").is_err());
}
