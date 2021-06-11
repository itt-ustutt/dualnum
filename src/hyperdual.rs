use crate::dual::PyDual64;
use crate::hd2::{PyHD2Dual64, PyHD2_64};
use num_hyperdual::*;
use pyo3::exceptions::PyTypeError;
use pyo3::number::PyNumberProtocol;
use pyo3::prelude::*;

#[pyclass(name = "HyperDual64")]
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
    /// Second hyperdual part.
    fn get_eps2(&self) -> f64 {
        self._data.eps2
    }

    #[getter]
    /// Third hyperdual part.
    fn get_eps1eps2(&self) -> f64 {
        self._data.eps1eps2
    }
}

#[pyclass(name = "HyperDualDual64")]
#[derive(Clone)]
/// Hyper dual number using dual numbers of 64-bit-floats.
pub struct PyHyperDualDual64 {
    pub _data: HyperDualDual64,
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
    /// Second hyperdual part.
    fn get_eps2(&self) -> PyDual64 {
        self._data.eps2.into()
    }

    #[getter]
    /// Third hyperdual part.
    fn get_eps1eps2(&self) -> PyDual64 {
        self._data.eps1eps2.into()
    }
}

macro_rules! impl_hyper_dual_n {
    ($py_type_name:ident, $n:literal) => {
        #[pyclass(name = "HyperDualN64")]
        #[derive(Clone, Copy)]
        pub struct $py_type_name {
            pub _data: HyperDualN64<$n>,
        }

        impl $py_type_name {
            pub fn new(re: f64, v1: [f64; $n], v2: [[f64; $n]; $n]) -> Self {
                Self {
                    _data: HyperDualN64::new(re, StaticVec::new_vec(v1), StaticMat::new(v2)),
                }
            }
        }

        #[pymethods]
        impl $py_type_name {
            #[getter]
            /// Gradient.
            pub fn get_v1(&self) -> [f64; $n] {
                *self._data.v1.raw_array()
            }

            // #[getter]
            // /// Hessian.
            // pub fn get_hessian(&self) -> [[f64; $n]; $n] {
            //     *self._data.v2.raw_data()
            // }
        }

        impl_dual_num!($py_type_name, HyperDualN64<$n>, f64);
    };
}

macro_rules! impl_hyper_dual_mn {
    ($py_type_name:ident, $m:literal, $n:literal) => {
        #[pyclass(name = "HyperDualMN64")]
        #[derive(Clone, Copy)]
        pub struct $py_type_name {
            pub _data: HyperDualMN64<$m, $n>,
        }

        impl $py_type_name {
            pub fn new(
                re: f64,
                eps1: [f64; $m],
                eps2: [f64; $n],
                eps1eps2: [[f64; $n]; $m],
            ) -> Self {
                Self {
                    _data: HyperDualMN64::new(
                        re,
                        StaticVec::new_vec(eps1),
                        StaticVec::new_vec(eps2),
                        StaticMat::new(eps1eps2),
                    ),
                }
            }
        }

        #[pymethods]
        impl $py_type_name {
            #[getter]
            /// First hyperdual part.
            fn get_eps1(&self) -> [f64; $m] {
                *self._data.eps1.raw_array()
            }

            #[getter]
            /// Second hyperdual part.
            fn get_eps2(&self) -> [f64; $n] {
                *self._data.eps2.raw_array()
            }

            // #[getter]
            // /// Hessian.
            // pub fn get_hessian(&self) -> [[f64; $n]; $n] {
            //     *self._data.v2.raw_data()
            // }
        }

        impl_dual_num!($py_type_name, HyperDualMN64<$m, $n>, f64);
    };
}

