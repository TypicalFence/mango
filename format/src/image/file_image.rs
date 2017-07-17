use std::io::{Error, ErrorKind, Read};
use std::fs::File;
use std::path::Path;
use super::Base64Image;
use meta::FileImageMetadata;

pub struct FileImage {
    file: File,
    meta: FileImageMetadata,
}

impl FileImage {
    pub fn open(p: &Path) -> Result<FileImage, Error> {
        if p.is_file() {
            match File::open(&p) {
                Ok(file) => {
                    match FileImageMetadata::new(&p) {
                        Some(meta) => Ok(FileImage { file, meta }),
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

    pub fn get_meta(&self) -> FileImageMetadata {
        self.meta.clone()
    }

    pub fn to_base64(&mut self) -> Base64Image {
        Base64Image::from_file(self)
    }
}
