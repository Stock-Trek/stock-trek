#[cfg(feature = "python")]
use {
    crate::aligned_window::AlignedWindow,
    pyo3::{pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "AlignedWindow")]
pub struct PyAlignedWindow {
    inner: AlignedWindow,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyAlignedWindow {
    pub const SECONDS_5: Self = Self {
        inner: AlignedWindow::Seconds5,
    };
    pub const SECONDS_10: Self = Self {
        inner: AlignedWindow::Seconds10,
    };
    pub const SECONDS_15: Self = Self {
        inner: AlignedWindow::Seconds15,
    };
    pub const SECONDS_30: Self = Self {
        inner: AlignedWindow::Seconds30,
    };
    pub const SECONDS_60: Self = Self {
        inner: AlignedWindow::Seconds60,
    };
    pub const MINUTES_1: Self = Self {
        inner: AlignedWindow::Minutes1,
    };
    pub const MINUTES_2: Self = Self {
        inner: AlignedWindow::Minutes2,
    };
    pub const MINUTES_3: Self = Self {
        inner: AlignedWindow::Minutes3,
    };
    pub const MINUTES_5: Self = Self {
        inner: AlignedWindow::Minutes5,
    };
    pub const MINUTES_10: Self = Self {
        inner: AlignedWindow::Minutes10,
    };
    pub const MINUTES_15: Self = Self {
        inner: AlignedWindow::Minutes15,
    };
    pub const MINUTES_30: Self = Self {
        inner: AlignedWindow::Minutes30,
    };
    pub const MINUTES_60: Self = Self {
        inner: AlignedWindow::Minutes60,
    };
    pub const HOURS_1: Self = Self {
        inner: AlignedWindow::Hours1,
    };
    pub const HOURS_2: Self = Self {
        inner: AlignedWindow::Hours2,
    };
    pub const HOURS_3: Self = Self {
        inner: AlignedWindow::Hours3,
    };
    pub const HOURS_4: Self = Self {
        inner: AlignedWindow::Hours4,
    };
    pub const HOURS_6: Self = Self {
        inner: AlignedWindow::Hours6,
    };
    pub const HOURS_8: Self = Self {
        inner: AlignedWindow::Hours8,
    };
    pub const HOURS_12: Self = Self {
        inner: AlignedWindow::Hours12,
    };
    pub const HOURS_24: Self = Self {
        inner: AlignedWindow::Hours24,
    };
    pub const DAYS_1: Self = Self {
        inner: AlignedWindow::Days1,
    };
    pub const DAYS_7: Self = Self {
        inner: AlignedWindow::Days7,
    };
    pub const WEEKS_1: Self = Self {
        inner: AlignedWindow::Weeks1,
    };
    pub const WEEKS_2: Self = Self {
        inner: AlignedWindow::Weeks2,
    };
    pub const WEEKS_4: Self = Self {
        inner: AlignedWindow::Weeks4,
    };
    pub const MONTHS_1: Self = Self {
        inner: AlignedWindow::Months1,
    };
    pub const MONTHS_2: Self = Self {
        inner: AlignedWindow::Months2,
    };
    pub const MONTHS_3: Self = Self {
        inner: AlignedWindow::Months3,
    };
    pub const MONTHS_4: Self = Self {
        inner: AlignedWindow::Months4,
    };
    pub const MONTHS_6: Self = Self {
        inner: AlignedWindow::Months6,
    };
    pub const MONTHS_12: Self = Self {
        inner: AlignedWindow::Months12,
    };
    pub const YEARS_1: Self = Self {
        inner: AlignedWindow::Years1,
    };
    pub const YEARS_2: Self = Self {
        inner: AlignedWindow::Years2,
    };
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

#[cfg(feature = "python")]
impl From<PyAlignedWindow> for AlignedWindow {
    fn from(py: PyAlignedWindow) -> Self {
        py.inner
    }
}

#[cfg(feature = "python")]
impl From<AlignedWindow> for PyAlignedWindow {
    fn from(inner: AlignedWindow) -> Self {
        Self { inner: inner }
    }
}
