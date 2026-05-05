use crate::exchanges::order_capability::OrderCapability;
use digdigdig3::ExchangeId;
use std::collections::{HashMap, HashSet};

pub struct ExchangeCapabilities {
    capabilities: HashMap<ExchangeId, HashSet<OrderCapability>>,
}

impl ExchangeCapabilities {
    pub fn has_capability(
        &self,
        exchange: &ExchangeId,
        capability: &OrderCapability,
    ) -> Option<bool> {
        self.capabilities
            .get(exchange)
            .map(|e| e.contains(capability))
    }
}
