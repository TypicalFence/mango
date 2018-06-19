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

    pub fn uncompress(&self, comp: CompressionType) -> Option<Base64Image> {
        let meta = &self.meta;

        if meta.compression.is_some() && meta.encryption.is_none() {
            return Some(compression::uncompress(comp, self));
        }

        None
    }

    pub fn encrypt(self, etype: EncryptionType, key: String) -> Option<Base64Image> {
        if self.meta.encryption.is_none() {
            return Some(encryption::encrypt(etype, self, key));
        }

        None
    }

    pub fn decrypt(self, key: String) -> Option<Base64Image> {
        if self.meta.encryption.is_some() && self.meta.iv.is_some() {
            let iv = self.meta.iv.clone().unwrap();
            let etype = self.meta.encryption.clone().unwrap();
            return Some(encryption::decrypt(etype, self, key, &iv));
        }

        None
    }

    pub fn save(&self, file_name: &str) -> std::io::Result<()> {
        let data = base64::decode(&self.base64).unwrap();
        let mut file = File::create(file_name)?;
        file.write_all(&data)?;
        Ok(())
    }
}
