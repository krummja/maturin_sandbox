mod parallel_jaccard_mod;

use pyo3::prelude::*;
use polars::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    result: String = (a + b).to_string();
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn sandbox(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
