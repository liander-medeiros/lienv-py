use pyo3::prelude::*;
use std::str::FromStr;

impl FromStr for super::Dict {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Python::with_gil(|py: Python<'_>| {
            let result = py
                .eval(s, None, None)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e))?;
            if let Ok(dict) = result.extract::<&pyo3::types::PyDict>() {
                Ok(super::Dict(dict.to_object(py)))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Invalid dictionary object",
                ))
            }
        })
    }
}

impl FromStr for super::List {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Python::with_gil(|py: Python<'_>| {
            let result = py
                .eval(s, None, None)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e))?;
            if let Ok(list) = result.extract::<&pyo3::types::PyList>() {
                Ok(super::List(list.to_object(py)))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Invalid list object",
                ))
            }
        })
    }
}