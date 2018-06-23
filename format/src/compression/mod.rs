use std::io::{Write, Read};
use std::clone::Clone;
use base64;
use flate2;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use image::MangoImage;

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


pub fn compress(ctype: CompressionType, image: &MangoImage) -> MangoImage {
    match ctype {
        CompressionType::GZIP => gzip_compress(image)
    }
}

pub fn uncompress(ctype: CompressionType, image: &MangoImage) -> MangoImage {
    match ctype {
        CompressionType::GZIP => gzip_uncompress(image)
    }
}

fn gzip_compress(image: &MangoImage) -> MangoImage {
    let image_vec = &image.get_image_data();
    let mut e = GzEncoder::new(Vec::new(), flate2::Compression::Best);
    e.write_all(&image_vec).unwrap();
    let compressed = e.finish().unwrap();

    let mut new_meta = image.get_meta();
    new_meta.compression = Some(CompressionType::GZIP);

    MangoImage::new(compressed, new_meta)
}

fn gzip_uncompress(image: &MangoImage) -> MangoImage {
    let image_data = &image.get_image_data();
    let mut decoder = GzDecoder::new(image_data.as_slice()).unwrap();
    let mut raw_data = Vec::new();
    decoder.read_to_end(&mut raw_data);

    let mut new_meta = image.get_meta();
    new_meta.compression = None;

    MangoImage::new(raw_data, new_meta)
}
