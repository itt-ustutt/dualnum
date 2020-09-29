use crate::dual::PyDual64;
use num_hyperdual::*;
use pyo3::prelude::*;

#[pyclass(name=HD3_64)]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats.
pub struct PyHD3_64 {
    pub _data: HD3_64,
}

impl From<HD3_64> for PyHD3_64 {
    fn from(hd: HD3_64) -> Self {
        Self { _data: hd }
    }
}

impl From<PyHD3_64> for HD3_64 {
    fn from(d: PyHD3_64) -> Self {
        d._data
    }
}

#[pymethods]
impl PyHD3_64 {
    #[new]
    fn new(v: [f64; 4]) -> Self {
        HD3::new(v).into()
    }
}

#[pyclass(name=HD3Dual64)]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats.
pub struct PyHD3Dual64 {
    pub _data: HD3Dual64,
}

impl From<HD3Dual64> for PyHD3Dual64 {
    fn from(hd: HD3Dual64) -> Self {
        Self { _data: hd }
    }
}

impl From<PyHD3Dual64> for HD3Dual64 {
    fn from(d: PyHD3Dual64) -> Self {
        d._data
    }
}

#[pymethods]
impl PyHD3Dual64 {
    #[new]
    pub fn new(v0: PyDual64, v1: PyDual64, v2: PyDual64, v3: PyDual64) -> Self {
        HD3::new([v0.into(), v1.into(), v2.into(), v3.into()]).into()
    }
}
