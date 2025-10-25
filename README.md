# pycrc32
![PyPI - Version](https://img.shields.io/pypi/v/pycrc32)
![Python Version](https://img.shields.io/badge/python-3.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12%20%7C%203.13-blue)
![PyPI - License](https://img.shields.io/pypi/l/pycrc32)
![PyPI - Downloads](https://img.shields.io/pypi/dm/pycrc32)


`pycrc32` is a Python module for SIMD-accelerated CRC32 checksum computation.  
Big thanks to [rust-crc32fast](https://github.com/srijs/rust-crc32fast) - this project just provides Python bindings for their Rust implementation.

## Installation
```sh
pip install pycrc32
```

## Usage

### Basic CRC32 and CRC32C
```python
from pycrc32 import crc32, crc32c

data = b"123456789"

# Standard CRC32 (IEEE 802.3 polynomial)
print(f"CRC32 for {data!r}: {crc32(data):#x}")

# CRC32C (Castagnoli polynomial, used in iSCSI, Ethernet, etc.)
print(f"CRC32C for {data!r}: {crc32c(data):#x}")
```

### Incremental Hashing with `Hasher` Class

The `Hasher` class provides incremental hashing capabilities for processing large data in chunks:

#### Basic Incremental Hashing
```python
from pycrc32 import Hasher

# Create a new Hasher instance
hasher = Hasher()

# Update with data chunks
hasher.update(b"123456")
hasher.update(b"789")

# Get the final checksum
checksum = hasher.finalize()
print(f"Checksum: {checksum:#x}")
print(f"Bytes processed: {len(hasher)}")
print(f"Hasher state: {repr(hasher)}")
```

#### Advanced Hasher Features
```python
# Initialize with custom initial state
hasher = Hasher.with_initial(0x12345678)
hasher.update(b"data")
result = hasher.finalize()

# Create independent copies
hasher1 = Hasher()
hasher1.update(b"common")
hasher2 = hasher1.copy()  # Independent copy
hasher2.update(b"additional")

print(f"Original: {hasher1.finalize():#x}")  # Only "common"
print(f"Copy: {hasher2.finalize():#x}")      # "common" + "additional"

# Context manager usage
with Hasher() as ctx_hasher:
    ctx_hasher.update(b"context data")
    result = ctx_hasher.finalize()

# Reset functionality
hasher.reset()
print(f"After reset: {hasher.finalize():#x}")

# Combine states (for parallel processing)
hasher1 = Hasher()
hasher1.update(b"part1")
hasher2 = Hasher()
hasher2.update(b"part2")
hasher1.combine(hasher2)
combined = hasher1.finalize()
```

### File Processing
```python
from pycrc32 import crc32_file, crc32_fileobj

# Process files by path
file_crc = crc32_file("/path/to/file.txt")
print(f"File CRC32: {file_crc:#x}")

# Process file objects
with open("/path/to/file.txt", "rb") as f:
    fileobj_crc = crc32_fileobj(f)
    print(f"File object CRC32: {fileobj_crc:#x}")

# Works with any file-like object (BytesIO, etc.)
import io
data = b"file-like data"
bio = io.BytesIO(data)
bio_crc = crc32_fileobj(bio)
print(f"BytesIO CRC32: {bio_crc:#x}")
```

### Enhanced Error Handling
```python
from pycrc32 import crc32, Hasher

# Provides helpful error messages
try:
    crc32("invalid string")
except TypeError as e:
    print(f"Clear error message: {e}")
    # Output: crc32() expects bytes-like object, got string. Use b'your string' or your_string.encode() instead.

try:
    crc32_file("/nonexistent/file.txt")
except FileNotFoundError as e:
    print(f"File error: {e}")
    # Output: File not found: /nonexistent/file.txt
```

### Type Safety and IDE Support
```python
# Full type hints available
from pycrc32 import crc32, crc32c, Hasher, crc32_file, crc32_fileobj
from typing import Union

def process_data(data: Union[bytes, bytearray]) -> int:
    """Function with full type hints."""
    return crc32(data)

# IDE autocompletion and inline documentation
hasher: Hasher = Hasher()  # Type annotation
hasher.update(b"data")    # IDE shows method signatures
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
