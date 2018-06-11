#![feature(proc_macro, specialization)]
#![feature(const_fn, const_align_of, const_size_of, const_ptr_null, const_ptr_null_mut)]

extern crate pyo3;
extern crate mango_format;

use pyo3::prelude::*;
use pyo3::{PyResult, Python, PyModule};
use pyo3::py::modinit as pymodinit;
use pyo3::py::methods;
use pyo3::py::class as pyclass;

use mango_format::MangoFile;

#[pyclass]
struct PyMangoFile {
    file: MangoFile,
    token: PyToken,
}

#[methods]
impl PyMangoFile {

    #[new]
    fn __new__(obj: &PyRawObject, name: String) -> PyResult<()> {
        obj.init(|t|  PyMangoFile {file: MangoFile::new(name), token: t})
    }

    pub fn add_image(&mut self, path: String) -> PyResult<()> {
        let path = std::path::Path::new(&path);
        &self.file.add_image(&path);
        Ok(())
    }

    pub fn save(&self, path: String) -> PyResult<()> {
        let path = std::path::Path::new(&path);
        &self.file.save(&path);
        Ok(())
    }

}



#[pymodinit(mango)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyMangoFile>()?;

    Ok(())
}
