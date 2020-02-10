#[cfg(feature = "aes")]
mod openssl_mods;
#[cfg(feature = "aes")]
mod tiger;

use image::MangoImage;
use std::error;
use std::fmt;

//------------------------------------------------------------------------------
//  Custom Error
//------------------------------------------------------------------------------
/// Errors returned by encryption functions.
#[derive(Debug, Clone)]
pub enum EncryptionError {
    UnsupportedType,
    ExecutionError,
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "oh no something went wrong with the encryption")
    }
}

impl error::Error for EncryptionError {
    fn description(&self) -> &str {
        match self {
            EncryptionError::UnsupportedType => "The Encryption Type is not supported",
            EncryptionError::ExecutionError => "while en/decrypting a error occurred",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

//------------------------------------------------------------------------------
//  Encryption Types
//------------------------------------------------------------------------------
/// All "supported" types of encryptions.
///
/// # Important
///
/// All EncryptionTypes are optional and have to be compiled in by passing a --feature flag to
/// cargo.
///
/// Each variant specifies what feature it belongs to and important implementation details, should you
/// want to decrypt/encrypt image data without the use of this crate.
///
/// You can check if the support was compiled in with the [is_supported
/// method](#method.is_supported).
#[derive(Serialize, Deserialize)]
pub enum EncryptionType {
    /// **Feature:** aes
    ///
    /// It requires openssl to be installed on the system.
    ///
    /// The key will be hashed with a Tiger/128 hash, which consists of the first 128 bits of a
    /// Tiger/192 hash, check
    /// [this](http://www.cs.technion.ac.il/~biham/Reports/Tiger/tiger/node2.html)
    /// out for information.
    AES128,
    /// **Feature:** aes
    ///
    /// It requires openssl to be installed on the system.
    ///
    /// The key will be hashed with a SHA256 hash.
    AES256,
}

impl Clone for EncryptionType {
    fn clone(&self) -> EncryptionType {
        match self {
            EncryptionType::AES128 => EncryptionType::AES128,
            EncryptionType::AES256 => EncryptionType::AES256,
        }
    }
}

impl fmt::Display for EncryptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncryptionType::AES128 => write!(f, "AES128"),
            EncryptionType::AES256 => write!(f, "AES256"),
        }
    }
}

impl EncryptionType {
    /// Returns whether the EncryptionType got compiled in or not.
    #[allow(unreachable_patterns)]
    pub fn is_supported(&self) -> bool {
        match self {
            #[cfg(feature = "aes")]
            EncryptionType::AES128 => true,
            #[cfg(feature = "aes")]
            EncryptionType::AES256 => true,
            _ => false,
        }
    }
}

#[allow(unreachable_patterns)]
#[allow(unused_variables)]
pub fn encrypt(
    etype: EncryptionType,
    img: MangoImage,
    key: String,
) -> Result<MangoImage, EncryptionError> {
    match etype {
        #[cfg(feature = "aes")]
        EncryptionType::AES128 => Ok(openssl_mods::aes::encrypt_aes128(img, key)),
        #[cfg(feature = "aes")]
        EncryptionType::AES256 => Ok(openssl_mods::aes::encrypt_aes256(img, key)),
        _ => Err(EncryptionError::UnsupportedType),
    }
}

#[allow(unreachable_patterns)]
#[allow(unused_variables)]
pub fn decrypt(
    etype: EncryptionType,
    img: MangoImage,
    key: String,
    iv: &[u8],
) -> Result<MangoImage, EncryptionError> {
    match etype {
        #[cfg(feature = "aes")]
        EncryptionType::AES128 => Ok(openssl_mods::aes::decrypt_aes128(img, key, iv)),
        #[cfg(feature = "aes")]
        EncryptionType::AES256 => Ok(openssl_mods::aes::decrypt_aes256(img, key, iv)),
        _ => Err(EncryptionError::UnsupportedType),
    }
}
