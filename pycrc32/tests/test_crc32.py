import pytest

from pycrc32 import crc32


@pytest.mark.parametrize(
    "input_bytes,expected_crc",
    [
        (b"123456789", 3421780262),
        (b"", 0),
        (b"a", 3904355907),
        (b"The quick brown fox jumps over the lazy dog", 1095738169),
    ],
)
def test_crc32(input_bytes, expected_crc):
    assert crc32(input_bytes) == expected_crc
