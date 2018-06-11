use base64;
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

pub fn encrypt(etype: EncryptionType, img: Base64Image, key: String) -> Base64Image {
    match etype {
        EncryptionType::AES128 => openssl_encrypt(etype, img, key, Cipher::aes_128_cbc()),
        EncryptionType::AES256 => openssl_encrypt(etype, img, key, Cipher::aes_256_cbc()),
    }
}


pub fn decrypt(etype: EncryptionType, img: Base64Image, key: String, iv: &[u8]) -> Base64Image {
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

fn openssl_encrypt(
    etype: EncryptionType,
    img: Base64Image,
    key: String,
    cipher: Cipher,
) -> Base64Image {
    let image_data = base64::decode(&img.get_image()).unwrap();
    let iv = gen_iv(cipher);
    let encrypted_bytes = openssl::symm::encrypt(cipher, key.as_bytes(), Some(&iv), &image_data);
    let encrypted_data = base64::encode(&encrypted_bytes.unwrap());
    let mut meta = img.get_meta().clone();
    meta.encryption = Some(etype);
    meta.iv = Some(iv);
    Base64Image::new(encrypted_data, meta)
}

fn openssl_decrypt(img: Base64Image, key: String, iv: &[u8], cipher: Cipher) -> Base64Image {
    let image_data = base64::decode(&img.get_image()).unwrap();
    let decrypted_bytes = openssl::symm::decrypt(cipher, key.as_bytes(), Some(iv), &image_data);
    let decrypted_data = base64::encode(&decrypted_bytes.unwrap());
    let mut meta = img.get_meta().clone();
    meta.encryption = None;
    meta.iv = None;
    Base64Image::new(decrypted_data, meta)
}
