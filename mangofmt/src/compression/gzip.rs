extern crate flate2;

use self::flate2::read::GzDecoder;
use self::flate2::write::GzEncoder;
use super::CompressionError;
use super::CompressionType;
use image::MangoImage;
use std::io::{Read, Write};

pub fn compress(image: &MangoImage) -> MangoImage {
    let image_vec = &image.get_image_data();
    let mut e = GzEncoder::new(Vec::new(), flate2::Compression::Best);
    e.write_all(&image_vec).unwrap();
    let compressed = e.finish().unwrap();

    let mut new_meta = image.get_meta();
    new_meta.compression = Some(CompressionType::GZIP);

    MangoImage::new(compressed, new_meta)
}

pub fn uncompress(image: &MangoImage) -> Result<MangoImage, CompressionError> {
    let image_data = &image.get_image_data();
    if let Ok(mut decoder) = GzDecoder::new(image_data.as_slice()) {
        let mut raw_data = Vec::new();

        if decoder.read_to_end(&mut raw_data).is_err() {
            return Err(CompressionError::ExecutionError);
        };

        let mut new_meta = image.get_meta();
        new_meta.compression = None;

        Ok(MangoImage::new(raw_data, new_meta))
    } else {
        return Err(CompressionError::ExecutionError);
    }
}
