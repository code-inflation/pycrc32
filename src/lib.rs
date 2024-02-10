use pyo3::prelude::*;

use crc32fast::hash;

/// Calculates the crc32 hash using the crc32fast crate
#[pyfunction]
fn crc32(bytes: &[u8]) -> u32 {
    hash(bytes)
}

/// Python module that provides bindings for the crc32fast crate
#[pymodule]
fn pycrc32(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crc32, m)?)?;
    Ok(())
}
