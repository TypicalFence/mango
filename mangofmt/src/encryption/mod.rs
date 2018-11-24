#[cfg(feature = "aes")]
mod tiger;
#[cfg(feature = "aes")]
mod openssl_mods;

use std::fmt;
use MangoImage;

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

pub fn encrypt(etype: EncryptionType, img: MangoImage, key: String) -> MangoImage {
    match etype {
        #[cfg(feature = "aes")]
        EncryptionType::AES128 => openssl_mods::aes::encrypt_aes128(img, key),
        #[cfg(feature = "aes")]
        EncryptionType::AES256 => openssl_mods::aes::encrypt_aes256(img, key),
        _ => img,
    }
}


pub fn decrypt(etype: EncryptionType, img: MangoImage, key: String, iv: &[u8]) -> MangoImage {
    match etype {
        #[cfg(feature = "aes")]
        EncryptionType::AES128 => openssl_mods::aes::decrypt_aes128(img, key, iv),
        #[cfg(feature = "aes")]
        EncryptionType::AES256 => openssl_mods::aes::decrypt_aes256(img, key, iv),
        _ => img,
    }
}




