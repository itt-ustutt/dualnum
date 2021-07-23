# Hyper-Dual Numbers

[![documentation](https://img.shields.io/badge/docs-github--pages-blue)](https://itt-ustutt.github.io/dualnum/)
[![PyPI version](https://badge.fury.io/py/dualnum.svg)](https://badge.fury.io/py/dualnum)

Python bindings for the `num-dual` rust crate.

## Installation

```
pip install dualnum
```

## Installation from source

To install from source, you need to have the rust compiler installed.

```
pip install git+https://github.com/itt-ustutt/dualnum
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
from dualnum import HyperDual64 as HD64, derive2
import numpy as np

def f(x):
    return np.exp(x) / np.sqrt(np.sin(x)**3 + np.cos(x)**3)

x = derive2(1.5)
result = f(x)
print('f(x)    = {}'.format(result.value))
print('df/dx   = {}'.format(result.first_derivative))
print('d2f/dx2 = {}'.format(result.second_derivative))
```
