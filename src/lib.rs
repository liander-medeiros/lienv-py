use pyo3::prelude::*;
use pyo3::types::*;

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::env::VarError;
use std::sync::Mutex;

mod types;
mod parser;

type TypeHandlerFn = fn(&str) -> PyResult<PyObject>;

lazy_static!{
    static ref HANDLERS: Mutex<HashMap<String, TypeHandlerFn>> = Mutex::new(HashMap::new());
}

fn add_handler(new_type: &PyType, handler: TypeHandlerFn){
    let key: String = new_type.name().unwrap().to_string();
    HANDLERS.lock().unwrap().insert(key, handler);
}


/// Parse the content of an environment variable into an object of the specified type.
#[pyfunction]
fn get<'py>(py: pyo3::Python<'py>, var_type: &PyType, var_name: &str) -> PyResult<PyObject> {
    let var_content: String = match std::env::var(var_name) {
        Ok(content) => content,
        Err(e) => match e {
            VarError::NotPresent => return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("The environment variable '{}' is not defined", var_name),
            )),
            VarError::NotUnicode(_) => return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("The environment variable '{}' has an invalid name", var_name),
            )),
        }
    };

    let key = var_type.name().unwrap().to_string();
    let binding = HANDLERS.lock().unwrap();
    let parser = binding.get(&key);
    let result = match parser {
        Some(parser) => parser(var_content.as_str()),
        None => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            format!("There is no defined parser for type '{}'", key),
        ))
    };
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            let message = e.value(py).to_string();
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!(
                    "An error ocurred while parsing the environment variable '{}': {}",
                    var_name,
                    message
                ),
            ))
        }
    }
}

/// A Python module to parse content stored in environment variables
#[pymodule]
fn lienv(_py: Python, m: &PyModule) -> PyResult<()> {
    add_handler(_py.get_type::<PyInt>(), parser::to_pyobject::<types::Int>);
    add_handler(_py.get_type::<PyFloat>(), parser::to_pyobject::<types::Float>);
    add_handler(_py.get_type::<PyString>(), parser::to_pyobject::<types::String>);
    add_handler(_py.get_type::<PyDict>(), parser::to_pyobject::<types::Dict>);
    add_handler(_py.get_type::<PyList>(), parser::to_pyobject::<types::List>);
    add_handler(_py.get_type::<PyTuple>(), parser::to_pyobject::<types::Tuple>);
    add_handler(_py.get_type::<PyBool>(), parser::to_pyobject::<types::Bool>);
    m.add_function(wrap_pyfunction!(get, m)?)?;
    Ok(())
}
