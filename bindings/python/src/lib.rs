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

    fn split_bytes(&self, input: &str) -> PyResult<Vec<Vec<u8>>> {
        let result = self.splitter.split(input);
        Ok(result.iter().map(|s| s.as_bytes().to_vec()).collect())
    }
}

#[pymodule]
mod hat_splitter {
    #[pymodule_export]
    use super::PyHATSplitter;
}
