use crate::dual::PyDual64;
use num_hyperdual::*;
use pyo3::exceptions::PyTypeError;
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

    #[getter]
    /// First hyperdual part.
    fn get_v1(&self) -> f64 {
        self._data.v1
    }

    #[getter]
    /// Second hyperdual part.
    fn get_v2(&self) -> f64 {
        self._data.v2
    }

    #[getter]
    /// Third hyperdual part.
    fn get_v3(&self) -> f64 {
        self._data.v3
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
        HD3::new(v0._data, v1._data, v2._data, v3._data).into()
    }

    #[getter]
    /// First hyperdual part.
    fn get_v1(&self) -> PyDual64 {
        self._data.v1.into()
    }

    #[getter]
    /// Second hyperdual part.
    fn get_v2(&self) -> PyDual64 {
        self._data.v2.into()
    }

    #[getter]
    /// Third hyperdual part.
    fn get_v3(&self) -> PyDual64 {
        self._data.v3.into()
    }
}

#[pymethods]
impl PyHD3_64 {
    #[staticmethod]
    fn derive(x: &PyAny) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(x) = x.extract::<f64>() {
                return Ok(
                    PyCell::new(py, PyHD3_64::from(HD3_64::from_re(x).derive()))?.to_object(py),
                );
            };
            if let Ok(x) = x.extract::<PyDual64>() {
                return Ok(PyCell::new(
                    py,
                    PyHD3Dual64::from(HD3Dual64::from_re(x._data).derive()),
                )?
                .to_object(py));
            };
            Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
        })
    }
}
