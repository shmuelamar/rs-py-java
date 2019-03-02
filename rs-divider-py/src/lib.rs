#[macro_use]
extern crate pyo3;
extern crate rsdivider;

use pyo3::prelude::*;

/// return the result of `a / b`
#[pyfunction]
fn div_numbers(a: f64, b: f64) -> PyResult<f64> {
    Ok(rsdivider::div_numbers(a, b))
}

/// divider python module impl in rust
#[pymodule]
fn rsdivider_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(div_numbers))?;
    Ok(())
}
