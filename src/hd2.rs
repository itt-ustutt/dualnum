use crate::dual::PyDual64;
use num_hyperdual::*;
use pyo3::prelude::*;

#[pyclass(name = "HD2_64")]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats.
pub struct PyHD2_64 {
    pub _data: HD2_64,
}

#[pymethods]
impl PyHD2_64 {
    #[new]
    fn new(eps: f64, v1: f64, v2: f64) -> Self {
        HD2::new(eps, v1, v2).into()
    }
}

#[pyclass(name = "HD2Dual64")]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats.
pub struct PyHD2Dual64 {
    pub _data: HD2Dual64,
}

#[pymethods]
impl PyHD2Dual64 {
    #[new]
    pub fn new(v0: PyDual64, v1: PyDual64, v2: PyDual64) -> Self {
        HD2::new(v0.into(), v1.into(), v2.into()).into()
    }
}
