extern crate mangofmt;
mod util;

use std::ffi::{CStr, CString};
use std::fs;
use std::io;
use std::os::raw::{c_char, c_int, c_short};
use std::path::Path;
use std::slice;

use mangofmt::meta::MangoImageMetadata;
use mangofmt::meta::MangoMetadata;
use mangofmt::MangoFile;
use mangofmt::MangoImage;

//----------------------------------------------------------------------------------------
// Helper Structs
//----------------------------------------------------------------------------------------
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageData {
    pub pointer: *mut u8,
    pub length: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IntOption {
    pub value: c_int,
    pub present: c_int,
}

//----------------------------------------------------------------------------------------
// Support Checks
//----------------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn mango_encryption_is_supported(enc_type: *const c_char) -> bool {
    let enc_type_str = unsafe { CStr::from_ptr(enc_type).to_str() };

    if enc_type_str.is_ok() {
        let e_type = util::to_enc_type(enc_type_str.unwrap().to_string());
        return match e_type {
            Some(value) => value.is_supported(),
            None => false,
        };
    }

    false
}

#[no_mangle]
pub extern "C" fn mango_compression_is_supported(comp_type: *const c_char) -> bool {
    let comp_type_str = unsafe { CStr::from_ptr(comp_type).to_str() };

    if comp_type_str.is_ok() {
        let c_type = util::to_comp_type(comp_type_str.unwrap().to_string());
        return match c_type {
            Some(value) => value.is_supported(),
            None => false,
        };
    }

    false
}

//----------------------------------------------------------------------------------------
// Mango File
//----------------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn mangofile_new() -> *mut MangoFile {
    Box::into_raw(Box::new(MangoFile::new()))
}

#[no_mangle]
pub extern "C" fn mangofile_free(pointer: *mut MangoFile) {
    if pointer.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(pointer);
    }
}

#[no_mangle]
pub extern "C" fn mangofile_add_image_by_path(pointer: *mut MangoFile, path: *const c_char) -> i8 {
    if pointer.is_null() {
        return -42;
    }

    let file: &mut MangoFile = unsafe { &mut *pointer };

    let c_str = unsafe { CStr::from_ptr(path) };

    let path_str = c_str.to_str().unwrap();
    let error = file.add_image_by_path(Path::new(&path_str.to_owned()));

    match error {
        Ok(()) => 0,
        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied => 1,
            _ => -1,
        },
    }
}

#[no_mangle]
pub extern "C" fn mangofile_add_image(pointer: *mut MangoFile, img_pointer: *mut MangoImage) -> i8 {
    if pointer.is_null() {
        return -42;
    }

    let file: &mut MangoFile = unsafe { &mut *pointer };

    if img_pointer.is_null() {
        return -42;
    }

    let img = unsafe { &mut *img_pointer };

    file.add_image(img.clone());

    0
}

#[no_mangle]
pub extern "C" fn mangofile_get_image(pointer: *mut MangoFile, index: usize) -> *mut MangoImage {
    let file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    let img = file.get_image_mut(index);
    let p_mut: *mut MangoImage = img;
    p_mut
}

#[no_mangle]
pub extern "C" fn mangofile_set_image(
    file_pointer: *mut MangoFile,
    img_pointer: *mut MangoImage,
    index: usize,
) -> usize {
    let file: &mut MangoFile = unsafe {
        assert!(!file_pointer.is_null());
        &mut *file_pointer
    };

    let img: &mut MangoImage = unsafe {
        assert!(!img_pointer.is_null());
        &mut *img_pointer
    };

    let mut imgs = file.get_images();
    if index == 0 || index <= imgs.len() - 1 {
        imgs[index] = img.clone();
        file.set_images(imgs);
        return 1;
    }

    0
}

#[no_mangle]
pub extern "C" fn mangofile_remove_image(file_pointer: *mut MangoFile, index: usize) -> usize {
    let file: &mut MangoFile = unsafe {
        assert!(!file_pointer.is_null());
        &mut *file_pointer
    };

    let mut imgs = file.get_images();
    if (index == 0 && imgs.len() > 0) || index <= imgs.len() - 1 {
        imgs.remove(index);
        file.set_images(imgs);
        return 1;
    }

    0
}

#[no_mangle]
pub extern "C" fn mangofile_get_image_count(pointer: *mut MangoFile) -> usize {
    let file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    let imgs = file.get_images();
    imgs.len()
}

#[no_mangle]
pub extern "C" fn mangofile_get_meta(pointer: *mut MangoFile) -> *mut MangoMetadata {
    let file: &mut MangoFile = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };
    let meta = file.get_meta_mut();
    let p_mut: *mut MangoMetadata = meta;
    p_mut
}

