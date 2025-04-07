use pyo3::prelude::*;

use ::hat_splitter::{Splitter, WhitespaceSplitter};

#[pyclass(frozen, name = "WhitespaceSplitter")]
struct PyWhitespaceSplitter {
    splitter: WhitespaceSplitter,
}

#[pymethods]
impl PyWhitespaceSplitter {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            splitter: WhitespaceSplitter,
        })
    }

    fn split<'a>(&self, input: &'a str) -> PyResult<Vec<&'a str>> {
        Ok(self.splitter.split(input))
    }
}

#[pymodule]
mod hat_splitter {
    #[pymodule_export]
    use super::PyWhitespaceSplitter;
}
