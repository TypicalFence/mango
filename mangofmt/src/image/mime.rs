use std::io::{Error, ErrorKind};
use std::path::Path;
use std::clone::Clone;

/// Contains all supported image file formats.
#[derive(Copy, Serialize, Deserialize)]
pub enum Mime {
    PNG,
    JPEG,
}

impl Mime {
    fn path_string_to_mime(path: &str) -> Option<Mime> {
        if path.ends_with("png") {
            Some(Mime::PNG)
        } else if path.ends_with("jpg") {
            Some(Mime::JPEG)
        } else if path.ends_with("jpeg") {
            Some(Mime::JPEG)
        } else {
            None
        }
    }

    /// Determines the mimetype from the file extension of a path
    pub fn get_from_path(p: &Path) -> Result<Mime, Error> {
        if p.is_file() {
            match p.to_str() {
                Some(path_str) => {
                    match Mime::path_string_to_mime(path_str) {
                        Some(mime) => Ok(mime),
                        None => Err(Error::new(
                            ErrorKind::InvalidInput,
                            "file format is not supported",
                        )),
                    }
                }
                None => Err(Error::new(
                    ErrorKind::Other,
                    "can't convert your path to string",
                )),
            }
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "path is not a file"))
        }
    }
}

impl Clone for Mime {
    fn clone(&self) -> Mime {
        *self
    }
}
