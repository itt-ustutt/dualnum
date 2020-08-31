use num_hyperdual::DualNumMethods;
use num_hyperdual::{Dual64 as D64, HyperDual64 as HD64, HD3_64};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::PyNumberProtocol;

#[pyclass(name=Dual64)]
#[derive(Clone)]
/// Dual number using 64-bit-floats.
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
    _data: D64,
}

impl From<D64> for PyDual64 {
    fn from(d: D64) -> Self {
        Self { _data: d }
    }
}

#[pymethods]
impl PyDual64 {
    #[new]
    pub fn new(re: f64, eps: f64) -> Self {
        Self {
            _data: D64::new(re, eps),
        }
    }

    #[getter]
    /// Dual part.
    pub fn get_eps(&self) -> f64 {
        self._data.eps
    }
}

#[pyclass(name=HyperDual64)]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats.
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
    _data: HD64,
}

impl From<HD64> for PyHyperDual64 {
    fn from(hd: HD64) -> Self {
        Self { _data: hd }
    }
}

#[pymethods]
impl PyHyperDual64 {
    #[new]
    pub fn new(re: f64, eps1: f64, eps2: f64, eps1eps2: f64) -> Self {
        Self {
            _data: HD64::new(re, eps1, eps2, eps1eps2),
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

#[pyclass(name=HD3_64)]
#[derive(Clone)]
/// Hyper dual number using 64-bit-floats.
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
pub struct PyHD_3_64 {
    _data: HD3_64,
}

impl From<HD3_64> for PyHD_3_64 {
    fn from(hd: HD3_64) -> Self {
        Self { _data: hd }
    }
}

#[pymethods]
impl PyHD_3_64 {
    #[new]
    pub fn new(re: f64) -> Self {
        HD3_64::new(re).into()
    }
}

macro_rules! impl_dual_num {
    ($type_name:ty, $data_type_name:ty) => {
        #[pymethods]
        impl $type_name {
            #[staticmethod]
            /// (Hyper) dual number from real part, setting all other parts to zero.
            pub fn from_re(re: f64) -> Self {
                Self::from(<$data_type_name>::from(re))
            }

            #[getter]
            /// Real part.
            fn get_re(&self) -> f64 {
                self._data.re()
            }

            #[inline]
            /// Reciprocal value of self.
            pub fn recip(&self) -> Self {
                Self {
                    _data: self._data.recip(),
                }
            }

            #[inline]
            /// Power using 32-bit integer as exponent.
            pub fn powi(&self, n: i32) -> Self {
                Self::from(self._data.powi(n))
            }

            #[inline]
            /// Power using 64-bin float as exponent.
            pub fn powf(&self, n: f64) -> Self {
                Self {
                    _data: self._data.powf(n),
                }
            }

            #[inline]
            /// Power using self (hyper) dual number as exponent.
            pub fn powd(&self, n: Self) -> Self {
                Self {
                    _data: self._data.powd(&n._data),
                }
            }

            #[inline]
            /// Sqaure root.
            pub fn sqrt(&self) -> Self {
                Self {
                    _data: self._data.sqrt(),
                }
            }

            #[inline]
            /// Cubic root.
            pub fn cbrt(&self) -> Self {
                Self {
                    _data: self._data.cbrt(),
                }
            }

            #[inline]
            /// Calculate the exponential of (hyper) dual number.
            pub fn exp(&self) -> Self {
                Self {
                    _data: self._data.exp(),
                }
            }

            #[inline]
            /// Calculate 2**x of (hyper) dual number x.
            pub fn exp2(&self) -> Self {
                Self {
                    _data: self._data.exp2(),
                }
            }

            #[inline]
            /// Calculate exp(x) - 1.
            pub fn expm1(&self) -> Self {
                Self {
                    _data: self._data.exp_m1(),
                }
            }

            #[inline]
            /// Calculate natural logarithm.
            pub fn log(&self) -> Self {
                Self {
                    _data: self._data.ln(),
                }
            }

            #[inline]
            /// Calculate logarithm with given base.
            pub fn log_base(&self, base: f64) -> Self {
                Self {
                    _data: self._data.log(base),
                }
            }

            #[inline]
            /// Calculate logarithm with base 2.
            pub fn log2(&self) -> Self {
                Self {
                    _data: self._data.log2(),
                }
            }

            #[inline]
            /// Calculate logarithm with base 10.
            pub fn log10(&self) -> Self {
                Self {
                    _data: self._data.log10(),
                }
            }

            #[inline]
            /// Returns ln(1+n) (natural logarithm) more accurately than if the operations were performed separately.
            pub fn log1p(&self) -> Self {
                Self {
                    _data: self._data.ln_1p(),
                }
            }

            #[inline]
            /// Hyperbolic sine function.
            pub fn sin(&self) -> Self {
                Self {
                    _data: self._data.sin(),
                }
            }

            #[inline]
            /// Hyperbolic cosine function.
            pub fn cos(&self) -> Self {
                Self {
                    _data: self._data.cos(),
                }
            }

            #[inline]
            /// Computes the tangent of a (hyper) dual number (in radians).
            pub fn tan(&self) -> Self {
                Self {
                    _data: self._data.tan(),
                }
            }

            #[inline]
            /// Simultaneously computes the sine and cosine of the (hyper) dual number, x.
            pub fn sin_cos(&self) -> (Self, Self) {
                let (a, b) = self._data.sin_cos();
                (Self::from(a), Self::from(b))
            }

            #[inline]
            /// Computes the arcsine of a (hyper) dual number.
            pub fn arcsin(&self) -> Self {
                Self {
                    _data: self._data.asin(),
                }
            }

            #[inline]
            /// Computes the arccosine of a (hyper) dual number.
            pub fn arccos(&self) -> Self {
                Self {
                    _data: self._data.acos(),
                }
            }

            #[inline]
            /// Computes the arctangent of a (hyper) dual number.
            pub fn arctan(&self) -> Self {
                Self {
                    _data: self._data.atan(),
                }
            }

            #[inline]
            /// Computes the hyperbolic sine of a (hyper) dual number.
            pub fn sinh(&self) -> Self {
                Self {
                    _data: self._data.sinh(),
                }
            }

            #[inline]
            /// Computes the hyperbolic cosine of a (hyper) dual number.
            pub fn cosh(&self) -> Self {
                Self {
                    _data: self._data.cosh(),
                }
            }

            #[inline]
            /// Computes the hyperbolic tangent of a (hyper) dual number.
            pub fn tanh(&self) -> Self {
                Self {
                    _data: self._data.tanh(),
                }
            }

            #[inline]
            /// Computes the inverse hyperbolic sine of a (hyper) dual number.
            pub fn arcsinh(&self) -> Self {
                Self {
                    _data: self._data.asinh(),
                }
            }

            #[inline]
            /// Computes the inverse hyperbolic cosine of a (hyper) dual number.
            pub fn arccosh(&self) -> Self {
                Self {
                    _data: self._data.acosh(),
                }
            }

            #[inline]
            /// Computes the inverse hyperbolic tangent of a (hyper) dual number.
            pub fn arctanh(&self) -> Self {
                Self {
                    _data: self._data.atanh(),
                }
            }

            #[inline]
            /// Computes the first spherical bessel function.
            pub fn sph_j0(&self) -> Self {
                Self {
                    _data: self._data.sph_j0(),
                }
            }
            #[inline]
            /// Computes the second spherical bessel function.
            pub fn sph_j1(&self) -> Self {
                Self {
                    _data: self._data.sph_j1(),
                }
            }

            #[inline]
            /// Computes the third spherical bessel function.
            pub fn sph_j2(&self) -> Self {
                Self {
                    _data: self._data.sph_j2(),
                }
            }

            #[inline]
            #[text_signature = "($self, b: Self, c: Self)"]
            /// Fused multiply-add. Computes (self * a) + b with only one rounding error.
            fn mul_add(&self, a: Self, b: Self) -> Self {
                Self::from(self._data.mul_add(a._data, b._data))
            }
        }

        #[pyproto]
        impl PyNumberProtocol for $type_name {
            fn __add__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<Self> {
                if let Ok(r) = rhs.extract::<f64>() {
                    return Ok(Self {
                        _data: lhs._data + r,
                    });
                };
                if let Ok(r) = rhs.extract::<Self>() {
                    return Ok(Self {
                        _data: lhs._data + r._data,
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __radd__(&self, other: &PyAny) -> PyResult<Self> {
                if let Ok(o) = other.extract::<f64>() {
                    return Ok(Self {
                        _data: self._data + o,
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __sub__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<Self> {
                if let Ok(r) = rhs.extract::<f64>() {
                    return Ok(Self {
                        _data: lhs._data - r,
                    });
                };
                if let Ok(r) = rhs.extract::<Self>() {
                    return Ok(Self {
                        _data: lhs._data - r._data,
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __rsub__(&self, other: &PyAny) -> PyResult<Self> {
                if let Ok(o) = other.extract::<f64>() {
                    return Ok(Self {
                        _data: -self._data + o,
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __mul__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<Self> {
                if let Ok(r) = rhs.extract::<f64>() {
                    return Ok(Self {
                        _data: lhs._data * r,
                    });
                };
                if let Ok(r) = rhs.extract::<Self>() {
                    return Ok(Self {
                        _data: lhs._data * r._data,
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __rmul__(&self, other: &PyAny) -> PyResult<Self> {
                if let Ok(o) = other.extract::<f64>() {
                    return Ok(Self {
                        _data: self._data * o,
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __truediv__(lhs: PyRef<'p, Self>, rhs: &PyAny) -> PyResult<Self> {
                if let Ok(r) = rhs.extract::<f64>() {
                    return Ok(Self {
                        _data: lhs._data / r,
                    });
                };
                if let Ok(r) = rhs.extract::<Self>() {
                    return Ok(Self {
                        _data: lhs._data / r._data,
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __rtruediv__(&self, other: &PyAny) -> PyResult<Self> {
                if let Ok(o) = other.extract::<f64>() {
                    return Ok(Self {
                        _data: self._data.recip() * o,
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }

            fn __pow__(lhs: &PyAny, rhs: &PyAny, _mod: Option<u32>) -> PyResult<Self> {
                if let (Ok(l), Ok(r)) = (lhs.extract::<Self>(), rhs.extract::<f64>()) {
                    return Ok(Self {
                        _data: l._data.powf(r),
                    });
                };
                if let (Ok(l), Ok(r)) = (lhs.extract::<Self>(), rhs.extract::<i32>()) {
                    return Ok(Self {
                        _data: l._data.powi(r),
                    });
                };
                if let (Ok(l), Ok(r)) = (lhs.extract::<Self>(), rhs.extract::<Self>()) {
                    return Ok(Self {
                        _data: l._data.powd(&r._data),
                    });
                };
                Err(PyErr::new::<PyTypeError, _>(format!("not implemented!")))
            }
        }

        #[pyproto]
        impl pyo3::class::basic::PyObjectProtocol for $type_name {
            fn __repr__(&self) -> PyResult<String> {
                Ok(self._data.to_string())
            }
        }
    };
}

impl_dual_num!(PyDual64, D64);
impl_dual_num!(PyHyperDual64, HD64);
impl_dual_num!(PyHD_3_64, HD3_64);

/// Hyperdual numbers.
/// ==================
///
/// Hyper dual numbers enable computation of function derivatives
/// without the need for numerical derivatives.
///
/// Examples
/// --------
///
/// First derivative using dual numbers.
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///
/// >>> from hyperdual import Dual64
/// >>> import numpy as np
/// >>> x = Dual64(2.0, 1.0) # using a dual part of 1 for derivative
/// >>> np.cos(2.0) == np.sin(x).eps
/// True
///
/// Partial derivatives using hyper dual numbers.
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///
/// >>> from hyperdual import HyperDual64 as HD64
/// >>> from scipy.optimize import rosen, rosen_der
/// >>> rosen([HD64(0.5, 1.0, 0.0, 0.0), HD64(1.0, 0.0, 1.0, 0.0)])
/// 56.5 + -151ε1 + 150ε2 + -200ε1ε2
/// >>> rosen([0.5, 1.0])
/// 56.5
/// >>> rosen_der([0.5, 1.0])
/// array([-151.,  150.])
///
/// Partial derivatives using hyper dual numbers (2).
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// We can use hyper dual numbers together with multiple functions
/// as defined in `numpy`.
///
/// >>> def f(x):
/// ...     return np.exp(x) / np.sqrt(np.sin(x)**3 + np.cos(x)**3)
/// >>> f(1.5)
/// 4.497780053946161
///
/// Calling the same function with a hyper dual number and dual parts of 1
/// yields the first and second derivatives. (ε1 and ε2 parts are identical)
///
/// >>> from hyperdual import HyperDual64 as HD64
/// >>> x = HD64(1.5, 1.0, 1.0, 0.0)
/// >>> f(x)
/// 4.497780053946162 + 4.053427893898621ε1 + 4.053427893898621ε2 + 9.463073681596605ε1ε2
#[pymodule]
fn hyperdual(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDual64>()?;
    m.add_class::<PyHyperDual64>()?;
    m.add_class::<PyHD_3_64>()?;
    Ok(())
}
