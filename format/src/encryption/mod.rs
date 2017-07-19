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
    unimplemented!()
}

pub fn decrypt(etype: EncryptionType, img: Base64Image, key: String) -> Base64Image {
    unimplemented!()
}
