use pyo3::prelude::*;

impl ToPyObject for super::Bool {
    fn to_object(&self, py: Python) -> PyObject {
        self.0.to_object(py)
    }
}

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

impl ToPyObject for super::Tuple {
    fn to_object(&self, _py: Python) -> PyObject {
        self.0.to_owned()
    }
}

impl ToPyObject for super::Int {
    fn to_object(&self, py: Python) -> PyObject {
        self.0.to_object(py)
    }
}

impl ToPyObject for super::Float {
    fn to_object(&self, py: Python) -> PyObject {
        self.0.to_object(py)
    }
}

impl ToPyObject for super::String {
    fn to_object(&self, py: Python) -> PyObject {
        self.0.to_object(py)
    }
}