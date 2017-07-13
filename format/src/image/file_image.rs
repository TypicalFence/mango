use std::io::{Error, ErrorKind};
use std::fs::File;
use std::path::Path;
use super::Mime;
use super::Base64Image;
use std::io::Read;

pub struct FileImage {
    file: File,
    mime: Mime,
}

impl FileImage {
    pub fn open(p: &Path) -> Result<FileImage, Error> {
        if p.is_file() {
            match File::open(&p) {
                Ok(file) => {
                    match Mime::get_from_path(p) {
                        Ok(mime) => Ok((FileImage { file, mime })),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "path is not a file"))
        }
    }

    pub fn to_base64(&mut self) -> Base64Image {
        Base64Image::from_file(self.file.by_ref(), self.mime.clone())
    }
}
