extern crate mango_format;
extern crate libc;

use std::path::Path;
use std::ffi::{CStr, CString};
use libc::c_char;
use std::ptr;
use mango_format::MangoFile;
use mango_format::MangoImage;

#[no_mangle]
pub extern "C" fn new_mango_file() -> *mut MangoFile {
    Box::into_raw(Box::new(MangoFile::new()))
}

#[no_mangle]
pub extern "C" fn free_mangofile(pointer: *mut MangoFile) {
    if pointer.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(pointer);
    }
}

#[no_mangle]
pub extern "C" fn mangofile_add_image(pointer: *mut MangoFile, path: *const c_char) {
    let mut file = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    let c_str = unsafe {
        CStr::from_ptr(path)
    };
    let path_str  = c_str.to_str().unwrap();
    file.add_image_by_path(Path::new(&path_str.to_owned()));
}

#[no_mangle]
pub extern "C" fn mangofile_get_image(pointer: *mut MangoFile, index: usize) -> *mut MangoImage {
    let mut file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    let img = file.get_image_mut(index);
    let p_mut: *mut MangoImage = img;
    p_mut
}

use mango_format::MangoImageMetadata;


#[no_mangle]
pub extern "C" fn mangoimg_get_meta(pointer: *mut MangoImage) -> *mut MangoImageMetadata {
    let mut img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    let meta = img.get_meta_mut();
    let p_mut: *mut MangoImageMetadata = meta;
    p_mut
}


#[no_mangle]
pub extern "C" fn mangoimg_compress(pointer: *mut MangoImage) -> i8 {
    let mut img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    match img.compress_mut(mango_format::CompressionType::GZIP) {
        true => 1,
        false => 2,
    }
}

#[no_mangle]
pub extern "C" fn mangoimg_is_compressed(pointer: *mut MangoImage) -> i8 {
    let mut img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    match img.get_meta().compression.is_some() {
        true => 1,
        false => 0,
    }
}
