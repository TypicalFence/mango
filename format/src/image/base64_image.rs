use std::io::Read;
use std::fs::File;
use base64;
use super::Mime;
use compress::Compression;

pub struct Base64Image {
    base64: String,
    mime: Mime,
}

impl Base64Image {
    pub fn new(base64: String, mime: Mime) -> Base64Image {
        Base64Image { base64, mime }
    }
    pub fn from_file(file: &mut File, m: Mime) -> Base64Image {
        let mut vec = Vec::new();
        let _ = file.read_to_end(&mut vec);
        let muh_base64 = base64::encode(&vec);
        Base64Image::new(muh_base64.replace("\r\n", ""), m)
    }
    pub fn get_mime(&self) -> Mime {
        self.mime.clone()
    }
    pub fn get_image(&self) -> String {
        self.base64.clone()
    }
    pub fn compress(&self, comp: &Compression) -> Base64Image {
        comp.compress(self)
    }
}
