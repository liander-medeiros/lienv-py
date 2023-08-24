use pyo3::prelude::*;
use fsize;

mod impl_fromstr;
mod impl_topyobject;

pub struct List(PyObject);
pub struct Dict(PyObject);
pub struct Tuple(PyObject);
pub struct Bool(bool);
pub struct Int(isize);
pub struct Float(fsize::fsize);
pub struct String(std::string::String);