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

### Advanced Checksum Calculation with `Hasher`
For scenarios that require more flexibility, such as processing large amounts of data or computing the checksum in stages, you can use the `Hasher` class:
```python
from pycrc32 import Hasher

# Create a new Hasher instance
hasher = Hasher()

# Update the hasher with data chunks
hasher.update(b"123456")
hasher.update(b"789")

# Finalize the computation and get the checksum
checksum = hasher.finalize()
print(f"Checksum: {checksum}")

# Reset the hasher to compute another checksum
hasher.reset()
hasher.update(b"The quick brown fox jumps over the lazy dog")
new_checksum = hasher.finalize()
print(f"New checksum: {new_checksum}")
```

You can also initialize a `Hasher` with a specific initial CRC32 state:
```python
initial_crc = 12345678
hasher = Hasher.with_initial(initial_crc)

hasher.update(b"additional data")
final_checksum = hasher.finalize()
print(f"Final checksum with initial state: {final_checksum}")
```

To combine checksums from different data blocks without needing to concatenate the data, use the `combine` method:
```python
hasher1 = Hasher()
hasher1.update(b"Data block 1")
checksum1 = hasher1.finalize()

hasher2 = Hasher()
hasher2.update(b"Data block 2")
checksum2 = hasher2.finalize()

# Combine checksums from hasher1 into hasher2
hasher1.combine(hasher2)  # Combine the state of hasher2 into hasher1

# The final checksum after combination
combined_checksum = hasher1.finalize()
print(f"Combined checksum: {combined_checksum}")
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
