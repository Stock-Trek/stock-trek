#[cfg(feature = "python")]
use {
    crate::{
        bindings::python::{market_data::py_market::PyMarket, py_trading_pair::PyTradingPair},
        exchange::Exchange,
    },
    pyo3::{prelude::*, types::PyDict},
};

#[cfg(feature = "python")]
#[pyclass(name = "Exchange")]
pub struct PyExchange {
    inner: Exchange,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyExchange {
    pub fn markets(&self, py: Python<'_>) -> Py<PyDict> {
        let dict = PyDict::new(py);
        for (key, value) in self.inner.markets() {
            let py_key = PyTradingPair::from(key);
            let py_market = PyMarket::from(value);
            dict.set_item(py_key, py_market).unwrap();
        }
        dict.into()
    }
}

#[cfg(feature = "python")]
impl From<Exchange> for PyExchange {
    fn from(exchange: Exchange) -> Self {
        Self { inner: exchange }
    }
}
