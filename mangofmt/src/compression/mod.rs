#[cfg(feature = "gzip")]
mod gzip;

use std::fmt;
use std::clone::Clone;
use image::MangoImage;
use std::error;

//------------------------------------------------------------------------------
//  Custom Error
//------------------------------------------------------------------------------
/// Errors returned by compression functions.
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
/// All "supported" types of compression.
///
/// # Important
///
/// All CompressionTypes are optional and have to be compiled in by passing a --feature flag to
/// cargo.
///
/// Each variant specifies what feature it belongs to and important implementation details, should you
/// want to decrypt/encrypt image data without the use of this crate.
///
/// You can check if the support was compiled in with the [is_supported
/// method](#method.is_supported).
#[derive(Serialize, Deserialize)]
pub enum CompressionType {
    /// **Feature:** gzip
    ///
    /// It is implemented with the flate2 crate.
    /// The C code of the flate2 crate should get compiled automatically (magically) via cargo
    /// and does not need system dependencies, from my understanding of things.
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
    #[allow(unreachable_patterns)]
    pub fn is_supported(self) -> bool {
        match self {
            #[cfg(feature = "gzip")]
            CompressionType::GZIP => true,
            _ => false,
        }
    }
}

#[allow(unreachable_patterns)]
#[allow(unused_variables)]
pub fn compress(ctype: CompressionType, image: &MangoImage) -> Result<MangoImage, CompressionError> {
    match ctype {
        #[cfg(feature = "gzip")]
        CompressionType::GZIP => Ok(gzip::compress(image)),
        _ => Err(CompressionError::UnsupportedType),
    }
}

#[allow(unreachable_patterns)]
#[allow(unused_variables)]
pub fn uncompress(ctype: CompressionType, image: &MangoImage) -> Result<MangoImage, CompressionError> {
    match ctype {
        #[cfg(feature = "gzip")]
        CompressionType::GZIP => gzip::uncompress(image),
        _ => Err(CompressionError::UnsupportedType),
    }
}

