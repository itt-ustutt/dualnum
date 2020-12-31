use num_hyperdual::*;
use pyo3::prelude::*;

#[pyclass(name="Dual64")]
#[derive(Clone)]
/// Dual number using 64-bit-floats as fields.
///
/// A dual number consists of
/// a + b ε
///
/// # Examples
///
/// >>> from hyperdual import Dual64 as D64
/// >>> x = D64(1.0, 0.0)
/// >>> y = D64.from_re(2.0)
/// >>> x + y
/// 3 + 0ε
pub struct PyDual64 {
    pub _data: Dual64,
}

impl From<Dual64> for PyDual64 {
    fn from(d: Dual64) -> Self {
        Self { _data: d }
    }
}

impl From<PyDual64> for Dual64 {
    fn from(d: PyDual64) -> Self {
        d._data
    }
}

#[pymethods]
impl PyDual64 {
    #[new]
    pub fn new(re: f64, eps: f64) -> Self {
        Self {
            _data: Dual64::new(re, eps),
        }
    }

    #[getter]
    /// Dual part.
    pub fn get_eps(&self) -> f64 {
        self._data.eps
    }
}
