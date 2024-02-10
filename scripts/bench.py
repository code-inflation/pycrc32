import time
import zlib

import numpy as np
from fastcrc.crc32 import iso_hdlc

from pycrc32 import crc32


def generate_data():
    return np.random.random((2048, 2048)).tobytes()


def time_crc32_calculation(data, method):
    start_time = time.time()
    crc_value = method(data)
    end_time = time.time()
    return crc_value, end_time - start_time


def main():
    data = generate_data()

    # Benchmark pycrc32
    crc_value_pycrc32, time_pycrc32 = time_crc32_calculation(data, crc32)
    print(f"pycrc32: CRC32={crc_value_pycrc32}, Time={time_pycrc32:.6f} seconds")

    # Benchmark fastcrc
    crc_value_fastcrc, time_fastcrc = time_crc32_calculation(data, iso_hdlc)
    print(f"fastcrc: CRC32={crc_value_fastcrc}, Time={time_fastcrc:.6f} seconds")

    # Benchmark zlib
    crc_value_zlib, time_zlib = time_crc32_calculation(data, zlib.crc32)
    print(f"zlib: CRC32={crc_value_zlib}, Time={time_zlib:.6f} seconds")


if __name__ == "__main__":
    main()
