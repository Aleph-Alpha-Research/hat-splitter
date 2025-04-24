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

    fn split(&self, text: &str) -> PyResult<Vec<String>> {
        Ok(self.splitter.split(text))
    }

    fn split_with_limit(&self, text: &str, max_bytes_per_word: usize) -> PyResult<Vec<Vec<u8>>> {
        Ok(self.splitter.split_with_limit(text, max_bytes_per_word))
    }
}

#[pymodule]
mod hat_splitter {
    #[pymodule_export]
    use super::PyHATSplitter;
}
