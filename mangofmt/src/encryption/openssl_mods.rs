extern crate openssl;

use self::openssl::rand::rand_bytes;
use self::openssl::symm::Cipher;
use super::tiger::tiger_128;
use encryption::EncryptionType;
use image::MangoImage;
use sha2::{Digest, Sha256};

fn openssl_hash(key: String, cipher: Cipher) -> Vec<u8> {
    // key_len is the length in bytes
    match cipher.key_len() {
        16 => tiger_128(key),
        32 => {
            let mut hasher = Sha256::default();
            hasher.input(&key.as_bytes());
            return hasher.result().to_vec();
        }
        _ => Vec::default(),
    }
}

fn gen_iv(cipher: Cipher) -> Vec<u8> {
    let mut iv = vec![0; cipher.iv_len().unwrap()];
    rand_bytes(&mut iv).unwrap();
    iv
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
    let encrypted_bytes = openssl::symm::encrypt(
        cipher,
        openssl_hash(key, cipher).as_ref(),
        Some(&iv),
        &image_data,
    );
    let encrypted_data = encrypted_bytes.unwrap();
    let mut meta = img.get_meta().clone();
    meta.encryption = Some(etype);
    meta.iv = Some(iv);
    MangoImage::new(encrypted_data, meta)
}

#[cfg(feature = "aes")]
fn openssl_decrypt(img: MangoImage, key: String, iv: &[u8], cipher: Cipher) -> MangoImage {
    let image_data: Vec<u8> = img.get_image_data();
    let decrypted_bytes = openssl::symm::decrypt(
        cipher,
        openssl_hash(key, cipher).as_ref(),
        Some(iv),
        &image_data,
    );
    let decrypted_data = decrypted_bytes.unwrap();
    let mut meta = img.get_meta().clone();
    meta.encryption = None;
    meta.iv = None;
    MangoImage::new(decrypted_data, meta)
}

#[cfg(feature = "aes")]
pub mod aes {
    use super::Cipher;
    use super::{openssl_decrypt, openssl_encrypt};
    use super::{EncryptionType, MangoImage};

    pub fn encrypt_aes128(img: MangoImage, key: String) -> MangoImage {
        openssl_encrypt(EncryptionType::AES128, img, key, Cipher::aes_128_cbc())
    }

    pub fn encrypt_aes256(img: MangoImage, key: String) -> MangoImage {
        openssl_encrypt(EncryptionType::AES256, img, key, Cipher::aes_256_cbc())
    }

    pub fn decrypt_aes128(img: MangoImage, key: String, iv: &[u8]) -> MangoImage {
        openssl_decrypt(img, key, iv, Cipher::aes_128_cbc())
    }

    pub fn decrypt_aes256(img: MangoImage, key: String, iv: &[u8]) -> MangoImage {
        openssl_decrypt(img, key, iv, Cipher::aes_128_cbc())
    }
}
