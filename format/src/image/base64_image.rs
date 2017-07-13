use std::io::Read;
use std::fs::File;
use base64;
use super::Mime;

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
}
