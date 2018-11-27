#[cfg(feature = "gzip")]
mod gzip;

use std::fmt;
use std::clone::Clone;
use MangoImage;

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

impl fmt::Display for CompressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompressionType::GZIP => write!(f, "GZIP"),
        }
    }
}

impl CompressionType {
    pub fn is_supported(self) -> bool {
        match self {
            #[cfg(feature = "gzip")]
            CompressionType::GZIP => true,
            _ => false,
        }
    }
}

pub fn compress(ctype: CompressionType, image: &MangoImage) -> MangoImage {
    match ctype {
        #[cfg(feature = "gzip")]
        CompressionType::GZIP => gzip::compress(image),
        _ => image.clone(),
    }
}

pub fn uncompress(ctype: CompressionType, image: &MangoImage) -> MangoImage {
    match ctype {
        #[cfg(feature = "gzip")]
        CompressionType::GZIP => gzip::uncompress(image),
        _ => image.clone(),
    }
}

