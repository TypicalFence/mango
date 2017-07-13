extern crate mango_format;
extern crate libc;

use std::path::Path;
use std::ffi::{CStr, CString};
use libc::c_char;
use std::ptr;
use mango_format::file::MangoFile;


#[no_mangle]
pub extern "C" fn kek(s: *mut c_char) -> *mut c_char {
    s
}


#[no_mangle]
pub extern "C" fn new_mango_file(s: *const c_char) -> *mut MangoFile {
    if s.is_null() {
        panic!("ASS");
        return ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(s) };
    if let Ok(name) = c_str.to_str() {
        Box::into_raw(Box::new(MangoFile::new(name.to_string())))
    } else {
        panic!("FUCK");
        return ptr::null_mut();
    }
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
pub extern "C" fn mangofile_add_image(pointer: *mut MangoFile, path: String) {
    let mut file = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    file.add_image(Path::new(path.as_str()))
}

#[no_mangle]
pub extern "C" fn mangofile_get_name(pointer: *mut MangoFile) -> *mut c_char {
    let file = unsafe {
        assert!(!pointer.is_null());
        &*pointer
    };
    let c_string = CString::new(file.get_name().clone()).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn mangofile_set_name(pointer: *mut MangoFile, name: String) {
    let mut file = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    file.set_name(name);
}
