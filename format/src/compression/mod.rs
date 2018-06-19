use std::io::{Write, Read};
use std::clone::Clone;
use base64;
use flate2;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use image::Base64Image;

#[derive(Serialize, Deserialize)]
pub enum CompressionType {
    GZIP,
}

impl Clone for CompressionType {
    fn clone(&self) -> CompressionType {
        match self {
            &CompressionType::GZIP => CompressionType::GZIP,
        }
    }
}


pub fn compress(ctype: CompressionType, image: &Base64Image) -> Base64Image {
    match ctype {
        CompressionType::GZIP => gzip_compress(image)
    }
}

pub fn uncompress(ctype: CompressionType, image: &Base64Image) -> Base64Image {
    match ctype {
        CompressionType::GZIP => gzip_uncompress(image)
    }
}

fn gzip_compress(image: &Base64Image) -> Base64Image {
    let image_vec = base64::decode(&image.get_image_data()).unwrap();
    let mut e = GzEncoder::new(Vec::new(), flate2::Compression::Best);
    e.write(&image_vec).unwrap();
    let compressed = e.finish().unwrap();
    let mut muh_base64 = base64::encode(&compressed);
    muh_base64 = muh_base64.replace("\r\n", "");

    let mut new_meta = image.get_meta();
    new_meta.compression = Some(CompressionType::GZIP);

    Base64Image::new(muh_base64, new_meta)
}

fn gzip_uncompress(image: &Base64Image) -> Base64Image {
    let image_data = base64::decode(&image.get_image_data()).unwrap();
    let mut decoder = GzDecoder::new(image_data.as_slice()).unwrap();
    let mut raw_data = Vec::new();
    decoder.read_to_end(&mut raw_data);
    let mut muh_base64 = base64::encode(&raw_data);
    muh_base64 = muh_base64.replace("\r\n", "");

    let mut new_meta = image.get_meta();
    new_meta.compression = None;

    Base64Image::new(muh_base64, new_meta)
}
