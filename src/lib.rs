use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[macro_use]
mod macros;
mod dual;
mod hd2;
mod hd3;
mod hyperdual;

use dual::__pyo3_get_function_derive1;
use hd3::__pyo3_get_function_derive3;
use hyperdual::__pyo3_get_function_derive2;

pub use dual::PyDual64;
pub use hd2::{PyHD2Dual64, PyHD2_64};
pub use hd3::{PyHD3Dual64, PyHD3_64};
pub use hyperdual::{PyHyperDual64, PyHyperDualDual64};

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
    m.add_class::<PyHD2_64>()?;
    m.add_class::<PyHD3_64>()?;
    m.add_class::<PyHyperDualDual64>()?;
    m.add_class::<PyHD2Dual64>()?;
    m.add_class::<PyHD3Dual64>()?;
    m.add_function(wrap_pyfunction!(derive1, m)?).unwrap();
    m.add_function(wrap_pyfunction!(derive2, m)?).unwrap();
    m.add_function(wrap_pyfunction!(derive3, m)?).unwrap();
    Ok(())
}
