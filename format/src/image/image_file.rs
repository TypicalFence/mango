use std::io::{Error, ErrorKind, Read};
use std::fs::File;
use std::path::Path;
use super::Base64Image;
use meta::ImageFileMetadata;

/// A struct that represents an image file.
///
/// In most cases you should not need to use this struct yourself.
/// I exposed it to the outside just in case.
pub struct ImageFile {
    file: File,
    meta: ImageFileMetadata,
}

impl ImageFile {
    /// Returns a new Instance of the Struct.
    ///
    /// The new instance is based on a file from the filesystem.
    pub fn open(p: &Path) -> Result<ImageFile, Error> {
        if p.is_file() {
            match File::open(&p) {
                Ok(file) => {
                    match ImageFileMetadata::new(&p) {
                        Some(meta) => Ok(ImageFile { file, meta }),
                        None => Err(Error::new(ErrorKind::Other, "couldn't read metadata")),
                    }
                }
                Err(e) => Err(e),
            }
        } else {
           Err(Error::new(ErrorKind::InvalidInput, "path is not a file"))
        }
    }

    /// Returns an instance of the internal File.
    pub fn get_file(&mut self) -> &mut File {
        self.file.by_ref()
    }

    /// Returns a copy of the metadata of the file.
    pub fn get_meta(&self) -> ImageFileMetadata {
        self.meta.clone()
    }

    /// Converts the ImageFile struct to a Base64Image struct
    ///
    /// The file itself stays untouched it just converts the data to what
    /// is used inside .mango files.
    pub fn to_base64(&mut self) -> Base64Image {
        Base64Image::from_file(self)
    }
}
