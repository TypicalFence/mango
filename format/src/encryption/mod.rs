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

pub fn encrypt(etype: EncryptionType, img: Base64Image, key: String, iv: &[u8]) -> Option<Base64Image> {
    match etype {
        EncryptionType::AES128 => {
            let data = img.get_image().as_bytes();
            let encrypted_bytes = openssl::symm::encrypt(Cipher::aes_128_cbc(), key.as_bytes(), Some(gen_aes_iv_128()), data);
            let encrypted_data = String::from_utf8(encrypted_bytes.unwrap()).unwrap();
            let mut meta = img.get_meta().clone();
            meta.encryption = Some(EncryptionType::AES128);
            Some(Base64Image::new(encrypted_data, meta))
        },
        _ => None
    }
}

pub fn decrypt(etype: EncryptionType, img: Base64Image, key: String, iv: &[u8]) -> Base64Image {
    unimplemented!()
}

fn gen_aes_iv_128() -> [u8] {
    let mut iv = [0; 128];
    rand_bytes(&mut iv).unwrap();
    iv
}
