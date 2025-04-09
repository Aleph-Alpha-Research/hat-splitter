use pyo3::prelude::*;

use ::hat_splitter::{HATSplitter, Splitter};

#[pyclass(frozen, name = "HATSplitter")]
struct PyHATSplitter {
    splitter: HATSplitter,
}

#[pymethods]
impl PyHATSplitter {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            splitter: HATSplitter::new(),
        })
    }

    fn split(&self, input: &str) -> PyResult<Vec<String>> {
        Ok(self.splitter.split(input))
    }

    fn split_with_limit(&self, input: &str, max_bytes: usize) -> PyResult<Vec<Vec<u8>>> {
        Ok(self.splitter.split_with_limit(input, max_bytes))
    }
}

#[pymodule]
mod hat_splitter {
    #[pymodule_export]
    use super::PyHATSplitter;
}
