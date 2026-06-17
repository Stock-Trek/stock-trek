use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderTag(pub String);

impl OrderTag {
    pub fn new(tag: &str) -> Self {
        OrderTag(tag.to_string())
    }
}
