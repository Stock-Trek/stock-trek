use crate::{
    exchanges::{bot_id::BotId, exchange::Exchange},
    portfolios::portfolio::Portfolio,
    scratch::{key::ExchangeName, scratch_pad::ScratchPad},
};
use std::collections::HashMap;

pub struct ResolvedContext {
    pub bot_id: BotId,
    pub exchanges: HashMap<ExchangeName, Exchange>,
    pub portfolio: Portfolio,
    pub scratch_pad: ScratchPad,
}
