use pyo3::prelude::*;
use std::str::FromStr;

pub fn to_pyobject<T>(str_content: &str) -> PyResult<PyObject>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
    T: pyo3::ToPyObject,
{
    let parse_result = str_content.trim().parse::<T>();
    Python::with_gil(|py| match parse_result {
        Ok(value) => Ok(value.to_object(py)),
        Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Parsing error: the content is not a valid '{}'", std::any::type_name::<T>()),
        )),
    })
}
