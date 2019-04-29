use std;
use std::fs::File;
use std::io::prelude::*;
use super::ImageFile;
use meta::MangoImageMetadata;
use compression;
use compression::{CompressionType, CompressionError};
use encryption;
use encryption::{EncryptionType, EncryptionError};
use serde_bytes;
use base64;

/// Represents an image inside of a MangoFile.
///
/// It contains raw image data as a Vec containing its bytes (u8) and some meta data in form of
/// an instance of MangoImageMetaData.
///
/// The image data can be compressed and encrypted, see the coresponding methods for more info.
#[derive(Serialize, Deserialize, Clone)]
pub struct MangoImage {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    meta: MangoImageMetadata,
}

impl MangoImage {
    /// Creates a new MangoImage,
    ///
    /// but you have to provide all values.
    pub fn new(data: Vec<u8>, meta: MangoImageMetadata) -> MangoImage {
        MangoImage { data, meta }
    }
    
    /// Creates a new MangoImage based on an ImageFile.
    pub fn from_file(file_image: &ImageFile) -> MangoImage {
        let mut vec = Vec::new();

        // we can assume that ImageFile struct returns a valid file Path struct
        // therefore we can ignore the following result (probably) (maybe?)
        let mut file = File::open(file_image.get_path().as_path()).unwrap();
        #[allow(unused_must_use)]
        file.read_to_end(&mut vec).is_err();

        let new_meta = file_image.get_meta();
        MangoImage::new(
            vec,
            new_meta.to_base64_metadata(),
        )
    }
    
    /// Returns the meta data.
    pub fn get_meta(&self) -> MangoImageMetadata {
        self.meta.clone()
    }
    
    /// Returns the meta data in a mutable form.
    pub fn get_meta_mut(&mut self) -> &mut MangoImageMetadata {
        &mut self.meta
    }
    
    /// Returns the raw image data.
    pub fn get_image_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    
    /// Returns the raw image data in a Base64 encoding.
	pub fn get_base64_image_data(&self) -> String {
		base64::encode(&self.data.clone())
	}
    
    /// Compresses the MangoImage and returns a copy of it.
    pub fn compress(&self, comp: CompressionType) -> Result<MangoImage, CompressionError> {
        if self.meta.encryption.is_none() && self.meta.compression.is_none() {
            return compression::compress(comp, self);
        }

        Err(CompressionError::UnsupportedType)
    }
    
    /// Compresses this MangoImage instance and returns if it worked or not.
    pub fn compress_mut(&mut self, comp: CompressionType) -> bool {
        let compressed_opt = self.clone().compress(comp);
        if compressed_opt.is_ok() {
            let compressed_img = compressed_opt.unwrap();
            self.data = compressed_img.get_image_data();
            self.meta = compressed_img.get_meta();
            true
        } else {
            false
        }
    }
    
    /// Decompresses the MangoImage and returns a copy of it.
    pub fn uncompress(&self) -> Result<MangoImage, CompressionError> {
        let meta = &self.meta;

        if meta.compression.is_some() && meta.encryption.is_none() {
            let comp = meta.clone().compression.unwrap();
            return compression::uncompress(comp, self);
        }

        Err(CompressionError::UnsupportedType)
    }
    
    /// Decompresses this MangoImage instance and returns if it worked or not.
    pub fn uncompress_mut(&mut self) -> bool {
        let uncompressed_opt = self.clone().uncompress();
        if uncompressed_opt.is_ok() {
            let uncompressed_img = uncompressed_opt.unwrap();
            self.data = uncompressed_img.get_image_data();
            self.meta = uncompressed_img.get_meta();
            true
        } else {
            false
        }
    }
    
    /// Encrypts the MangoImage and returns a copy of it.
    pub fn encrypt(self, etype: EncryptionType, key: String) -> Result<MangoImage, EncryptionError> {
        if self.meta.encryption.is_none() {
            return encryption::encrypt(etype, self, key);
        }

        Err(EncryptionError::UnsupportedType)
    }
    
    /// Encrypts this MangoImage instance and returns if it worked or not.
    pub fn encrypt_mut(&mut self, etype: EncryptionType, key: String) -> bool {
        let encrypted_opt = self.clone().encrypt(etype, key);
        if encrypted_opt.is_ok() {
            let encrypted_img = encrypted_opt.unwrap();
            self.data = encrypted_img.get_image_data();
            self.meta = encrypted_img.get_meta();
            true
        } else {
            false
        }
    }
    

    /// Decrypts the MangoImage and returns a copy of it.
    pub fn decrypt(self, key: String) -> Result<MangoImage, EncryptionError> {
        if self.meta.encryption.is_some() && self.meta.iv.is_some() {
            let iv = self.meta.iv.clone().unwrap();
            let etype = self.meta.encryption.clone().unwrap();
            return encryption::decrypt(etype, self, key, &iv);
        }

        Err(EncryptionError::UnsupportedType)
    }

    /// Decrypts this MangoImage instance and returns if it worked or not.
    pub fn decrypt_mut(&mut self, key: String) -> bool {
        let decrypted_opt = self.clone().decrypt(key);
        if decrypted_opt.is_ok() {
            let decrypted_img = decrypted_opt.unwrap();
            self.data = decrypted_img.get_image_data();
            self.meta = decrypted_img.get_meta();
            true
        } else {
            false
        }
    }
    
    /// saves the raw image data to a file.
    pub fn save(&self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(&self.data)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use std;
    use super::{MangoImage, ImageFile};
    use encryption::EncryptionType;
    use compression::CompressionType;


    #[test]
    #[cfg(feature = "aes")]
    fn mut_crypt() {
        let p = std::path::Path::new("test.jpg");
        let mut file = ImageFile::open(p).unwrap();

        let mut img = MangoImage::from_file(&mut file);
        let clean_data = img.get_image_data();

        img.encrypt_mut(EncryptionType::AES128, String::from("1234567812345678"));
        assert_eq!(img.get_meta().encryption.is_some(), true);
        assert_ne!(img.get_image_data(), clean_data);
        img.decrypt_mut(String::from("1234567812345678"));
        assert_eq!(img.get_meta().encryption.is_none(), true);
        assert_eq!(img.get_image_data(), clean_data);
    }

    #[test]
    #[cfg(feature = "gzip")]
    fn mut_compress() {
        let p = std::path::Path::new("test.jpg");
        let mut file = ImageFile::open(p).unwrap();

        let mut img = MangoImage::from_file(&mut file);
        let clean_data = img.get_image_data();

        img.compress_mut(CompressionType::GZIP);
        assert_eq!(img.get_meta().compression.is_some(), true);
        assert_ne!(img.get_image_data(), clean_data);
        img.uncompress_mut();
        assert!(img.save("lol.jpg").is_ok());
        assert_eq!(img.get_meta().compression.is_none(), true);
        assert_eq!(img.get_image_data(), clean_data);
    }

    #[test]
    fn check_sum() {
        let p = std::path::Path::new("test.jpg");
        let mut file = ImageFile::open(p).unwrap();

        let img = MangoImage::from_file(&mut file);
        //img.meta.checksum
        assert_eq!(img.meta.checksum.len() > 0, true)
    }
}


