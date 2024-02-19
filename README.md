# pycrc32
![PyPI - Version](https://img.shields.io/pypi/v/pycrc32)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/pycrc32)
![PyPI - License](https://img.shields.io/pypi/l/pycrc32)
![PyPI - Downloads](https://img.shields.io/pypi/dm/pycrc32)


`pycrc32` is a Python module for SIMD-accelerated CRC32 checksum computation.  
Big thanks to [rust-crc32fast](https://github.com/srijs/rust-crc32fast) - this project just provides Python bindings for their Rust implementation.

## Installation
```sh
pip install pycrc32
```

## Usage
```python
from pycrc32 import crc32

data = b"123456789"
print(f"crc32 for {data!r} is {crc32(data)}")
```

## Speed
The performance of `pycrc32` has been benchmarked on a trusty old Intel i7-8550U using 32MB of random input data. Below is a comparison of the median computation times across different libraries:

| Library   | Median Time (s) |
|-----------|-----------------|
| `pycrc32` | 0.002703        |
| `zlib`    | 0.019796        |
| `fastcrc` | 0.071426        |

We reach almost 10x performance improvements compared to the `zlib` baseline implementation.  
See `scripts/bench.py` for more details.
