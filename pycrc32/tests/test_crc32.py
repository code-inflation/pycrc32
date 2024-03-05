import pytest

from pycrc32 import Hasher, crc32


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


@pytest.mark.parametrize(
    "input_bytes,expected_crc",
    [
        (b"123456789", 3421780262),
        (b"", 0),
        (b"a", 3904355907),
        (b"The quick brown fox jumps over the lazy dog", 1095738169),
    ],
)
def test_Hasher_simple_usage(input_bytes, expected_crc):
    hasher = Hasher()
    hasher.update(input_bytes)
    assert hasher.finalize() == expected_crc


def test_Hasher_with_initial():
    initial_crc = 3421780262  # CRC for "123456789"
    additional_data = b" continuation"
    combined_crc = crc32(b"123456789 continuation")

    hasher = Hasher.with_initial(initial_crc)
    hasher.update(additional_data)
    assert hasher.finalize() == combined_crc


def test_Hasher_reset():
    input_bytes = b"The quick brown fox jumps over the lazy dog"
    hasher = Hasher()
    hasher.update(input_bytes)
    first_crc = hasher.finalize()

    # Reset and reuse the hasher for the same input
    hasher.reset()
    hasher.update(input_bytes)
    second_crc = hasher.finalize()

    assert first_crc == 1095738169
    assert second_crc == 1095738169


@pytest.mark.parametrize(
    "input_bytes1,input_bytes2",
    [
        (b"123456789", b" continuation"),
        (b"The quick brown fox", b" jumps over the lazy dog"),
    ],
)
def test_Hasher_combine(input_bytes1, input_bytes2):
    hasher1 = Hasher()
    hasher1.update(input_bytes1)
    crc1 = hasher1.finalize()

    hasher2 = Hasher()
    hasher2.update(input_bytes2)
    hasher2.finalize()

    combined_crc = crc32(input_bytes1 + input_bytes2)
    hasher_combined = Hasher.with_initial(crc1)
    hasher_combined.update(input_bytes2)

    assert hasher_combined.finalize() == combined_crc
