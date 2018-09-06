extern crate mangofmt;
extern crate libc;

mod util;

use std::path::Path;
use std::ffi::{CStr, CString};
use libc::c_char;
use std::ptr;
use mangofmt::MangoFile;
use mangofmt::MangoImage;
use mangofmt::meta::MangoMetadata;

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
pub extern "C" fn mangofile_add_image_by_path(pointer: *mut MangoFile, path: *const c_char) {
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

// Save
#[no_mangle]
pub extern "C" fn mangofile_save(pointer: *mut MangoFile, path_pointer: *mut c_char) {
    let mut file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !path_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_pointer) };
        if let Ok(value) = c_str.to_str() {
            file.save(std::path::Path::new(value))
        }
    }
}

#[no_mangle]
pub extern "C" fn mangofile_save_cbor(pointer: *mut MangoFile, path_pointer: *mut c_char) {
    let mut file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !path_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_pointer) };
        if let Ok(value) = c_str.to_str() {
            file.save_cbor(std::path::Path::new(value))
        }
    }
}

#[no_mangle]
pub extern "C" fn mangofile_save_bson(pointer: *mut MangoFile, path_pointer: *mut c_char) {
    let mut file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    if !path_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_pointer) };
        if let Ok(value) = c_str.to_str() {
            file.save_bson(std::path::Path::new(value))
        }
    }
}

#[no_mangle]
pub extern "C" fn mangofile_save_json(pointer: *mut MangoFile, path_pointer: *mut c_char) {
    let mut file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    if !path_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_pointer) };
        if let Ok(value) = c_str.to_str() {
            file.save_json(std::path::Path::new(value))
        }
    }
}

// Open
#[no_mangle]
pub extern "C" fn mangofile_open(path_pointer: *mut c_char) -> *mut MangoFile {
    if !path_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_pointer) };
        if let Ok(path) = c_str.to_str() {
            let file = MangoFile::open(Path::new(path));
            if file.is_ok() {
                Box::into_raw(Box::new(file.unwrap()));
            }
        }
    }

    std::ptr::null_mut()
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
#[no_mangle]
pub extern "C" fn mangoimg_from_path(value_pointer: *mut c_char) -> *mut MangoImage {
    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {
            use std::fs;
            use std::error::Error;
            let data = "Some data!";
            fs::write("/tmp/foo", value).expect("Unable to write file");

            use mangofmt::ImageFile;

            let mut file = ImageFile::open(std::path::Path::new(value));
            if file.is_ok() {
                let mut img = file.unwrap().to_mango_image();
                //println!("{}", img.clone().get_meta().checksum);
                return Box::into_raw(Box::new(img));
            }
        }
    }

    std::ptr::null_mut()
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageData {
    pub pointer: *mut u8,
    pub length: usize,
}

use std::slice;

#[no_mangle]
pub unsafe extern "C" fn mango_imagedata_free(bytes: ImageData) {
    let ImageData { pointer, length } = bytes;

    // Re-create the slice from the pointer and length. The cast is because we
    // are working in terms of a raw pointer, not a mutable reference with the
    // arbitrary lifetime it would come up with.
    let slice = slice::from_raw_parts_mut(pointer, length) as *mut _;

    // Re-create the boxed slice, and drop it. This will deallocate t he memory.
    drop(Box::from_raw(slice));
}

#[no_mangle]
pub extern "C" fn mangoimg_get_image_data(pointer: *mut MangoImage) -> ImageData {
	let mut img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    // For simplicity, make the Vec into a boxed slice.
    // This way we do not have to think of capacity and length separately.
    // This is zero-cost if the global allocator is worth its salt.
    let slice: Box<[u8]> = img.get_image_data().into_boxed_slice();

    // Save the length now because otherwise we have to do it unsafely.
    let length = slice.len();

    // Retrieve the raw pointer from the boxed slice. The cast is because we cannot
    // transmit a pointer to a slice (which has unstable ABI) to C, and we have the
    // length anyway.
    let pointer = Box::into_raw(slice) as *mut u8;

    ImageData {
		    pointer,
        length,
	  }
}


#[no_mangle]
pub extern "C" fn mangoimg_get_base64_image_data(pointer: *mut MangoImage) -> *mut c_char {
    let img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    CString::new(img.get_base64_image_data()).unwrap().into_raw()
}

use mangofmt::MangoImageMetadata;

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
pub extern "C" fn mangoimgmeta_encryption(pointer: *mut MangoImageMetadata) -> *mut c_char {
    let meta: &mut MangoImageMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match meta.encryption.clone() {
        Some(value) => CString::new(util::from_enc_type(value)).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangoimgmeta_checksum(pointer: *mut MangoImageMetadata) -> *mut c_char {
    let meta: &mut MangoImageMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    use std::fs;
    fs::write("/tmp/sum", meta.checksum.clone()).expect("Unable to write file");
    // TODO fix this
    if meta.checksum.len() > 0 {
        return CString::new(meta.checksum.clone()).unwrap().into_raw();
    } else {
        return CString::new("OH NO!".to_string()).unwrap().into_raw();
    }
}
