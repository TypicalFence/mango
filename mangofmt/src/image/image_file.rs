use image::MangoImage;
use meta::ImageFileMetadata;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// Represents an image file.
///
/// In most cases you should not need to use this struct yourself.
/// Its exposed to the outside just in case.
///
/// It can mainly opens a File and generates some meta data.
pub struct ImageFile {
    path: PathBuf,
    meta: ImageFileMetadata,
}

impl ImageFile {
    /// Returns a new Instance of the struct.
    ///
    /// The new instance is based on a file from the file system.
    pub fn open(path: &Path) -> Result<ImageFile, Error> {
        if path.is_file() {
            match ImageFileMetadata::new(&path) {
                Some(meta) => Ok(ImageFile {
                    path: path.to_path_buf(),
                    meta,
                }),
                None => Err(Error::new(ErrorKind::Other, "couldn't read metadata")),
            }
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "path is not a file"))
        }
    }

    /// Returns an instance of the internal Path to the File.
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns a copy of the metadata of the file.
    pub fn get_meta(&self) -> ImageFileMetadata {
        // TODO why not just return a reference and let the user clone the struct?
        self.meta.clone()
    }

    /// Converts the ImageFile struct to a MangoImage struct
    ///
    /// The file itself stays untouched it just converts the data to what
    /// is used inside .mango files.
    pub fn to_mango_image(&self) -> MangoImage {
        MangoImage::from_file(self)
    }
}
