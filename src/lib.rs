use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::OnceLock;

use crc32fast::hash;
use crc32fast::Hasher as RustHasher;

// CRC32C implementation (Castagnoli polynomial)
const CRC32C_POLY: u32 = 0x1EDC6F41;

/// Build CRC32C lookup table (forward processing)
fn crc32c_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0..256 {
        let mut crc = (i as u32) << 24;  // Move to MSB for forward processing
        for _ in 0..8 {
            if crc & 0x80000000 != 0 {  // Check MSB
                crc = (crc << 1) ^ CRC32C_POLY;
            } else {
                crc <<= 1;
            }
        }
        table[i] = crc;
    }
    table
}

/// Get CRC32C lookup table (lazy initialization)
fn get_crc32c_table() -> &'static [u32; 256] {
    static TABLE: OnceLock<[u32; 256]> = OnceLock::new();
    TABLE.get_or_init(crc32c_table)
}

/// Calculate CRC32C using the Castagnoli polynomial (forward processing)
fn crc32c_update(mut crc: u32, data: &[u8]) -> u32 {
    let table = get_crc32c_table();
    crc ^= 0xFFFFFFFF;  // Initial XOR
    for &byte in data {
        let index = ((crc >> 24) ^ byte as u32) & 0xFF;  // Use MSB
        crc = (crc << 8) ^ table[index as usize];
    }
    crc ^ 0xFFFFFFFF  // Final XOR
}

/// Calculate CRC32C of data (wrapper for consistency with crc32fast)
fn crc32c(data: &[u8]) -> u32 {
    crc32c_update(0, data)
}

/// CRC32C Hasher for incremental computation
struct Crc32CHasher {
    crc: u32,
}

impl Crc32CHasher {
    fn new() -> Self {
        Self { crc: 0 }
    }

    fn new_with_initial(init: u32) -> Self {
        Self { crc: init }
    }

    fn update(&mut self, data: &[u8]) {
        self.crc = crc32c_update(self.crc, data);
    }

    fn finalize(&self) -> u32 {
        self.crc
    }

    fn reset(&mut self) {
        self.crc = 0;
    }

    fn clone(&self) -> Self {
        Self { crc: self.crc }
    }
}

/// Calculates the CRC32 hash using the crc32fast crate.
///
/// # Arguments
/// * `bytes` - Any bytes-like object (bytes, memoryview, etc.) that can be converted to bytes
///
/// # Returns
/// CRC32 checksum as a 32-bit unsigned integer
///
/// # Examples
/// ```python
/// import pycrc32
/// result = pycrc32.crc32(b"hello")
/// # result = 907060870
/// ```
///
/// # Type Support
/// - `bytes` - Native bytes (no conversion overhead)
/// - Objects with `__bytes__()` method (converted to bytes)
/// - Returns `TypeError` for unsupported types with helpful error messages
#[pyfunction]
fn crc32(bytes: &Bound<'_, PyAny>) -> PyResult<u32> {
    // Try to extract as bytes first
    if let Ok(py_bytes) = bytes.clone().cast::<PyBytes>() {
        let bytes_u8 = py_bytes.as_bytes();
        Ok(hash(bytes_u8))
    } else {
        // Check for common unsupported types with helpful error messages
        let type_name = bytes.get_type().name()?.to_string_lossy().into_owned();
        match type_name.as_str() {
            "str" => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "crc32() expects bytes-like object, got string. Use b'your string' or your_string.encode() instead."
                ));
            },
            "int" => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "crc32() expects bytes-like object, got int. Use bytes([value]) or similar."
                ));
            },
            _ => {
                // Try to convert to bytes object (works for memoryview and other objects with __bytes__)
                match bytes.call_method0("__bytes__") {
                    Ok(py_bytes) => {
                        match py_bytes.cast::<PyBytes>() {
                            Ok(casted_bytes) => {
                                let bytes_u8 = casted_bytes.as_bytes();
                                Ok(hash(bytes_u8))
                            },
                            Err(_) => {
                                Err(pyo3::exceptions::PyTypeError::new_err(format!(
                                    "Object __bytes__() method did not return bytes, got {}",
                                    py_bytes.get_type().name()?
                                )))
                            }
                        }
                    },
                    Err(_) => {
                        Err(pyo3::exceptions::PyTypeError::new_err(format!(
                            "crc32() expects bytes-like object (bytes, bytearray, memoryview), got {}. Consider converting to bytes first.",
                            type_name
                        )))
                    }
                }
            }
        }
    }
}