// Save
#[no_mangle]
pub extern "C" fn mangofile_save(file: &mut MangoFile, path_ptr: *mut c_char) -> i16 {
    if !path_ptr.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_ptr) };

        if let Ok(value) = c_str.to_str() {
            let result = file.save(std::path::Path::new(value));
            if result.is_err() {
                return util::handle_mangofile_error(result.err().unwrap());
            }
        } else {
            return -1;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn mangofile_save_cbor(file: &mut MangoFile, path_ptr: *mut c_char) -> i16 {
    if !path_ptr.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_ptr) };
        if let Ok(value) = c_str.to_str() {
            let result = file.save_cbor(std::path::Path::new(value));
            if result.is_err() {
                return util::handle_mangofile_error(result.err().unwrap());
            }
        } else {
            return -1;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn mangofile_save_bson(file: &mut MangoFile, path_ptr: *mut c_char) -> i16 {
    if !path_ptr.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_ptr) };
        if let Ok(value) = c_str.to_str() {
            let result = file.save_bson(std::path::Path::new(value));
            if result.is_err() {
                return util::handle_mangofile_error(result.err().unwrap());
            }
        }
    } else {
        return -1;
    }

    0
}

#[no_mangle]
pub extern "C" fn mangofile_save_json(file: &mut MangoFile, path_ptr: *mut c_char) -> i16 {
    if !path_ptr.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_ptr) };
        if let Ok(value) = c_str.to_str() {
            let result = file.save_json(std::path::Path::new(value));
            if result.is_err() {
                return util::handle_mangofile_error(result.err().unwrap());
            }
        }
    } else {
        return -1;
    }

    0
}

