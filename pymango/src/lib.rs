#![feature(proc_macro, specialization)]
#![feature(const_fn, const_align_of, const_size_of, const_ptr_null, const_ptr_null_mut)]

extern crate pyo3;
extern crate mango_format;

mod enums;

use pyo3::prelude::*;
use pyo3::{PyResult, Python, PyModule};
use pyo3::py::modinit as pymodinit;
use pyo3::py::methods;
use pyo3::py::class as pyclass;

use mango_format::MangoFile;
use mango_format::image::{Base64Image, ImageFile};



#[pyclass]
struct PyMangoImage {
    img: Base64Image,
    token: PyToken,
}

#[methods]
impl PyMangoImage {

    #[new]
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        let path = std::path::Path::new(&path);
        let mut img = ImageFile::open(path)?;
        obj.init(|t|  PyMangoImage {img: Base64Image::from_file(&mut img), token: t})
    }

    pub fn get_image(&self) -> PyResult<String> {
        Ok(self.img.get_image())
    }

    pub fn compress(&self, type_string: String) -> PyResult<bool> {
        let type_enum = enums::compression(type_string);

        if type_enum.is_some() {
            self.img.compress(type_enum.unwrap());
            return Ok(true);
        }

        Ok(false)
    }
}

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

    pub fn get_image(&self, py: Python, index: usize) -> PyResult<Py<PyMangoImage>> {
        let img: Base64Image = self.file.get_image(index).clone();
        py.init(|token|  PyMangoImage {img, token})
    }

}



#[pymodinit(mango)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyMangoFile>()?;
    m.add_class::<PyMangoImage>()?;
    Ok(())
}
