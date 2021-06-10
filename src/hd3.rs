use crate::dual::PyDual64;
use num_hyperdual::*;
use pyo3::prelude::*;

#[pyclass(name = "HD3_64")]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats.
pub struct PyHD3_64 {
    pub _data: HD3_64,
}

#[pymethods]
impl PyHD3_64 {
    #[new]
    fn new(eps: f64, v1: f64, v2: f64, v3: f64) -> Self {
        HD3::new(eps, v1, v2, v3).into()
    }
}

#[pyclass(name = "HD3Dual64")]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats.
pub struct PyHD3Dual64 {
    pub _data: HD3Dual64,
}

#[pymethods]
impl PyHD3Dual64 {
    #[new]
    pub fn new(v0: PyDual64, v1: PyDual64, v2: PyDual64, v3: PyDual64) -> Self {
        HD3::new(v0.into(), v1.into(), v2.into(), v3.into()).into()
    }
}
