#[cfg(feature = "python")]
use {
    crate::trading_pair::TradingPair,
    pyo3::{class::basic::CompareOp, prelude::*, IntoPyObjectExt},
    std::hash::{DefaultHasher, Hash, Hasher},
};

#[cfg(feature = "python")]
#[derive(Clone)]
#[pyclass(name = "TradingPair", from_py_object)]
pub struct PyTradingPair {
    inner: TradingPair,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyTradingPair {
    #[new]
    pub fn new(base: String, quote: String) -> Self {
        Self {
            inner: TradingPair::new(base, quote),
        }
    }
    pub fn base(&self) -> String {
        self.inner.base().to_owned()
    }
    pub fn quote(&self) -> String {
        self.inner.quote().to_owned()
    }
    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        hasher.finish()
    }
    pub fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: CompareOp,
        py: Python<'_>,
    ) -> Py<PyAny> {
        if let Ok(other_pair) = other.extract::<PyTradingPair>() {
            match op {
                CompareOp::Eq => (self.inner == other_pair.inner)
                    .into_py_any(py)
                    .unwrap()
                    .into_any(),
                CompareOp::Ne => (self.inner != other_pair.inner)
                    .into_py_any(py)
                    .unwrap()
                    .into_any(),
                _ => py.NotImplemented(),
            }
        } else {
            py.NotImplemented()
        }
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

#[cfg(feature = "python")]
impl From<TradingPair> for PyTradingPair {
    fn from(pair: TradingPair) -> Self {
        Self::new(pair.base().to_owned(), pair.quote().to_owned())
    }
}
