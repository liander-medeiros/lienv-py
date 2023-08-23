use pyo3::prelude::*;
use std::str::FromStr;

pub fn to_pyobject<T>(str_content: &str) -> PyResult<PyObject>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Display,
    T: pyo3::ToPyObject,
{
    let parse_result = str_content.trim().parse::<T>();
    Python::with_gil(|py| match parse_result {
        Ok(value) => Ok(value.to_object(py)),
        Err(e) => Err(
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        ),
    })
}
