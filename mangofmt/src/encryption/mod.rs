#[cfg(feature = "aes")]
mod tiger;
#[cfg(feature = "aes")]
mod openssl_mods;

use std::fmt;
use MangoImage;
use std::error;

//------------------------------------------------------------------------------
//  Custom Error
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum EncryptionError {
    UnsupportedType,
    ExecutionError
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

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

//------------------------------------------------------------------------------
//  Encryption Types
//------------------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
pub enum EncryptionType {
    AES256,
    AES128,
}

impl Clone for EncryptionType {
    fn clone(&self) -> EncryptionType {
        match self {
            &EncryptionType::AES128 => EncryptionType::AES128,
            &EncryptionType::AES256 => EncryptionType::AES256,
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

    #[allow(unreachable_patterns)]
    pub fn is_supported(self) -> bool {
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
pub fn encrypt(etype: EncryptionType, img: MangoImage, key: String) -> Result<MangoImage, EncryptionError> {
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
pub fn decrypt(etype: EncryptionType, img: MangoImage, key: String, iv: &[u8]) -> Result<MangoImage, EncryptionError> {
    match etype {
        #[cfg(feature = "aes")]
        EncryptionType::AES128 => Ok(openssl_mods::aes::decrypt_aes128(img, key, iv)),
        #[cfg(feature = "aes")]
        EncryptionType::AES256 => Ok(openssl_mods::aes::decrypt_aes256(img, key, iv)),
        _ => Err(EncryptionError::UnsupportedType),
    }
}


