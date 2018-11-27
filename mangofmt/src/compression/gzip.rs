extern crate flate2;

use std::io::{Write, Read};
use self::flate2::write::GzEncoder;
use self::flate2::read::GzDecoder;
use image::MangoImage;
use super::CompressionType;

pub fn compress(image: &MangoImage) -> MangoImage {
    let image_vec = &image.get_image_data();
    let mut e = GzEncoder::new(Vec::new(), flate2::Compression::Best);
    e.write_all(&image_vec).unwrap();
    let compressed = e.finish().unwrap();

    let mut new_meta = image.get_meta();
    new_meta.compression = Some(CompressionType::GZIP);

    MangoImage::new(compressed, new_meta)
}

pub fn uncompress(image: &MangoImage) -> MangoImage {
    let image_data = &image.get_image_data();
    let mut decoder = GzDecoder::new(image_data.as_slice()).unwrap();
    let mut raw_data = Vec::new();
    decoder.read_to_end(&mut raw_data);

    let mut new_meta = image.get_meta();
    new_meta.compression = None;

    MangoImage::new(raw_data, new_meta)
}
