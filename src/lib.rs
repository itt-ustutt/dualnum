use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[macro_use]
mod macros;
mod dual;
mod dual2;
mod dual3;
mod hyperdual;

use dual::__pyo3_get_function_derive1;
use dual3::__pyo3_get_function_derive3;
use hyperdual::__pyo3_get_function_derive2;

pub use dual::PyDual64;
pub use dual2::{PyDual2Dual64, PyDual2_64};
pub use dual3::{PyDual3Dual64, PyDual3_64};
pub use hyperdual::{PyHyperDual64, PyHyperDualDual64};

/// Hyperdual numbers
/// =================
///
/// Using hyperdual numbers, you can compute exact derivatives of functions without writing analytical derivatives
/// or using numeric differentiation.
///
/// Examples
/// --------
///
/// First, second and third derivatives
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///
/// Let's define a simple, scalar valued function for which we want to compute the first, second and third derivative.
///
/// >>> def f(x):
/// >>> """f'(x) = 3 * x**2, f''(x) = 6 * x"""
/// >>>     return x**3
///
/// The function is defined just like a regular python function.
/// Different from a regular python function though, we use (hyper)dual numbers as arguments.
/// For example, to compute the first derivative at x = 2, we need to call the function with a dual number as input, setting the dual part (ε)
/// to 1.0.
///
/// >>> from hyperdual import Dual64
/// >>> x = Dual64(2.0, 1.0)
/// >>> x
/// 2 + 1ε
///
/// Then, calling the function, the result is also a dual number, where the real part (or value)
/// is the result of the function that we would get by simply calling it with a floating point number,
/// whereas the dual part (or first derivative) contains the derivative.
///
/// >>> result = f(x)
/// >>> result
/// 8 + 12ε
/// >>> result.value
/// 8
/// >>> result.first_derivative
/// 12
///
/// The value we used for the dual part (1.0) is not important, however,
/// the resulting derivatives will be multiples of the chosen value and as such we set it to unity.
///
/// The procedure as outlined above works fine, but you have to know what type of dual number you have to use.
/// E.g. for the second derivative, the function argument has to be a hyerdual number. We therefore introduce
/// helper functions that can be used to simply declare the order of the derivative you want to compute.
///
/// The same result as above can be created via
///
/// >>> x = derive1(2.0) # we want the first derivative
/// >>> result = f(x)
/// >>> result.first_derivative
/// 12
///
/// where `x = derive1(2.0)` constructs the correct dual number for us.
///
/// Let's compute the second and third derivatives!
///
/// >>> from hyperdual import derive3
/// >>> x = derive3(2.0)
/// >>> result = f(x)
/// >>> print(f"f(x)   = {result}\nf'(x)  = {result.first_derivative}\nf''(x) = {result.second_derivative}")
/// f(x)   = 8 + 12v1 + 12v2 + 6v3
/// f'(x)  = 12.0
/// f''(x) = 12.0
///
/// Partial derivatives
/// ^^^^^^^^^^^^^^^^^^^
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
    m.add_class::<PyDual2_64>()?;
    m.add_class::<PyDual3_64>()?;
    m.add_class::<PyHyperDualDual64>()?;
    m.add_class::<PyDual2Dual64>()?;
    m.add_class::<PyDual3Dual64>()?;
    m.add_function(wrap_pyfunction!(derive1, m)?).unwrap();
    m.add_function(wrap_pyfunction!(derive2, m)?).unwrap();
    m.add_function(wrap_pyfunction!(derive3, m)?).unwrap();
    Ok(())
}
