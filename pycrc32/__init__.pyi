"""
Type stubs for pycrc32 package.

This file provides type hints for the pycrc32 Python module.
"""

from typing import IO, AnyStr, Union
from typing_extensions import Any

class Hasher:
    """Represents an in-progress CRC32 computation."""

    def __init__(self) -> None: ...

    @staticmethod
    def with_initial(init: int) -> "Hasher": ...

    def update(self, data: AnyStr) -> None: ...

    def finalize(self) -> int: ...

    def reset(self) -> None: ...

    def combine(self, other: "Hasher") -> None: ...

    def copy(self) -> "Hasher": ...

    def __len__(self) -> int: ...

    def __repr__(self) -> str: ...

    def __enter__(self) -> "Hasher": ...

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None: ...

def crc32(data: AnyStr) -> int:
    """Calculate CRC32 checksum of bytes-like object."""
    ...

def crc32_file(path: Union[str, bytes]) -> int:
    """Calculate CRC32 checksum of file by path."""
    ...

def crc32_fileobj(fileobj: IO[bytes]) -> int:
    """Calculate CRC32 checksum of file-like object."""
    ...

def crc32c(data: AnyStr) -> int:
    """Calculate CRC32C checksum (Castagnoli polynomial) of bytes-like object."""
    ...

__all__ = ["Hasher", "crc32", "crc32c", "crc32_file", "crc32_fileobj"]