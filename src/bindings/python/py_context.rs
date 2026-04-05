#[cfg(feature = "python")]
use {
    crate::{bindings::python::py_exchange::PyExchange, context::StockTrekContext},
    pyo3::{prelude::*, types::PyDict},
};

#[cfg(feature = "python")]
#[pyclass(name = "StockTrekContext")]
pub struct PyStockTrekContext {
    inner: StockTrekContext,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStockTrekContext {
    pub fn exchanges(&self, py: Python<'_>) -> Py<PyDict> {
        let dict = PyDict::new(py);
        for (key, value) in self.inner.exchanges() {
            let py_key = key;
            let py_market = PyExchange::from(value);
            dict.set_item(py_key, py_market).unwrap();
        }
        dict.into()
    }
}

#[cfg(feature = "python")]
impl From<StockTrekContext> for PyStockTrekContext {
    fn from(exchange: StockTrekContext) -> Self {
        Self { inner: exchange }
    }
}
