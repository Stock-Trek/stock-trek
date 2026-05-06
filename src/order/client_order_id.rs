use chrono::Utc;
use std::hash::{DefaultHasher, Hash, Hasher};
use uuid::Uuid;

pub struct ClientOrderId;

impl ClientOrderId {
    pub fn create<T: Hash>(order_request: &T) -> String {
        let mut state = DefaultHasher::new();
        order_request.hash(&mut state);
        let high_bits = Utc::now().timestamp_millis() as u64;
        let low_bits = state.finish();
        Uuid::from_u64_pair(high_bits, low_bits).to_string()
    }
}