macro_rules! impl_derive {
    ([$(($py_type_name:ident, $n:literal)),+; $(($py_type_name12:ident, $py_type_name21:ident, $m:literal)),+; $(($py_type_name3:ident, $m1:literal, $m2:literal)),+]) => {
        #[pymethods]
        impl PyHyperDual64 {
            #[staticmethod]
            fn derive(x1: &PyAny, x2: Option<&PyAny>) -> PyResult<PyObject> {
                Python::with_gil(|py| {
                    match x2 {
                        None => {
                            if let Ok(x) = x1.extract::<f64>() {
                                return Ok(PyCell::new(py, PyHD2_64::from(HD2_64::from(x).derive()))?.to_object(py));
                            };
                            if let Ok(x) = x1.extract::<PyDual64>() {
                                return Ok(PyCell::new(py, PyHD2Dual64::from(HD2Dual64::from_re(x._data).derive()))?.to_object(py));
                            };
                            $(
                                if let Ok(x) = x1.extract::<[f64; $n]>() {
                                    let arr = StaticVec::new_vec(x).map(HyperDualN64::from).derive();
                                    let py_vec: Result<Vec<&PyCell<$py_type_name>>, _> = arr.raw_array().iter().map(|&i| PyCell::new(py, $py_type_name::from(i))).collect();
                                    return Ok(py_vec?.to_object(py));
                                };
                            )+
                        },
                        Some(x2) => {
                            if let (Ok(x1), Ok(x2)) = (x1.extract::<f64>(), x2.extract::<f64>()) {
                                let x1 = HyperDual64::from(x1).derive1();
                                let x2 = HyperDual64::from(x2).derive2();
                                let py_x1 = PyCell::new(py, PyHyperDual64::from(x1));
                                let py_x2 = PyCell::new(py, PyHyperDual64::from(x2));
                                return Ok((py_x1?, py_x2?).to_object(py));
                            };
                            $(
                                if let (Ok(x1), Ok(x2)) = (x1.extract::<f64>(), x2.extract::<[f64; $m]>()) {
                                    let x1 = HyperDualMN64::from(x1).derive1();
                                    let arr2 = StaticVec::new_vec(x2).map(HyperDualMN64::from).derive2();
                                    let py_x1 = PyCell::new(py, $py_type_name12::from(x1));
                                    let py_vec2: Result<Vec<&PyCell<$py_type_name12>>, _> = arr2.raw_array().iter().map(|&i| PyCell::new(py, $py_type_name12::from(i))).collect();
                                    return Ok((py_x1?, py_vec2?).to_object(py));
                                };
                            )+
                            $(
                                if let (Ok(x1), Ok(x2)) = (x1.extract::<[f64; $m]>(), x2.extract::<f64>(), ) {
                                    let arr1 = StaticVec::new_vec(x1).map(HyperDualMN64::from).derive2();
                                    let x2 = HyperDualMN64::from(x2).derive1();
                                    let py_vec1: Result<Vec<&PyCell<$py_type_name12>>, _> = arr1.raw_array().iter().map(|&i| PyCell::new(py, $py_type_name12::from(i))).collect();
                                    let py_x2 = PyCell::new(py, $py_type_name12::from(x2));
                                    return Ok((py_vec1?, py_x2?).to_object(py));
                                };
                            )+
                            $(
                                if let (Ok(x1), Ok(x2)) = (x1.extract::<[f64; $m1]>(), x2.extract::<[f64; $m2]>()) {
                                    let arr1 = StaticVec::new_vec(x1).map(HyperDualMN64::from).derive1();
                                    let arr2 = StaticVec::new_vec(x2).map(HyperDualMN64::from).derive2();
                                    let py_vec1: Result<Vec<&PyCell<$py_type_name3>>, _> = arr1.raw_array().iter().map(|&i| PyCell::new(py, $py_type_name3::from(i))).collect();
                                    let py_vec2: Result<Vec<&PyCell<$py_type_name3>>, _> = arr2.raw_array().iter().map(|&i| PyCell::new(py, $py_type_name3::from(i))).collect();
                                    return Ok((py_vec1?, py_vec2?).to_object(py));
                                };
                            )+
                        }
                    };
                    Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
                })
            }
        }
        $(impl_hyper_dual_n!($py_type_name, $n);)+
        $(impl_hyper_dual_mn!($py_type_name12, 1, $m);)+
        $(impl_hyper_dual_mn!($py_type_name21, $m, 1);)+
        $(impl_hyper_dual_mn!($py_type_name3, $m1, $m2);)+
    };
}

impl_derive!([
    (PyHyperDual64_2, 2),
    (PyHyperDual64_3, 3),
    (PyHyperDual64_4, 4),
    (PyHyperDual64_5, 5);
    (PyHyperDual64_1_2, PyHyperDual64_2_1, 2),
    (PyHyperDual64_1_3, PyHyperDual64_3_1, 3),
    (PyHyperDual64_1_4, PyHyperDual64_4_1, 4),
    (PyHyperDual64_1_5, PyHyperDual64_5_1, 5);
    (PyHyperDual64_2_2, 2, 2),
    (PyHyperDual64_2_3, 2, 3),
    (PyHyperDual64_2_4, 2, 4),
    (PyHyperDual64_2_5, 2, 5),
    (PyHyperDual64_3_2, 3, 2),
    (PyHyperDual64_3_3, 3, 3),
    (PyHyperDual64_3_4, 3, 4),
    (PyHyperDual64_3_5, 3, 5),
    (PyHyperDual64_4_2, 4, 2),
    (PyHyperDual64_4_3, 4, 3),
    (PyHyperDual64_4_4, 4, 4),
    (PyHyperDual64_4_5, 4, 5),
    (PyHyperDual64_5_2, 5, 2),
    (PyHyperDual64_5_3, 5, 3),
    (PyHyperDual64_5_4, 5, 4),
    (PyHyperDual64_5_5, 5, 5)
]);
