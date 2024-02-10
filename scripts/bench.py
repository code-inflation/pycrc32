import time
import zlib

import numpy as np
from fastcrc.crc32 import iso_hdlc

from pycrc32 import crc32


def time_crc32_calculation(data, method, iterations):
    times = []
    for _ in range(iterations):
        start_time = time.time()
        method(data)
        end_time = time.time()
        times.append(end_time - start_time)
    return np.median(times)


def main():
    iterations = 100
    print(f"Running benchmark with {iterations} iterations for each library.")

    data = np.random.random((2048, 2048)).tobytes()

    results = [
        ("pycrc32", time_crc32_calculation(data, crc32, iterations)),
        ("zlib", time_crc32_calculation(data, zlib.crc32, iterations)),
        ("fastcrc", time_crc32_calculation(data, iso_hdlc, iterations)),
    ]

    print(f"{'Library':<10} | {'Median Time (s)':>15}")
    print("-" * 28)
    for lib, result_time in results:
        print(f"{lib:<10} | {result_time:15.6f}")


if __name__ == "__main__":
    main()
