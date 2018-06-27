#![feature(proc_macro, specialization)]
#![feature(const_fn, const_align_of, const_size_of, const_ptr_null, const_ptr_null_mut)]

extern crate pyo3;
extern crate mango_format;

mod enums;

use std::string::ToString;

use pyo3::prelude::*;
use pyo3::{PyResult, Python, PyModule};
use pyo3::py::modinit as pymodinit;
use pyo3::py::methods;
use pyo3::py::class as pyclass;
use pyo3::py::*;

use mango_format::{MangoFile, MangoImage, ImageFile};
use mango_format::MangoImageMetadata;
use mango_format::meta::MangoMetadata;

#[pyclass]
struct PyMangoMetadata {
    meta: MangoMetadata,
    token: PyToken,
}

#[methods]
impl PyMangoMetadata {
    #[new]
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        obj.init(|t|  PyMangoMetadata {meta: MangoMetadata::new(), token: t})
    }

    #[getter]
    fn get_author(&self) -> PyResult<Option<String>> {
        Ok(self.meta.author.clone())
    }

    #[setter]
    fn set_author(&mut self, value: Option<String>) -> PyResult<()> {
        self.meta.author = value;
        Ok(())
    }
}

impl PyMangoMetadata {
    pub fn get_meta(&self) -> MangoMetadata {
        self.meta.clone()
    }
}

#[pyclass]
struct PyMangoImageMetadata {
    meta: MangoImageMetadata,
    token: PyToken,
}

#[methods]
impl PyMangoImageMetadata {
    #[getter]
    pub fn get_checksum(&self) -> PyResult<String> {
        Ok(self.meta.checksum.clone())
    }

    pub fn _get_encryption(&self) -> PyResult<Option<String>>{
        let encryption = self.meta.encryption.clone();
        match encryption {
            Some(v) => Ok(Some(v.to_string())),
            None => Ok(None),
        }
    }

    pub fn _get_compression(&self) -> PyResult<Option<String>>{
        let encryption = self.meta.compression.clone();
        match encryption {
            Some(v) => Ok(Some(v.to_string())),
            None => Ok(None),
        }
    }
}

#[pyclass]
struct PyMangoImage {
    img:MangoImage,
    token: PyToken,
}

#[methods]
impl PyMangoImage {

    #[new]
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        let path = std::path::Path::new(&path);
        let mut img = ImageFile::open(path)?;
        obj.init(|t|  PyMangoImage {img: MangoImage::from_file(&mut img), token: t})
    }

    #[getter]
    pub fn get_image_data(&self) -> PyResult<Vec<u8>> {
        Ok(self.img.get_image_data())
    }

    pub fn compress(&mut self, type_string: String) -> PyResult<bool> {
        let type_enum = enums::compression(type_string);

        if type_enum.is_some() {
            let status = self.img.compress_mut(type_enum.unwrap());
            return Ok(status);
        }

        Ok(false)
    }

    pub fn uncompress(&mut self) -> PyResult<bool> {
            let status = self.img.uncompress_mut();
            Ok(status)
    }

    pub fn _get_meta_data(&self, py: Python) -> PyResult<Py<PyMangoImageMetadata>> {
        let meta = self.img.get_meta().clone();
        py.init(|token|  PyMangoImageMetadata {meta, token})
    }

    pub fn save(&self, filename: String) -> PyResult<()> {
        self.img.save(&filename);
        Ok(())
    }

}

impl PyMangoImage {
    pub fn get_base64_image(&self) -> MangoImage {
        return self.img.clone()
    }
}

//------------------------------------------------------------------------------
// MangoFile
//------------------------------------------------------------------------------
#[pyclass(subclass)]
struct PyMangoFile {
    file: MangoFile,
    token: PyToken,
}

#[methods]
impl PyMangoFile {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|t|  PyMangoFile {file: MangoFile::new(), token: t})
    }

    pub fn add_image_by_path(&mut self, path: String) -> PyResult<()> {
        let path = std::path::Path::new(&path);
        &self.file.add_image_by_path(&path);
        Ok(())
    }

    pub fn _add_image(&mut self, _py: Python, image_ptr: Py<PyMangoImage>) -> PyResult<()> {
        let image: &PyMangoImage = image_ptr.as_ref(_py);
        self.file.add_image(image.get_base64_image());
        Ok(())
    }

    pub fn save(&self, path: String) -> PyResult<()> {
        let path = std::path::Path::new(&path);
        &self.file.save(&path);
        Ok(())
    }

    pub fn _get_image(&self, py: Python, index: usize) -> PyResult<Py<PyMangoImage>> {
        let img_option = self.file.get_image(index);

        if img_option.is_none() {
            return Err(exc::IndexError::new("index does not exist"));
        }
        let img: MangoImage = img_option.unwrap().clone();
        py.init(|token|  PyMangoImage {img, token})
    }

    #[getter]
    pub fn get_meta(&self) -> PyResult<&PyMangoMetadata> {
        let meta = self.file.get_meta_ref().clone();
        self.token.py().init_ref(|token|  PyMangoMetadata {meta, token})
    }

    #[setter]
    pub fn set_meta(&mut self, meta_ptr: Py<PyMangoMetadata>) -> PyResult<()> {
        let meta: &PyMangoMetadata = meta_ptr.as_ref(self.token.py());
        self.file.set_meta(meta.get_meta());
        Ok(())
    }
}



#[pymodinit(_rust_pymango)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyMangoFile>()?;
    m.add_class::<PyMangoImage>()?;
    m.add_class::<PyMangoMetadata>()?;
    m.add_class::<PyMangoImageMetadata>()?;
    Ok(())
}
