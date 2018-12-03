use std::io::{Error, ErrorKind, Read};
use std::fs::File;
use std::path::Path;
use image::MangoImage;
use meta::ImageFileMetadata;

/// Represents an image file.
///
/// In most cases you should not need to use this struct yourself.
/// Its exposed to the outside just in case.
///
/// It can mainly opens a File and generates some meta data.
pub struct ImageFile {
    file: File,
    meta: ImageFileMetadata,
}

impl ImageFile {
    /// Returns a new Instance of the struct.
    ///
    /// The new instance is based on a file from the file system.
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

    /// Converts the ImageFile struct to a MangoImage struct
    ///
    /// The file itself stays untouched it just converts the data to what
    /// is used inside .mango files.
    pub fn to_mango_image(&mut self) -> MangoImage {
        MangoImage::from_file(self)
    }
}
