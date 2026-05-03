use crate::exchanges::{exchange::Exchange, stub_exchange::StubExchange};

pub struct ExchangeFactory;

impl ExchangeFactory {
    pub fn stub() -> Exchange {
        StubExchange::new().into()
    }
    // TODO simulated, mocked, delayed, failing
}
