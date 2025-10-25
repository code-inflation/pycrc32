"""
Comprehensive example demonstrating all pycrc32 features.

This example shows:
- Basic CRC32 computation
- CRC32C variant (Castagnoli polynomial)
- Incremental hashing with Hasher
- Advanced Hasher features (copy, context manager, etc.)
- File processing
- Error handling
"""

from pycrc32 import crc32, crc32c, Hasher, crc32_file, crc32_fileobj
import tempfile
import io


def basic_usage():
    """Demonstrate basic CRC32 and CRC32C usage."""
    print("=== Basic Usage ===")

    data = b"123456789"

    # Standard CRC32 (IEEE 802.3 polynomial)
    crc32_result = crc32(data)
    print(f"Standard CRC32 for {data!r}: {crc32_result:#x} ({crc32_result})")

    # CRC32C (Castagnoli polynomial, used in iSCSI, Ethernet, etc.)
    crc32c_result = crc32c(data)
    print(f"CRC32C for {data!r}: {crc32c_result:#x} ({crc32c_result})")

    # They should be different
    print(f"Different algorithms: {crc32_result != crc32c_result}")
    print()


def incremental_hashing():
    """Demonstrate incremental hashing features."""
    print("=== Incremental Hashing ===")

    data_chunks = [b"chunk1", b"chunk2", b"chunk3"]

    # Basic incremental hashing
    hasher = Hasher()
    for chunk in data_chunks:
        hasher.update(chunk)
        print(f"Added {chunk!r}, bytes processed: {len(hasher)}")

    final_result = hasher.finalize()
    print(f"Final CRC32: {final_result:#x}")
    print(f"Hasher state: {repr(hasher)}")
    print()

    # Using copy method
    hasher1 = Hasher()
    hasher1.update(b"first")
    hasher2 = hasher1.copy()  # Independent copy
    hasher2.update(b"second")

    print(f"Original: {hasher1.finalize():#x}")  # Only "first"
    print(f"Copy: {hasher2.finalize():#x}")      # "first" + "second"
    print()

    # Context manager usage
    with Hasher() as ctx_hasher:
        ctx_hasher.update(b"context data")
        ctx_result = ctx_hasher.finalize()

    print(f"Context manager result: {ctx_result:#x}")
    print()


def advanced_hasher_features():
    """Demonstrate advanced Hasher features."""
    print("=== Advanced Hasher Features ===")

    # Hasher with initial state
    initial_value = 0x12345678
    hasher = Hasher.with_initial(initial_value)
    hasher.update(b"additional data")
    result = hasher.finalize()
    print(f"Hasher with initial value {initial_value:#x}: {result:#x}")

    # Reset functionality
    hasher.reset()
    print(f"After reset - bytes processed: {len(hasher)}")
    print(f"After reset - CRC32: {hasher.finalize():#x}")

    # Combine functionality
    hasher1 = Hasher()
    hasher1.update(b"part1")
    hasher2 = Hasher()
    hasher2.update(b"part2")

    hasher1.combine(hasher2)
    combined_result = hasher1.finalize()
    direct_result = crc32(b"part1part2")

    print(f"Combined result: {combined_result:#x}")
    print(f"Direct computation: {direct_result:#x}")
    print(f"Combine works correctly: {combined_result == direct_result}")
    print()


def file_processing():
    """Demonstrate file processing capabilities."""
    print("=== File Processing ===")

    test_data = b"This is test data for file processing."

    # Create a temporary file
    with tempfile.NamedTemporaryFile(delete=False) as tmp_file:
        tmp_file.write(test_data)
        tmp_file.flush()

        # Process file by path
        file_result = crc32_file(tmp_file.name)
        print(f"CRC32 of file '{tmp_file.name}': {file_result:#x}")

        # Process file as file object
        with open(tmp_file.name, 'rb') as f:
            fileobj_result = crc32_fileobj(f)
            print(f"CRC32 of file object: {fileobj_result:#x}")

        # Verify consistency
        direct_result = crc32(test_data)
        print(f"Direct computation: {direct_result:#x}")
        print(f"All methods match: {file_result == fileobj_result == direct_result}")

    # Process in-memory file-like object
    bio = io.BytesIO(test_data)
    bio_result = crc32_fileobj(bio)
    print(f"CRC32 of BytesIO: {bio_result:#x}")
    print()


def error_handling():
    """Demonstrate error handling."""
    print("=== Error Handling ===")

    # These operations will raise informative exceptions
    try:
        crc32("invalid string input")
    except TypeError as e:
        print(f"Expected error for string input: {e}")

    try:
        crc32(12345)
    except TypeError as e:
        print(f"Expected error for int input: {e}")

    try:
        crc32_file("/nonexistent/file.txt")
    except FileNotFoundError as e:
        print(f"Expected error for nonexistent file: {e}")

    try:
        crc32_fileobj("not a file object")
    except TypeError as e:
        print(f"Expected error for invalid file object: {e}")
    print()


def performance_comparison():
    """Simple performance comparison."""
    import time

    print("=== Simple Performance Test ===")

    data = b"performance test data" * 1000  # ~23KB
    iterations = 1000

    # Time standard CRC32
    start_time = time.time()
    for _ in range(iterations):
        crc32(data)
    crc32_time = time.time() - start_time

    # Time CRC32C
    start_time = time.time()
    for _ in range(iterations):
        crc32c(data)
    crc32c_time = time.time() - start_time

    print(f"Processed {len(data) * iterations} bytes")
    print(f"CRC32 time: {crc32_time:.4f}s")
    print(f"CRC32C time: {crc32c_time:.4f}s")
    print(f"CRC32C overhead: {(crc32c_time / crc32_time - 1) * 100:.1f}%")
    print()


if __name__ == "__main__":
    basic_usage()
    incremental_hashing()
    advanced_hasher_features()
    file_processing()
    error_handling()
    performance_comparison()

    print("All pycrc32 features demonstrated successfully!")
