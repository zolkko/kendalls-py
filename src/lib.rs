use numpy::PyArray1;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::cmp::Ordering;


#[pyfunction]
fn taub(x: &PyArray1<f64>, y: &PyArray1<f64>) -> PyResult<f64> {
    let x_slice: &[f64] = unsafe {
        x.as_slice()
            .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?
    };
    let y_slice: &[f64] = unsafe {
        y.as_slice()
            .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?
    };
    let correlation = kendalls::tau_b_with_comparator(x_slice, y_slice, |a: &f64, b: &f64| {
        a.partial_cmp(b).unwrap_or(Ordering::Greater)
    })
    .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
    Ok(correlation)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn kendallspy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(taub))?;
    Ok(())
}
