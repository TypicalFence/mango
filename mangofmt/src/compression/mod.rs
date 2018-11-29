#[cfg(feature = "gzip")]
mod gzip;

use std::fmt;
use std::clone::Clone;
use MangoImage;
use std::error;

//------------------------------------------------------------------------------
//  Custom Error
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum CompressionError {
    UnsupportedType,
    ExecutionError
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "oh no something went wrong with the encryption")
    }
}

impl error::Error for CompressionError {
    fn description(&self) -> &str {
        match self {
            CompressionError::UnsupportedType => "The Compression Type is not supported",
            CompressionError::ExecutionError => "while (de)compressing a error occurred",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

//------------------------------------------------------------------------------
//  Compression Types
//------------------------------------------------------------------------------
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

pub fn compress(ctype: CompressionType, image: &MangoImage) -> Result<MangoImage, CompressionError> {
    match ctype {
        #[cfg(feature = "gzip")]
        CompressionType::GZIP => Ok(gzip::compress(image)),
        _ => Err(CompressionError::UnsupportedType),
    }
}

pub fn uncompress(ctype: CompressionType, image: &MangoImage) -> Result<MangoImage, CompressionError> {
    match ctype {
        #[cfg(feature = "gzip")]
        CompressionType::GZIP => Ok(gzip::uncompress(image)),
        _ => Err(CompressionError::UnsupportedType),
    }
}

