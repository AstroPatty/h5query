use pyo3::prelude::*;

mod column;
mod filter;
mod query;
mod select;
mod stage;
mod take;

/// A Python module implemented in Rust.
#[pymodule]
mod h5query {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
