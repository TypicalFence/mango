use std;
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use super::ImageFile;
use meta::MangoImageMetadata;
use compression;
use compression::CompressionType;
use encryption;
use encryption::EncryptionType;
use serde_bytes;
use base64;

#[derive(Serialize, Deserialize, Clone)]
pub struct MangoImage {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    meta: MangoImageMetadata,
}

impl MangoImage {
    pub fn new(data: Vec<u8>, meta: MangoImageMetadata) -> MangoImage {
        MangoImage { data, meta }
    }

    pub fn from_file(file_image: &mut ImageFile) -> MangoImage {
        let mut vec = Vec::new();
        file_image.get_file().read_to_end(&mut vec);
        let new_meta = file_image.get_meta();
        MangoImage::new(
            vec,
            new_meta.to_base64_metadata(),
        )
    }

    pub fn get_meta(&self) -> MangoImageMetadata {
        self.meta.clone()
    }

    pub fn get_meta_mut(&mut self) -> &mut MangoImageMetadata {
        &mut self.meta
    }

    pub fn get_image_data(&self) -> Vec<u8> {
        self.data.clone()
    }

	pub fn get_base64_image_data(&self) -> String {
		base64::encode(&self.data.clone())
	}

    pub fn compress(&self, comp: CompressionType) -> Option<MangoImage> {
        if self.meta.encryption.is_none()  && self.meta.compression.is_none() {
            return Some(compression::compress(comp, self));
        }

        None
    }

    pub fn compress_mut(&mut self, comp: CompressionType) -> bool {
        let compressed_opt = self.clone().compress(comp);
        if compressed_opt.is_some() {
            let compressed_img = compressed_opt.unwrap();
            self.data = compressed_img.get_image_data();
            self.meta = compressed_img.get_meta();
            true
        } else {
            false
        }
    }

    pub fn uncompress(&self) -> Option<MangoImage> {
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
            self.data = uncompressed_img.get_image_data();
            self.meta = uncompressed_img.get_meta();
            true
        } else {
            false
        }
    }

    pub fn encrypt(self, etype: EncryptionType, key: String) -> Option<MangoImage> {
        if self.meta.encryption.is_none() {
            return Some(encryption::encrypt(etype, self, key));
        }

        None
    }

    pub fn encrypt_mut(&mut self, etype: EncryptionType, key: String) -> bool {
        let encrypted_opt = self.clone().encrypt(etype, key);
        if encrypted_opt.is_some() {
            let encrypted_img = encrypted_opt.unwrap();
            self.data = encrypted_img.get_image_data();
            self.meta = encrypted_img.get_meta();
            true
        } else {
            false
        }
    }

    pub fn decrypt(self, key: String) -> Option<MangoImage> {
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
            self.data = decrypted_img.get_image_data();
            self.meta = decrypted_img.get_meta();
            true
        } else {
            false
        }
    }

    pub fn save(&self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(&self.data)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use std;
    use super::{MangoImage, ImageFile, EncryptionType, CompressionType};

    #[test]
    fn mut_crypt() {
        let p = std::path::Path::new("test.jpg");
        let mut file = ImageFile::open(p).unwrap();

        let mut img = MangoImage::from_file(&mut file);
        let clean_data = img.get_image_data();

        img.encrypt_mut(EncryptionType::AES256, String::from("1234567812345678"));
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

        let mut img = MangoImage::from_file(&mut file);
        let clean_data = img.get_image_data();

        img.compress_mut(CompressionType::GZIP);
        assert_eq!(img.get_meta().compression.is_some(), true);
        assert_ne!(img.get_image_data(), clean_data);
        img.uncompress_mut();
        img.save("lol.jpg");
        assert_eq!(img.get_meta().compression.is_none(), true);
        assert_eq!(img.get_image_data(), clean_data);
    }

    #[test]
    fn check_sum() {
        let p = std::path::Path::new("test.jpg");
        let mut file = ImageFile::open(p).unwrap();

        let mut img = MangoImage::from_file(&mut file);
        //img.meta.checksum
        assert_eq!(img.meta.checksum.len() > 0, true)
    }
}


