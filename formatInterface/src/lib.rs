extern crate mango_format;
extern crate libc;

mod util;

use std::path::Path;
use std::ffi::{CStr, CString};
use libc::c_char;
use std::ptr;
use mango_format::MangoFile;
use mango_format::MangoImage;
use mango_format::meta::MangoMetadata;

//----------------------------------------------------------------------------------------
// Mango File
//----------------------------------------------------------------------------------------
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


#[no_mangle]
pub extern "C" fn mangofile_get_meta(pointer: *mut MangoFile) -> *mut MangoMetadata {
    let mut file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    let meta = file.get_meta_mut();
    let p_mut: *mut MangoMetadata = meta;
    p_mut
}

//----------------------------------------------------------------------------------------
// Mango File Metadata
//----------------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn mangometa_get_title(pointer: *mut MangoMetadata) -> *mut c_char {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match meta.clone().title {
        Some(x) => {
            CString::new(x).unwrap().into_raw()
        },
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_title(pointer: *mut MangoMetadata, value_pointer: *mut c_char) {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {
            meta.title = Some(value.to_string());
        }
    } else {
        meta.title = None;
    }
}

#[no_mangle]
pub extern "C" fn mangometa_get_author(pointer: *mut MangoMetadata) -> *mut c_char {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match meta.clone().author {
        Some(x) => {
            CString::new(x).unwrap().into_raw()
        },
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_author(pointer: *mut MangoMetadata, value_pointer: *mut c_char) {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {

            meta.author = Some(value.to_string());
        }
    } else {
        meta.author = None;
    }
}

#[no_mangle]
pub extern "C" fn mangometa_get_publisher(pointer: *mut MangoMetadata) -> *mut c_char {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match meta.clone().publisher {
        Some(x) => {
            CString::new(x).unwrap().into_raw()
        },
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_publisher(pointer: *mut MangoMetadata, value_pointer: *mut c_char) {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {

            meta.publisher = Some(value.to_string());
        }
    } else {
        meta.publisher = None;
    }
}

#[no_mangle]
pub extern "C" fn mangometa_get_source(pointer: *mut MangoMetadata) -> *mut c_char {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match meta.clone().source {
        Some(x) => {
            CString::new(x).unwrap().into_raw()
        },
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_source(pointer: *mut MangoMetadata, value_pointer: *mut c_char) {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {

            meta.source = Some(value.to_string());
        }
    } else {
        meta.source = None;
    }
}

#[no_mangle]
pub extern "C" fn mangometa_get_translation(pointer: *mut MangoMetadata) -> *mut c_char {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match meta.clone().translation {
        Some(x) => {
            CString::new(x).unwrap().into_raw()
        },
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_translation(pointer: *mut MangoMetadata, value_pointer: *mut c_char) {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {

            meta.translation = Some(value.to_string());
        }
    } else {
        meta.translation = None;
    }
}

//----------------------------------------------------------------------------------------
// Mango Image
//----------------------------------------------------------------------------------------
pub extern "C" fn mangoimg_from_path(value_pointer: *mut c_char) -> *mut MangoImage {
    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {
            use mango_format::ImageFile;
            // TODO this should only be temporary
            let mut img = MangoImage::from_file(&mut ImageFile::open(std::path::Path::new(value)).unwrap());
            let p_mut: *mut MangoImage = &mut img;
            return p_mut;
        }
    }

    std::ptr::null_mut()
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
pub extern "C" fn mangoimg_compress(pointer: *mut MangoImage, value_pointer: *mut c_char) -> i8 {
    let mut img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {
            match util::to_comp_type(value.to_string()) {
                Some(comptype) => {
                    return match img.compress_mut(comptype) {
                        true => 1,
                        false => 2,
                    }
                },
                None => {
                    return 2;
                },
            }
        }
    }

    2
}

#[no_mangle]
pub extern "C" fn mangoimg_uncompress(pointer: *mut MangoImage) -> i8 {
    let mut img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match img.uncompress_mut() {
        true => 1,
        false => 2,
    }
}

//----------------------------------------------------------------------------------------
// Mango Imagemetadata
//----------------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn mangoimgmeta_compression(pointer: *mut MangoImageMetadata) -> *mut c_char {
    let meta: &mut MangoImageMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match meta.compression.clone() {
        Some(value) => CString::new(util::from_comp_type(value)).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangoimgmeta_checksum(pointer: *mut MangoImageMetadata) -> *mut c_char {
    let meta: &mut MangoImageMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    CString::new(meta.checksum.clone()).unwrap().into_raw()
}
