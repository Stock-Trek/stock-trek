use crate::exchanges::{exchange::Exchange, stub_exchange::StubExchange};

pub struct ExchangeFactory;

impl ExchangeFactory {
    pub fn stub() -> Exchange {
        StubExchange.into()
    }
    // TODO simulated, mocked, delayed, failing
}
