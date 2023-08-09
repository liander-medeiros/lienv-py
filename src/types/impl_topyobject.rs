use pyo3::prelude::*;

impl ToPyObject for super::Dict {
    fn to_object(&self, _py: Python) -> PyObject {
        self.0.to_owned()
    }
}

impl ToPyObject for super::List {
    fn to_object(&self, _py: Python) -> PyObject {
        self.0.to_owned()
    }
}