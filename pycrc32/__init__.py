"""
pycrc32 - Python module for SIMD-accelerated CRC32 checksum computation.

This package provides high-performance CRC32 computation using Rust's crc32fast library
with Python bindings via PyO3.

Basic usage:
    >>> from pycrc32 import crc32
    >>> crc32(b"hello")
    907060870

Advanced usage with incremental hashing:
    >>> from pycrc32 import Hasher
    >>> hasher = Hasher()
    >>> hasher.update(b"first chunk")
    >>> hasher.update(b"second chunk")
    >>> hasher.finalize()
    1234567890
"""

from typing import IO, Union, Any
from .pycrc32 import Hasher, crc32, crc32_file, crc32_fileobj, crc32c_func as crc32c

__all__ = ["Hasher", "crc32", "crc32c", "crc32_file", "crc32_fileobj"]
__version__ = "0.3.0"

# Type aliases for better documentation
FilePath = Union[str, bytes]
FileObject = IO[bytes]
DataInput = Union[bytes, bytearray, memoryview, Any]
