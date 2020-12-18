use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn multiply_array(a: Vec<usize>, b: usize) -> PyResult<Vec<usize>> {
    Ok(a.iter().map(|v| v * b).collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply_array, m)?)?;
    Ok(())
}
