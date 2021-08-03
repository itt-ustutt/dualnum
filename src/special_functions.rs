use super::*;
use pyo3::exceptions::PyTypeError;

/// 0th order spherical bessel function of the first kind
#[pyfunction]
#[pyo3(text_signature = "(x)")]
pub fn sph_j0(x: &PyAny) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        if let Ok(x) = x.extract::<f64>() {
            return Ok(x.sph_j0().to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual64>() {
            return Ok(PyCell::new(py, x.sph_j0())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyHyperDual64>() {
            return Ok(PyCell::new(py, x.sph_j0())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual2_64>() {
            return Ok(PyCell::new(py, x.sph_j0())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual3_64>() {
            return Ok(PyCell::new(py, x.sph_j0())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyHyperDualDual64>() {
            return Ok(PyCell::new(py, x.sph_j0())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual2Dual64>() {
            return Ok(PyCell::new(py, x.sph_j0())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual3Dual64>() {
            return Ok(PyCell::new(py, x.sph_j0())?.to_object(py));
        };
        Err(PyErr::new::<PyTypeError, _>(format!(
            "sph_j0 not implemented for this data type!"
        )))
    })
}

/// 1st order spherical bessel function of the first kind
#[pyfunction]
#[pyo3(text_signature = "(x)")]
pub fn sph_j1(x: &PyAny) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        if let Ok(x) = x.extract::<f64>() {
            return Ok(x.sph_j1().to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual64>() {
            return Ok(PyCell::new(py, x.sph_j1())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyHyperDual64>() {
            return Ok(PyCell::new(py, x.sph_j1())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual2_64>() {
            return Ok(PyCell::new(py, x.sph_j1())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual3_64>() {
            return Ok(PyCell::new(py, x.sph_j1())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyHyperDualDual64>() {
            return Ok(PyCell::new(py, x.sph_j1())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual2Dual64>() {
            return Ok(PyCell::new(py, x.sph_j1())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual3Dual64>() {
            return Ok(PyCell::new(py, x.sph_j1())?.to_object(py));
        };
        Err(PyErr::new::<PyTypeError, _>(format!(
            "sph_j1 not implemented for this data type!"
        )))
    })
}

/// 2nd order spherical bessel function of the first kind
#[pyfunction]
#[pyo3(text_signature = "(x)")]
pub fn sph_j2(x: &PyAny) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        if let Ok(x) = x.extract::<f64>() {
            return Ok(x.sph_j2().to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual64>() {
            return Ok(PyCell::new(py, x.sph_j2())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyHyperDual64>() {
            return Ok(PyCell::new(py, x.sph_j2())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual2_64>() {
            return Ok(PyCell::new(py, x.sph_j2())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual3_64>() {
            return Ok(PyCell::new(py, x.sph_j2())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyHyperDualDual64>() {
            return Ok(PyCell::new(py, x.sph_j2())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual2Dual64>() {
            return Ok(PyCell::new(py, x.sph_j2())?.to_object(py));
        };
        if let Ok(x) = x.extract::<PyDual3Dual64>() {
            return Ok(PyCell::new(py, x.sph_j2())?.to_object(py));
        };
        Err(PyErr::new::<PyTypeError, _>(format!(
            "sph_j1 not implemented for this data type!"
        )))
    })
}