/// Calculate CRC32 checksum of a file by path.
///
/// # Arguments
/// * `path` - Path to the file to checksum
///
/// # Returns
/// CRC32 checksum as a 32-bit unsigned integer
///
/// # Errors
/// Returns FileNotFoundError if file doesn't exist
/// Returns PermissionError for permission issues
/// Returns IOError for other I/O problems
#[pyfunction]
fn crc32_file(path: &Bound<'_, PyString>) -> PyResult<u32> {
    let path_str = path.to_str()
        .map_err(|e| pyo3::exceptions::PyTypeError::new_err(format!("Invalid path string: {}", e)))?;
    let path = Path::new(path_str);

    // Check if file exists first for better error messages
    if !path.exists() {
        return Err(pyo3::exceptions::PyFileNotFoundError::new_err(format!(
            "File not found: {}", path_str
        )));
    }

    let file = File::open(path).map_err(|e| {
        match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                pyo3::exceptions::PyPermissionError::new_err(format!(
                    "Permission denied when opening file: {}", path_str
                ))
            },
            _ => {
                pyo3::exceptions::PyIOError::new_err(format!(
                    "Failed to open file '{}': {}", path_str, e
                ))
            }
        }
    })?;

    let mut reader = BufReader::new(file);
    let mut hasher = RustHasher::new();
    let mut buffer = [0; 8192]; // 8KB buffer for efficient reading

    loop {
        let bytes_read = reader.read(&mut buffer)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to read file: {}", e)))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize())
}

/// Calculate CRC32 checksum of a file-like object.
///
/// # Arguments
/// * `fileobj` - File-like object with read() method
///
/// # Returns
/// CRC32 checksum as a 32-bit unsigned integer
///
/// # Errors
/// Returns TypeError if object doesn't support read() method
/// Returns ValueError if read() returns invalid data
#[pyfunction]
fn crc32_fileobj(fileobj: &Bound<'_, PyAny>) -> PyResult<u32> {
    // Check if object has read method
    if !fileobj.hasattr("read")? {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "Object must have a read() method to be used as a file-like object"
        ));
    }

    let mut hasher = RustHasher::new();
    let chunk_size = 8192; // 8KB chunks

    loop {
        // Try to read a chunk from the file object
        let chunk_result = fileobj.call_method1("read", (chunk_size,));

        match chunk_result {
            Ok(chunk) => {
                // Check if we got EOF (empty bytes or None)
                if chunk.is_none() {
                    break;
                }

                // Convert chunk to bytes
                if let Ok(py_bytes) = chunk.clone().cast::<PyBytes>() {
                    let data = py_bytes.as_bytes();
                    if data.is_empty() {
                        break; // EOF
                    }
                    hasher.update(data);
                } else {
                    // Try to convert to bytes if not already bytes
                    match chunk.call_method0("__bytes__") {
                        Ok(bytes_obj) => {
                            match bytes_obj.cast::<PyBytes>() {
                                Ok(casted_bytes) => {
                                    let data = casted_bytes.as_bytes();
                                    if data.is_empty() {
                                        break; // EOF
                                    }
                                    hasher.update(data);
                                },
                                Err(_) => {
                                    return Err(pyo3::exceptions::PyValueError::new_err(
                                        "Object __bytes__() method did not return bytes"
                                    ));
                                }
                            }
                        },
                        Err(_) => {
                            return Err(pyo3::exceptions::PyValueError::new_err(
                                "read() method returned object that cannot be converted to bytes"
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                    "Failed to read from file-like object: {}", e
                )));
            }
        }
    }

    Ok(hasher.finalize())
}

/// Calculates the CRC32C hash using the Castagnoli polynomial (iSCSI, Ethernet, etc.).
///
/// # Arguments
/// * `bytes` - Any bytes-like object that can be converted to bytes
///
/// # Returns
/// CRC32C checksum as a 32-bit unsigned integer
///
/// # Examples
/// ```python
/// import pycrc32
/// result = pycrc32.crc32c(b"hello")
/// # result = 2240272413
/// ```
///
/// # Note
/// CRC32C uses a different polynomial than standard CRC32 and is commonly used
/// in modern storage systems, network protocols, and error detection.
#[pyfunction]
fn crc32c_func(bytes: &Bound<'_, PyAny>) -> PyResult<u32> {
    // Try to extract as bytes first
    if let Ok(py_bytes) = bytes.clone().cast::<PyBytes>() {
        let bytes_u8 = py_bytes.as_bytes();
        Ok(crc32c(bytes_u8))
    } else {
        // Check for common unsupported types with helpful error messages
        let type_name = bytes.get_type().name()?.to_string_lossy().into_owned();
        match type_name.as_str() {
            "str" => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "crc32c() expects bytes-like object, got string. Use b'your string' or your_string.encode() instead."
                ));
            },
            "int" => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "crc32c() expects bytes-like object, got int. Use bytes([value]) or similar."
                ));
            },
            _ => {
                // Try to convert to bytes object (works for memoryview and other objects with __bytes__)
                match bytes.call_method0("__bytes__") {
                    Ok(py_bytes) => {
                        match py_bytes.cast::<PyBytes>() {
                            Ok(casted_bytes) => {
                                let bytes_u8 = casted_bytes.as_bytes();
                                Ok(crc32c(bytes_u8))
                            },
                            Err(_) => {
                                Err(pyo3::exceptions::PyTypeError::new_err(format!(
                                    "Object __bytes__() method did not return bytes, got {}",
                                    py_bytes.get_type().name()?
                                )))
                            }
                        }
                    },
                    Err(_) => {
                        Err(pyo3::exceptions::PyTypeError::new_err(format!(
                            "crc32c() expects bytes-like object (bytes, bytearray, memoryview), got {}. Consider converting to bytes first.",
                            type_name
                        )))
                    }
                }
            }
        }
    }
}

