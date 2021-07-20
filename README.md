# Hyper-Dual Numbers

[![documentation](https://img.shields.io/badge/docs-github--pages-blue)](https://itt-ustutt.github.io/dualnum/)

Python bindings for the `num-dual` rust crate.

## Installation

Currently, you need to have a rust compiler and python installed.
You also need `virtualenv`.

From project root:

```
virtualenv venv
source venv/bin/activate
pip install maturin
maturin build --release
pip install target/wheels/dualnum.***.whl
```

## Sphinx documentation

To build the documentation with sphinx:

```
maturin develop --release
cd docs
make html
make doctest
firefox _build/html/index.html
```

## Usage

### Compute first and second derivative of a scalar valued function.

```python
from dualnum import HyperDual64 as HD64
import numpy as np

def f(x):
    return np.exp(x) / np.sqrt(np.sin(x)**3 + np.cos(x)**3)

print(f(1.5))

x = HD64(1.5, 1.0, 1.0, 0.0)
result = f(x)
print('f(x)    = {}'.format(result.re))
print('df/dx   = {}'.format(result.eps1))
print('d2f/dx2 = {}'.format(result.eps1eps2))
```
