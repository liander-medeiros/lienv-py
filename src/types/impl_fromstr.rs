use pyo3::prelude::*;
use std::str::FromStr;

impl FromStr for super::Bool {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(parsed_value) = s.trim().to_lowercase().parse::<bool>() {
            Ok(super::Bool(parsed_value))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid boolean value",
            ))
        }
    }
}

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

impl FromStr for super::Tuple {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Python::with_gil(|py: Python<'_>| {
            let result = py
                .eval(s, None, None)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e))?;
            if let Ok(tuple) = result.extract::<&pyo3::types::PyTuple>() {
                Ok(super::Tuple(tuple.to_object(py)))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Invalid tuple object",
                ))
            }
        })
    }
}