/// Represents an in-progress CRC32 computation in Python.
///
/// This class provides incremental CRC32 calculation, allowing you to process
/// large data in chunks without loading everything into memory at once.
///
/// # Examples
/// ```python
/// import pycrc32
///
/// # Simple incremental hashing
/// hasher = pycrc32.Hasher()
/// hasher.update(b"first chunk")
/// hasher.update(b"second chunk")
/// result = hasher.finalize()
///
/// # With initial state
/// hasher = pycrc32.Hasher.with_initial(0x12345678)
/// hasher.update(b"more data")
/// result = hasher.finalize()
///
/// # Context manager usage
/// with pycrc32.Hasher() as hasher:
///     hasher.update(b"data")
///     result = hasher.finalize()
/// ```
#[pyclass(module = "pycrc32")]
struct Hasher {
    hasher: RustHasher,
    bytes_processed: usize,
}

#[pymethods]
impl Hasher {
    /// Create a new Hasher with default initial state.
    ///
    /// # Returns
    /// New Hasher instance ready to process data
    #[new]
    fn new() -> Self {
        Hasher {
            hasher: RustHasher::new(),
            bytes_processed: 0,
        }
    }

    /// Create a new Hasher with a custom initial CRC32 state.
    ///
    /// # Arguments
    /// * `init` - Initial CRC32 value (32-bit unsigned integer)
    ///
    /// # Returns
    /// New Hasher instance with the specified initial state
    ///
    /// # Examples
    /// ```python
    /// # Continue a CRC32 calculation from a known state
    /// hasher = pycrc32.Hasher.with_initial(0x12345678)
    /// hasher.update(b"additional data")
    /// ```
    #[staticmethod]
    fn with_initial(init: u32) -> Self {
        Hasher {
            hasher: RustHasher::new_with_initial(init),
            bytes_processed: 0,
        }
    }

