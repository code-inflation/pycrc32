# pycrc32
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

