use pyo3::prelude::*;

use ::hat_splitter::{Splitter, HATSplitter};

#[pyclass(frozen, name = "HATSplitter")]
struct PyHATSplitter {
    splitter: HATSplitter,
}

#[pymethods]
impl PyHATSplitter {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            splitter: HATSplitter,
        })
    }

    fn split<'a>(&self, input: &'a str) -> PyResult<Vec<&'a str>> {
        Ok(self.splitter.split(input))
    }

    fn split_bytes<'a>(&self, input: &'a str) -> PyResult<Vec<&'a [u8]>> {
        let result = self.splitter.split(input);
        Ok(result.iter().map(|s| s.as_bytes()).collect())
    }
}

#[pymodule]
mod hat_splitter {
    #[pymodule_export]
    use super::PyHATSplitter;
}
