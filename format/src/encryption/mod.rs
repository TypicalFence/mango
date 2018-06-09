use openssl;
use openssl::symm::Cipher;
use openssl::rand::rand_bytes;

use image::Base64Image;

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

pub fn encrypt(etype: EncryptionType, img: Base64Image, key: String, iv: &[u8]) -> Base64Image {
    match etype {
        EncryptionType::AES128 => {
            openssl_encrypt(etype, img, key, iv, Cipher::aes_128_cbc())
        },
        EncryptionType::AES256 => {
            openssl_encrypt(etype, img, key, iv, Cipher::aes_256_cbc())
        }
    }
}

fn openssl_encrypt(etype: EncryptionType, img: Base64Image, key: String, iv: &[u8], cipher: Cipher) -> Base64Image {
    let data = img.get_image().as_bytes();
    let encrypted_bytes = openssl::symm::encrypt(cipher, key.as_bytes(), Some(iv), data);
    let encrypted_data = String::from_utf8(encrypted_bytes.unwrap()).unwrap();
    let mut meta = img.get_meta().clone();
    meta.encryption = Some(etype);
    Base64Image::new(encrypted_data, meta)
}


pub fn decrypt(etype: EncryptionType, img: Base64Image, key: String, iv: &[u8]) -> Base64Image {
    match etype {
        EncryptionType::AES128 => {
            openssl_decrypt(img, key, iv, Cipher::aes_128_cbc())
        },
        EncryptionType::AES256 => {
            openssl_decrypt(img, key, iv, Cipher::aes_256_cbc())
        }
    }
}

fn openssl_decrypt(img: Base64Image, key: String, iv: &[u8], cipher: Cipher) -> Base64Image {
    let data = img.get_image().as_bytes();
    let decrypted_bytes = openssl::symm::decrypt(cipher, key.as_bytes(), Some(iv), data);
    let decrypted_data = String::from_utf8(decrypted_bytes.unwrap()).unwrap();
    let mut meta = img.get_meta().clone();
    meta.encryption = None;
    Base64Image::new(decrypted_data, meta)
}

fn gen_aes_iv(size: i16) -> Option<Vec<u8>> {
    if size == 128 {
        let mut iv = [0; 128];
        rand_bytes(&mut iv).unwrap();
        Some(iv.to_vec())
    } else if size == 256 {
        let mut iv = [0; 256];
        rand_bytes(&mut iv).unwrap();
        Some(iv.to_vec())
    } else {
        None
    }
}
