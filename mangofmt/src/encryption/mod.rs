mod tiger;

use std::fmt;
use openssl;
use openssl::symm::Cipher;
use openssl::rand::rand_bytes;
use sha2::{Sha256, Digest};
use image::MangoImage;
use self::tiger::tiger_128;

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
        EncryptionType::AES128 => openssl_encrypt(etype, img, key, Cipher::aes_128_cbc()),
        EncryptionType::AES256 => openssl_encrypt(etype, img, key, Cipher::aes_256_cbc()),
    }
}


pub fn decrypt(etype: EncryptionType, img: MangoImage, key: String, iv: &[u8]) -> MangoImage {
    match etype {
        EncryptionType::AES128 => openssl_decrypt(img, key, iv, Cipher::aes_128_cbc()),
        EncryptionType::AES256 => openssl_decrypt(img, key, iv, Cipher::aes_256_cbc()),
    }
}

pub fn gen_iv(cipher: Cipher) -> Vec<u8> {
    let mut iv = vec![0; cipher.iv_len().unwrap()];
    rand_bytes(&mut iv).unwrap();
    iv
}

fn openssl_hash(key: String, cipher: Cipher) -> Vec<u8> {
    // key_len is the length in bytes
    match cipher.key_len() {
        16 => tiger_128(key),
        32 => {
            let mut hasher = Sha256::default();
            hasher.input(&key.as_bytes());
            return hasher.result().to_vec();
        },
        _ => Vec::default(),
    }
}

fn openssl_encrypt(
    etype: EncryptionType,
    img: MangoImage,
    key: String,
    cipher: Cipher,
) -> MangoImage {
    println!("{}", cipher.key_len());
    let image_data: Vec<u8> = img.get_image_data();
    let iv = gen_iv(cipher);
    let encrypted_bytes = openssl::symm::encrypt(cipher, openssl_hash(key, cipher).as_ref(), Some(&iv), &image_data);
    let encrypted_data = encrypted_bytes.unwrap();
    let mut meta = img.get_meta().clone();
    meta.encryption = Some(etype);
    meta.iv = Some(iv);
    MangoImage::new(encrypted_data, meta)
}

fn openssl_decrypt(img: MangoImage, key: String, iv: &[u8], cipher: Cipher) -> MangoImage {
    let image_data: Vec<u8> = img.get_image_data();
    let decrypted_bytes = openssl::symm::decrypt(cipher, openssl_hash(key, cipher).as_ref(), Some(iv), &image_data);
    let decrypted_data = decrypted_bytes.unwrap();
    let mut meta = img.get_meta().clone();
    meta.encryption = None;
    meta.iv = None;
    MangoImage::new(decrypted_data, meta)
}
