use crate::dual::PyDual64;
use num_hyperdual::*;
use pyo3::prelude::*;

#[pyclass(name=HyperDual64)]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats as fields.
///
/// A hyper dual number consists of
/// a + b ε1 + c ε2 + d ε1ε2
///
/// # Examples
///
/// >>> from hyperdual import HyperDual64 as HD64
/// >>> x = HD64(1.0, 0.0, 0.0, 0.0)
/// >>> y = HD64.from_re(2.0)
/// >>> x + y
/// 3 + 0ε1 + 0ε2 + 0ε1ε2
pub struct PyHyperDual64 {
    pub _data: HyperDual<f64>,
}

impl From<HyperDual<f64>> for PyHyperDual64 {
    fn from(hd: HyperDual<f64>) -> Self {
        Self { _data: hd }
    }
}

impl From<PyHyperDual64> for HyperDual<f64> {
    fn from(d: PyHyperDual64) -> Self {
        d._data
    }
}

#[pymethods]
impl PyHyperDual64 {
    #[new]
    pub fn new(re: f64, eps1: f64, eps2: f64, eps1eps2: f64) -> Self {
        Self {
            _data: HyperDual::new(re, eps1, eps2, eps1eps2),
        }
    }

    #[getter]
    /// First hyperdual part.
    fn get_eps1(&self) -> f64 {
        self._data.eps1
    }

    #[getter]
    /// Second hyerdual part.
    fn get_eps2(&self) -> f64 {
        self._data.eps2
    }

    #[getter]
    /// Third hyerdual part.
    fn get_eps1eps2(&self) -> f64 {
        self._data.eps1eps2
    }
}

#[pyclass(name=HyperDualDual64)]
#[derive(Clone)]
/// Hyper dual number using dual numbers of 64-bit-floats.
pub struct PyHyperDualDual64 {
    pub _data: HyperDualDual64,
}

impl From<HyperDualDual64> for PyHyperDualDual64 {
    fn from(hd: HyperDualDual64) -> Self {
        Self { _data: hd }
    }
}

impl From<PyHyperDualDual64> for HyperDualDual64 {
    fn from(d: PyHyperDualDual64) -> Self {
        d._data
    }
}

#[pymethods]
impl PyHyperDualDual64 {
    #[new]
    pub fn new(re: PyDual64, eps1: PyDual64, eps2: PyDual64, eps1eps2: PyDual64) -> Self {
        Self {
            _data: HyperDualDual64::new(re._data, eps1._data, eps2._data, eps1eps2._data),
        }
    }

    #[getter]
    /// First hyperdual part.
    fn get_eps1(&self) -> PyDual64 {
        self._data.eps1.into()
    }

    #[getter]
    /// Second hyerdual part.
    fn get_eps2(&self) -> PyDual64 {
        self._data.eps2.into()
    }

    #[getter]
    /// Third hyerdual part.
    fn get_eps1eps2(&self) -> PyDual64 {
        self._data.eps1eps2.into()
    }
}
