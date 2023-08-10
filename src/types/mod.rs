use pyo3::prelude::*;

mod impl_fromstr;
mod impl_topyobject;

pub struct List(PyObject);
pub struct Dict(PyObject);

pub struct Tuple(PyObject);

pub struct Bool(bool);