// Open
#[no_mangle]
#[allow(unused_variables, unused_assignments)]
pub extern "C" fn mangofile_open(
    path_pointer: *mut c_char,
    error_code: *mut std::os::raw::c_int,
) -> *mut MangoFile {
    if !path_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(path_pointer) };
        if let Ok(path) = c_str.to_str() {
            let file = MangoFile::open(Path::new(path));
            if file.is_ok() {
                unsafe {
                    *error_code = 0;
                }
                return Box::into_raw(Box::new(file.unwrap()));
            } else {
                unsafe {
                    *error_code = util::handle_mangofile_error(file.err().unwrap()).into();
                }
            }
        } else {
            // set error code to -1 because something was wrong with the parameters given
            unsafe {
                *error_code = -1;
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
        Some(x) => CString::new(x).unwrap().into_raw(),
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
        Some(x) => CString::new(x).unwrap().into_raw(),
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
        Some(x) => CString::new(x).unwrap().into_raw(),
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
        Some(x) => CString::new(x).unwrap().into_raw(),
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
pub extern "C" fn mangometa_get_language(meta: &mut MangoMetadata) -> *mut c_char {
    match meta.language.clone() {
        Some(lang) => CString::new(util::from_lang(lang)).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_language(meta: &mut MangoMetadata, value_pointer: *mut c_char) {
    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {
            meta.language = util::to_lang(value);
        }
    } else {
        meta.language = None;
    }
}

#[no_mangle]
pub extern "C" fn mangometa_get_translation(pointer: *mut MangoMetadata) -> *mut c_char {
    let meta: &mut MangoMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match meta.clone().translation {
        Some(x) => CString::new(x).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_translation(
    pointer: *mut MangoMetadata,
    value_pointer: *mut c_char,
) {
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

#[no_mangle]
pub extern "C" fn mangometa_get_volume(meta: &mut MangoMetadata) -> IntOption {
    match &meta.volume {
        Some(value) => IntOption {
            value: value.clone().into(),
            present: 1, /* true */
        },
        None => IntOption {
            value: 0,
            present: 0, /* flase */
        },
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_volume(meta: &mut MangoMetadata, value_pointer: *mut c_short) {
    if !value_pointer.is_null() {
        unsafe {
            meta.volume = Some(*value_pointer);
        }
    } else {
        meta.volume = None;
    }
}

#[no_mangle]
pub extern "C" fn mangometa_get_chapter(meta: &mut MangoMetadata) -> IntOption {
    match &meta.chapter {
        Some(value) => IntOption {
            value: value.clone().into(),
            present: 1, /* true */
        },
        None => IntOption {
            value: 0,
            present: 0, /* flase */
        },
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_chapter(meta: &mut MangoMetadata, value_pointer: *mut c_short) {
    if !value_pointer.is_null() {
        unsafe {
            meta.chapter = Some(*value_pointer);
        }
    } else {
        meta.chapter = None;
    }
}

#[no_mangle]
pub extern "C" fn mangometa_get_year(meta: &mut MangoMetadata) -> IntOption {
    match &meta.year {
        Some(value) => IntOption {
            value: value.clone().into(),
            present: 1, /* true */
        },
        None => IntOption {
            value: 0,
            present: 0, /* flase */
        },
    }
}

#[no_mangle]
pub extern "C" fn mangometa_set_year(meta: &mut MangoMetadata, value_pointer: *mut c_short) {
    if !value_pointer.is_null() {
        unsafe {
            meta.year = Some(*value_pointer);
        }
    } else {
        meta.year = None;
    }
}

//----------------------------------------------------------------------------------------
// Mango Image
//----------------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn mangoimg_free(pointer: *mut MangoImage) {
    // make sure memory will only be freed once
    if pointer.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(pointer);
    }
}

#[no_mangle]
pub extern "C" fn mangoimg_from_path(
    value_pointer: *mut c_char,
    error_code: *mut std::os::raw::c_int,
) -> *mut MangoImage {
    if !value_pointer.is_null() {
        let c_str = unsafe { CStr::from_ptr(value_pointer) };
        if let Ok(value) = c_str.to_str() {
            fs::write("/tmp/foo", value).expect("Unable to write file");

            use mangofmt::ImageFile;

            let file = ImageFile::open(std::path::Path::new(value));
            if file.is_ok() {
                let img = file.unwrap().to_mango_image();
                //println!("{}", img.clone().get_meta().checksum);
                unsafe {
                    *error_code = 0;
                }
                return Box::into_raw(Box::new(img));
            } else {
                unsafe {
                    *error_code = match file.err().unwrap().kind() {
                        io::ErrorKind::NotFound => 1,
                        _ => -1,
                    };
                }
            }
        }
    }

    std::ptr::null_mut()
}

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
    let img: &mut MangoImage = unsafe {
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

    ImageData { pointer, length }
}

#[no_mangle]
pub extern "C" fn mangoimg_get_base64_image_data(pointer: *mut MangoImage) -> *mut c_char {
    let img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    CString::new(img.get_base64_image_data())
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn mangoimg_get_meta(img: &mut MangoImage) -> *mut MangoImageMetadata {
    let meta = img.get_meta_mut();
    let p_mut: *mut MangoImageMetadata = meta;
    p_mut
}

#[no_mangle]
pub extern "C" fn mangoimg_compress(pointer: *mut MangoImage, value_pointer: *mut c_char) -> i8 {
    let img: &mut MangoImage = unsafe {
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
                }
                None => {
                    return 2;
                }
            }
        }
    }

    2
}

#[no_mangle]
pub extern "C" fn mangoimg_uncompress(pointer: *mut MangoImage) -> i8 {
    let img: &mut MangoImage = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    match img.uncompress_mut() {
        true => 1,
        false => 2,
    }
}

#[no_mangle]
pub extern "C" fn mangoimg_encrypt(
    image: &mut MangoImage,
    enc_type: *const c_char,
    password: *const c_char,
) -> i8 {
    let pw = unsafe { CStr::from_ptr(password).to_str() };

    if pw.is_ok() {
        let enc_type_r = unsafe { CStr::from_ptr(enc_type).to_str() };

        if enc_type_r.is_ok() {
            let enc = util::to_enc_type(enc_type_r.unwrap().to_string());
            if enc.is_some() {
                return match image.encrypt_mut(enc.unwrap(), pw.unwrap().to_string()) {
                    true => 1,
                    false => 2,
                };
            }
        }
    }

    2
}

#[no_mangle]
pub extern "C" fn mangoimg_decrypt(image: &mut MangoImage, password: *const c_char) -> i8 {
    let pw = unsafe { CStr::from_ptr(password).to_str() };

    if pw.is_ok() {
        return match image.decrypt_mut(pw.unwrap().to_string()) {
            true => 1,
            false => 2,
        };
    }

    2
}

#[no_mangle]
pub extern "C" fn mangoimg_save(image: &MangoImage, filename: *const c_char) -> i8 {
    let name = unsafe { CStr::from_ptr(filename).to_str() };
    if name.is_ok() {
        return match image.save(&name.unwrap().to_string()) {
            Ok(()) => 0,
            Err(err) => match err.kind() {
                io::ErrorKind::PermissionDenied => 1,
                _ => -1,
            },
        };
    }

    -1
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
        Some(value) => CString::new(util::from_comp_type(value))
            .unwrap()
            .into_raw(),
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
pub extern "C" fn mangoimgmeta_checksum(meta: &mut MangoImageMetadata) -> *mut c_char {
    util::filter_nul_bytes(meta.checksum.clone()).into_raw()
}

#[no_mangle]
pub extern "C" fn mangoimgmeta_mime(pointer: *mut MangoImageMetadata) -> *mut c_char {
    let meta: &mut MangoImageMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    let mime_str = util::from_mime(meta.mime);

    CString::new(mime_str).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn mangoimgmeta_filename(pointer: *mut MangoImageMetadata) -> *mut c_char {
    let meta: &mut MangoImageMetadata = unsafe {
        assert!(!pointer.is_null());
        &mut *pointer
    };

    CString::new(meta.filename.clone()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn mangoimgmeta_iv(meta: &MangoImageMetadata) -> *const u8 {
    meta.iv
        .as_ref()
        .map_or(std::ptr::null_mut(), |iv| iv.as_ptr())
}

#[no_mangle]
pub extern "C" fn mangoimgmeta_iv_size(meta: &MangoImageMetadata) -> *const u16 {
    meta.iv
        .as_ref()
        .map_or(0 as *const u16, |iv| iv.len() as *const u16)
}
