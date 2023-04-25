use pyo3::exceptions::PyIndexError;
use pyo3::{pyclass, PyErr};
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct OutOfBoundsError;

impl std::convert::From<OutOfBoundsError> for PyErr {
    fn from(err: OutOfBoundsError) -> PyErr {
        PyIndexError::new_err("Index out of bounds")
    }
}