    /// Process the given bytes-like object and update the hash state.
    ///
    /// # Arguments
    /// * `data` - Any bytes-like object (bytes, memoryview, etc.)
    ///
    /// # Examples
    /// ```python
    /// hasher = pycrc32.Hasher()
    /// hasher.update(b"first chunk")
    /// hasher.update(bytearray(b"second chunk"))
    /// ```
    fn update(&mut self, data: &Bound<'_, PyAny>) -> PyResult<()> {
        // Try to extract as bytes first
        if let Ok(py_bytes) = data.clone().cast::<PyBytes>() {
            let data_u8 = py_bytes.as_bytes();
            self.hasher.update(data_u8);
            self.bytes_processed += data_u8.len();
            Ok(())
        } else {
            // Check for common unsupported types with helpful error messages
            let type_name = data.get_type().name()?.to_string_lossy().into_owned();
            match type_name.as_str() {
                "str" => {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "Hasher.update() expects bytes-like object, got string. Use b'your string' or your_string.encode() instead."
                    ));
                },
                "int" => {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "Hasher.update() expects bytes-like object, got int. Use bytes([value]) or similar."
                    ));
                },
                _ => {
                    // Try to convert to bytes object (works for memoryview and other objects with __bytes__)
                    match data.call_method0("__bytes__") {
                        Ok(py_bytes) => {
                            match py_bytes.cast::<PyBytes>() {
                                Ok(casted_bytes) => {
                                    let data_u8 = casted_bytes.as_bytes();
                                    self.hasher.update(data_u8);
                                    self.bytes_processed += data_u8.len();
                                    Ok(())
                                },
                                Err(_) => {
                                    Err(pyo3::exceptions::PyTypeError::new_err(format!(
                                        "Object __bytes__() method did not return bytes, got {}",
                                        py_bytes.get_type().name()?
                                    )))
                                }
                            }
                        },
                        Err(_) => {
                            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                                "Hasher.update() expects bytes-like object (bytes, bytearray, memoryview), got {}. Consider converting to bytes first.",
                                type_name
                            )))
                        }
                    }
                }
            }
        }
    }

    /// Finalize the hash state and return the computed CRC32 value.
    ///
    /// # Returns
    /// Current CRC32 checksum as a 32-bit unsigned integer
    ///
    /// Note: This does not modify the hasher state, so you can continue
    /// calling update() and finalize() to process more data.
    fn finalize(&self) -> u32 {
        self.hasher.clone().finalize()
    }

    /// Reset the hash state to initial values.
    ///
    /// After calling reset(), the hasher behaves as if newly created.
    /// The bytes processed counter is reset to 0.
    fn reset(&mut self) {
        self.hasher.reset();
        self.bytes_processed = 0;
    }

    /// Combine this hasher's state with another hasher's state.
    ///
    /// This is useful for parallel processing where different chunks
    /// are processed separately and then combined.
    ///
    /// # Arguments
    /// * `other` - Another Hasher instance to combine with this one
    ///
    /// # Examples
    /// ```python
    /// # Process data in parallel and combine results
    /// hasher1 = pycrc32.Hasher()
    /// hasher1.update(b"chunk1")
    ///
    /// hasher2 = pycrc32.Hasher()
    /// hasher2.update(b"chunk2")
    ///
    /// hasher1.combine(hasher2)
    /// result = hasher1.finalize()  # Combined result
    /// ```
    fn combine(&mut self, other: &Hasher) {
        self.hasher.combine(&other.hasher);
        self.bytes_processed += other.bytes_processed;
    }

    /// Create a copy of this hasher with the same state.
    ///
    /// # Returns
    /// New Hasher instance with identical internal state
    ///
    /// # Examples
    /// ```python
    /// # Create a backup before modifying
    /// hasher1 = pycrc32.Hasher()
    /// hasher1.update(b"data")
    /// hasher2 = hasher1.copy()
    ///
    /// hasher2.update(b"more data")
    /// # hasher1 still has just "data", hasher2 has "data" + "more data"
    /// ```
    fn copy(&self) -> Self {
        Hasher {
            hasher: self.hasher.clone(),
            bytes_processed: self.bytes_processed,
        }
    }

    /// Return the number of bytes processed by this hasher.
    fn __len__(&self) -> usize {
        self.bytes_processed
    }

    /// Return a string representation of the hasher.
    fn __repr__(&self) -> String {
        format!(
            "Hasher(bytes_processed={}, current_crc={:#x})",
            self.bytes_processed,
            self.hasher.clone().finalize()
        )
    }

    /// Enter context manager protocol.
    fn __enter__(&self) -> Self {
        self.copy()
    }

    /// Exit context manager protocol.
    fn __exit__(&mut self, _exc_type: Option<Py<PyAny>>, _exc_val: Option<Py<PyAny>>, _exc_tb: Option<Py<PyAny>>) -> PyResult<()> {
        Ok(())
    }
}

/// Python module that provides bindings for the crc32fast crate
#[pymodule]
fn pycrc32(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Hasher>()?;
    m.add_function(wrap_pyfunction!(crc32, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_file, m)?)?;
    m.add_function(wrap_pyfunction!(crc32_fileobj, m)?)?;
    m.add_function(wrap_pyfunction!(crc32c_func, m)?)?;
    Ok(())
}
