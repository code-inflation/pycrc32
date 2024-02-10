from pycrc32 import crc32

data = b"123456789"
print(f"crc32 for {data!r} is {crc32(data)}")
