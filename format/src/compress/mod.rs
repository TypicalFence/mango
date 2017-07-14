use base64;
use flate2;
use flate2::write::GzEncoder;
use image::Base64Image;
use std::io::Write;

pub trait Compression {
    fn compress(&self, image: &Base64Image) -> Base64Image;
}

pub enum CompressionType {
    GZIP,
}

pub struct Gz {}

impl Compression for Gz {
    fn compress(&self, image: &Base64Image) -> Base64Image {
        let image_vec = base64::decode(&image.get_image()).unwrap();
        let mut e = GzEncoder::new(Vec::new(), flate2::Compression::Best);
        e.write(&image_vec).unwrap();
        let compressed = e.finish().unwrap();
        let mut muh_base64 = base64::encode(&compressed);
        muh_base64 = muh_base64.replace("\r\n", "");

        Base64Image::new(muh_base64, image.get_mime())
    }
}

impl Gz {
    pub fn new() -> Gz {
        Gz {}
    }
}
