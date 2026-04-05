#[cfg(feature = "python")]
use {
    crate::bindings::python::{
        market_data::py_market_window::{PyAlignedWindow, PyRollingWindow},
        py_algorithm::PyStockTrekAlgorithm,
        py_context::PyStockTrekContext,
        py_exchange::PyExchange,
        py_signal::*,
        py_trading_pair::PyTradingPair,
    },
    pyo3::prelude::*,
};

#[cfg(feature = "python")]
pub fn create_module_signal(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "signal")?;
    module.add_class::<PyConfidencePercentageChanges>()?;
    module.add_class::<PyConfidenceProbabilities>()?;
    module.add_class::<PyConfidenceTimings>()?;
    module.add_class::<PyEvent>()?;
    module.add_class::<PyGenerator>()?;
    module.add_class::<PyHorizonConfidencesByMillis>()?;
    module.add_class::<PyInstrument>()?;
    module.add_class::<PyMarketContext>()?;
    module.add_class::<PyMarketRegime>()?;
    module.add_class::<PyMarketRegimeClassifications>()?;
    module.add_class::<PyMarketRegimeCycle>()?;
    module.add_class::<PyMarketRegimeTrend>()?;
    module.add_class::<PyMarketRegimeVolatility>()?;
    module.add_class::<PyMarketRegimeVolatilitySnapshot>()?;
    module.add_class::<PyMarketRegimeVolatilityTrend>()?;
    module.add_class::<PyMetadata>()?;
    module.add_class::<PyPrediction>()?;
    module.add_class::<PyPredictionRisk>()?;
    module.add_class::<PyPredictionRiskPercentageRisks>()?;
    module.add_class::<PyProvenance>()?;
    module.add_class::<PyRegimePersistence>()?;
    module.add_class::<PyStockTrekSignal>()?;
    Ok(module)
}

#[cfg(feature = "python")]
pub fn create_module_window(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new(py, "window")?;
    module.add_class::<PyAlignedWindow>()?;
    module.add_class::<PyRollingWindow>()?;
    Ok(module)
}

#[cfg(feature = "python")]
#[pymodule]
pub fn _stock_trek(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_submodule(&super::dto::python::create_module(py)?)?;
    module.add_submodule(&super::market_data::python::create_module(py)?)?;
    module.add_submodule(&super::statistics::python::create_module(py)?)?;
    module.add_submodule(&create_module_signal(py)?)?;
    module.add_submodule(&create_module_window(py)?)?;
    module.add_class::<PyExchange>()?;
    module.add_class::<PyStockTrekAlgorithm>()?;
    module.add_class::<PyStockTrekContext>()?;
    module.add_class::<PyStockTrekSignal>()?;
    module.add_class::<PyTradingPair>()?;
    Ok(())
}
