use std;
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use base64;
use super::ImageFile;
use meta::Base64ImageMetadata;
use compression;
use compression::CompressionType;
use encryption;
use encryption::EncryptionType;


#[derive(Serialize, Deserialize, Clone)]
pub struct Base64Image {
    base64: String,
    meta: Base64ImageMetadata,
}

impl Base64Image {
    pub fn new(base64: String, meta: Base64ImageMetadata) -> Base64Image {
        Base64Image { base64, meta }
    }

    pub fn from_file(file_image: &mut ImageFile) -> Base64Image {
        let mut vec = Vec::new();
        let _ = file_image.get_file().read_to_end(&mut vec);
        let muh_base64 = base64::encode(&vec);
        let new_meta = file_image.get_meta();
        Base64Image::new(
            muh_base64.replace("\r\n", ""),
            new_meta.to_base64_metadata(),
        )
    }

    pub fn get_meta(&self) -> Base64ImageMetadata {
        self.meta.clone()
    }

    pub fn get_image_data(&self) -> String {
        self.base64.clone()
    }

    pub fn compress(&self, comp: CompressionType) -> Option<Base64Image> {
        if self.meta.encryption.is_none()  && self.meta.compression.is_none() {
            return Some(compression::compress(comp, self));
        }

        None
    }

    pub fn compress_mut(&mut self, comp: CompressionType) -> bool {
        let compressed_opt = self.clone().compress(comp);
        if compressed_opt.is_some() {
            let compressed_img = compressed_opt.unwrap();
            self.base64 = compressed_img.get_image_data();
            self.meta = compressed_img.get_meta();
            true
        } else {
            false
        }
    }

    pub fn uncompress(&self) -> Option<Base64Image> {
        let meta = &self.meta;

        if meta.compression.is_some() && meta.encryption.is_none() {
            let comp = meta.clone().compression.unwrap();
            return Some(compression::uncompress(comp, self));
        }

        None
    }

    pub fn uncompress_mut(&mut self) -> bool {
        let uncompressed_opt = self.clone().uncompress();
        if uncompressed_opt.is_some() {
            let uncompressed_img = uncompressed_opt.unwrap();
            self.base64 = uncompressed_img.get_image_data();
            self.meta = uncompressed_img.get_meta();
            true
        } else {
            false
        }
    }

    pub fn encrypt(self, etype: EncryptionType, key: String) -> Option<Base64Image> {
        if self.meta.encryption.is_none() {
            return Some(encryption::encrypt(etype, self, key));
        }

        None
    }

    pub fn encrypt_mut(&mut self, etype: EncryptionType, key: String) -> bool {
        let encrypted_opt = self.clone().encrypt(etype, key);
        if encrypted_opt.is_some() {
            let encrypted_img = encrypted_opt.unwrap();
            self.base64 = encrypted_img.get_image_data();
            self.meta = encrypted_img.get_meta();
            true
        } else {
            false
        }
    }

    pub fn decrypt(self, key: String) -> Option<Base64Image> {
        if self.meta.encryption.is_some() && self.meta.iv.is_some() {
            let iv = self.meta.iv.clone().unwrap();
            let etype = self.meta.encryption.clone().unwrap();
            return Some(encryption::decrypt(etype, self, key, &iv));
        }

        None
    }

    pub fn decrypt_mut(&mut self, key: String) -> bool {
        let decrypted_opt = self.clone().decrypt(key);
        if decrypted_opt.is_some() {
            let decrypted_img = decrypted_opt.unwrap();
            self.base64 = decrypted_img.get_image_data();
            self.meta = decrypted_img.get_meta();
            true
        } else {
            false
        }
    }

    pub fn save(&self, file_name: &str) -> std::io::Result<()> {
        let data = base64::decode(&self.base64).unwrap();
        let mut file = File::create(file_name)?;
        file.write_all(&data)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std;
    use super::{Base64Image, ImageFile, EncryptionType, CompressionType};

    #[test]
    fn mut_crypt() {
        let p = std::path::Path::new("test.jpg");
        let mut file = ImageFile::open(p).unwrap();

        let mut img = Base64Image::from_file(&mut file);
        let clean_data = img.get_image_data();

        img.encrypt_mut(EncryptionType::AES128, String::from("1234567812345678"));
        assert_eq!(img.get_meta().encryption.is_some(), true);
        assert_ne!(img.get_image_data(), clean_data);
        img.decrypt_mut(String::from("1234567812345678"));
        assert_eq!(img.get_meta().encryption.is_none(), true);
        assert_eq!(img.get_image_data(), clean_data);
    }

    #[test]
    fn mut_compress() {
        let p = std::path::Path::new("test.jpg");
        let mut file = ImageFile::open(p).unwrap();

        let mut img = Base64Image::from_file(&mut file);
        let clean_data = img.get_image_data();

        img.compress_mut(CompressionType::GZIP);
        assert_eq!(img.get_meta().compression.is_some(), true);
        assert_ne!(img.get_image_data(), clean_data);
        img.uncompress_mut();
        img.save("lol.jpg");
        assert_eq!(img.get_meta().compression.is_none(), true);
        assert_eq!(img.get_image_data(), clean_data);
    }
}
