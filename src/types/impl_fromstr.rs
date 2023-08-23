use pyo3::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParsingError(&'static str);

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for super::Bool {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(parsed_value) = s.trim().to_lowercase().parse::<bool>() {
            Ok(super::Bool(parsed_value))
        } else {
            Err(ParsingError("Invalid value for type 'boolean'"))
        }
    }
}

impl FromStr for super::Dict {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Python::with_gil(|py: Python<'_>| {
            let result = py
                .eval(s, None, None)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e));
            match result {
                Ok(value) => match value.extract::<&pyo3::types::PyDict>(){
                    Ok(dict) => Ok(super::Dict(dict.to_object(py))),
                    Err(_) => Err(ParsingError("Invalid value for type 'dictionary'"))
                },
                Err(_) => Err(ParsingError("Invalid value for type 'dictionary'"))
            }
        })
    }
}

impl FromStr for super::List {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Python::with_gil(|py: Python<'_>| {
            let result = py
                .eval(s, None, None)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e));
            match result {
                Ok(value) => match value.extract::<&pyo3::types::PyList>(){
                    Ok(list) => Ok(super::List(list.to_object(py))),
                    Err(_) => Err(ParsingError("Invalid value for type 'list'"))
                },
                Err(_) => Err(ParsingError("Invalid value for type 'list'"))
            }
        })
    }
}

impl FromStr for super::Tuple {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Python::with_gil(|py: Python<'_>| {
            let result = py
                .eval(s, None, None)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e));
            match result {
                Ok(value) => match value.extract::<&pyo3::types::PyTuple>(){
                    Ok(tuple) => Ok(super::Tuple(tuple.to_object(py))),
                    Err(_) => Err(ParsingError("Invalid value for type 'tuple'"))
                },
                Err(_) => Err(ParsingError("Invalid value for type 'tuple'"))
            }
        })
    }
}