use std::io::{Error, ErrorKind, Read};
use std::fs::File;
use std::path::Path;
use super::Base64Image;
use meta::ImageFileMetadata;

pub struct ImageFile {
    file: File,
    meta: ImageFileMetadata,
}

impl ImageFile {
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

    pub fn get_file(&mut self) -> &mut File {
        self.file.by_ref()
    }

    pub fn get_meta(&self) -> ImageFileMetadata {
        self.meta.clone()
    }

    pub fn to_base64(&mut self) -> Base64Image {
        Base64Image::from_file(self)
    }
}
