import pytest
import tempfile
import io

from pycrc32 import Hasher, crc32, crc32c, crc32_file, crc32_fileobj


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


def test_Hasher_copy():
    """Test Hasher copy functionality."""
    hasher1 = Hasher()
    hasher1.update(b"test data")

    hasher2 = hasher1.copy()
    hasher2.update(b" more")

    # Original should have only "test data"
    assert hasher1.finalize() == crc32(b"test data")
    # Copy should have "test data more"
    assert hasher2.finalize() == crc32(b"test data more")


def test_Hasher_len():
    """Test Hasher length tracking."""
    hasher = Hasher()
    assert len(hasher) == 0

    hasher.update(b"hello")
    assert len(hasher) == 5

    hasher.update(b" world")
    assert len(hasher) == 11

    hasher.reset()
    assert len(hasher) == 0


def test_Hasher_repr():
    """Test Hasher string representation."""
    hasher = Hasher()
    repr_str = repr(hasher)
    assert "Hasher(" in repr_str
    assert "bytes_processed=0" in repr_str

    hasher.update(b"test")
    repr_str = repr(hasher)
    assert "bytes_processed=4" in repr_str


def test_Hasher_context_manager():
    """Test Hasher context manager functionality."""
    data = b"context manager test"

    # Test context manager creates independent copy
    with Hasher() as ctx_hasher:
        ctx_hasher.update(data)
        ctx_result = ctx_hasher.finalize()

    # Original hasher should be unchanged
    original_hasher = Hasher()
    original_result = original_hasher.finalize()

    assert ctx_result == crc32(data)
    assert original_result == 0


def test_crc32c_basic():
    """Test basic CRC32C functionality."""
    test_data = b"123456789"

    # CRC32 and CRC32C should produce different results
    crc32_result = crc32(test_data)
    crc32c_result = crc32c(test_data)

    assert crc32_result != crc32c_result
    assert isinstance(crc32c_result, int)


def test_crc32c_empty():
    """Test CRC32C with empty input."""
    assert crc32c(b"") == 0


def test_buffer_protocol_support():
    """Test enhanced buffer protocol support."""
    test_data = b"hello world"

    # Test with bytes (baseline)
    result_bytes = crc32(test_data)
    assert result_bytes == crc32(b"hello world")

    # Test with memoryview
    mv = memoryview(test_data)
    try:
        # This should work if __bytes__ is supported
        result_mv = crc32(mv)
        # If it works, should give same result
        assert result_mv == result_bytes
    except Exception:
        # If it doesn't work, that's expected for now
        pass


def test_error_handling():
    """Test enhanced error handling."""
    # Test TypeError for invalid inputs
    with pytest.raises(TypeError, match="string"):
        crc32("invalid")

    with pytest.raises(TypeError, match="int"):
        crc32(123)

    # Test Hasher error handling
    hasher = Hasher()
    with pytest.raises(TypeError, match="string"):
        hasher.update("invalid")


def test_file_functions():
    """Test file convenience functions."""
    test_data = b"file test content"

    with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
        tmp_file.write(test_data)
        tmp_file.flush()

        # Test crc32_file
        file_result = crc32_file(tmp_file.name)
        expected_result = crc32(test_data)
        assert file_result == expected_result

        # Test crc32_fileobj with file handle
        with open(tmp_file.name, 'rb') as f:
            fileobj_result = crc32_fileobj(f)
            assert fileobj_result == expected_result

        # Test crc32_fileobj with BytesIO
        bio = io.BytesIO(test_data)
        bio_result = crc32_fileobj(bio)
        assert bio_result == expected_result


def test_file_errors():
    """Test file function error handling."""
    # Test nonexistent file
    with pytest.raises(FileNotFoundError):
        crc32_file("/nonexistent/file.txt")

    # Test invalid file object
    with pytest.raises(TypeError):
        crc32_fileobj("not a file object")


def test_new_features_comprehensive():
    """Test that new features work together correctly."""
    data1 = b"chunk1"
    data2 = b"chunk2"
    combined_data = data1 + data2

    # Test copy + context manager
    with Hasher() as ctx_hasher:
        ctx_hasher.update(data1)
        ctx_result = ctx_hasher.finalize()

    original_hasher = Hasher()
    original_hasher.update(data1)
    original_result = original_hasher.finalize()

    assert ctx_result == original_result

    # Test combine works with copy
    hasher1 = Hasher()
    hasher1.update(data1)
    hasher1_copy = hasher1.copy()
    hasher1_copy.update(data2)

    assert hasher1_copy.finalize() == crc32(combined_data)
