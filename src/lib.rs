use pyo3::prelude::*;

use crc32fast::hash;
use crc32fast::Hasher as RustHasher;

/// Calculates the crc32 hash using the crc32fast crate
#[pyfunction]
fn crc32(bytes: &[u8]) -> u32 {
    hash(bytes)
}

/// Represents an in-progress CRC32 computation in Python.
#[pyclass(module = "pycrc32")]
struct Hasher {
    hasher: RustHasher,
}

#[pymethods]
impl Hasher {
    #[new]
    fn new() -> Self {
        Hasher {
            hasher: RustHasher::new(),
        }
    }

    /// Create a new `Hasher` with an initial CRC32 state.
    #[staticmethod]
    fn with_initial(init: u32) -> Self {
        Hasher {
            hasher: RustHasher::new_with_initial(init),
        }
    }

    /// Process the given byte slice and update the hash state.
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    /// Finalize the hash state and return the computed CRC32 value.
    fn finalize(&self) -> u32 {
        self.hasher.clone().finalize()
    }

    /// Reset the hash state.
    fn reset(&mut self) {
        self.hasher.reset();
    }

    /// Combine the hash state with the hash state for the subsequent block of bytes.
    fn combine(&mut self, other: &Hasher) {
        self.hasher.combine(&other.hasher);
    }
}

/// Python module that provides bindings for the crc32fast crate
#[pymodule]
fn pycrc32(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Hasher>()?;
    m.add_function(wrap_pyfunction!(crc32, m)?)?;
    Ok(())
}
