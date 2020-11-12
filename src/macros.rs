#[macro_use]
macro_rules! impl_dual_num {
    ($py_type_name:ty, $data_type:ty, $field_type:ty) => {
        #[pymethods]
        impl $py_type_name {
            #[staticmethod]
            /// (Hyper) dual number from real part, setting all other parts to zero.
            pub fn from_re(re: $field_type) -> Self {
                Self::from(<$data_type>::from_re(re.into()))
            }

            #[getter]
            /// Real part.
            fn get_re(&self) -> f64 {
                f64::from(self._data.re())
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
        impl PyNumberProtocol for $py_type_name {
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
                if let (Ok(l), Ok(r)) = (lhs.extract::<Self>(), rhs.extract::<i32>()) {
                    return Ok(Self {
                        _data: l._data.powi(r),
                    });
                };
                if let (Ok(l), Ok(r)) = (lhs.extract::<Self>(), rhs.extract::<f64>()) {
                    return Ok(Self {
                        _data: l._data.powf(r),
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
        impl pyo3::class::basic::PyObjectProtocol for $py_type_name {
            fn __repr__(&self) -> PyResult<String> {
                Ok(self._data.to_string())
            }
        }
    };
}
