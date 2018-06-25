use std;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use image::{ImageFile, MangoImage};
use json::JsonMangoFile;
use bson;

/// Structure that represents a mango file.
///
/// It can be used to create, save and modify the file format.
#[derive(Serialize, Deserialize)]
pub struct MangoFile {
    name: String,
    images: Vec<MangoImage>,
}

impl MangoFile {
    pub fn new(name: String) -> MangoFile {
        MangoFile {
            name: name,
            images: Vec::new(),
        }
    }

    // TODO check  what error serde returns
    pub fn open(p: &Path) -> Result<MangoFile, Box<Error>> {
        Self::open_json(&p)
    }

    pub fn open_bson(p: &Path) -> Result<MangoFile, Box<Error>> {
        let mut file = File::open(p)?;
        let document = bson::decode_document(&mut file)?;
        let u = bson::from_bson(bson::Bson::Document(document))?;

        Ok(u)
    }

    pub fn open_json(p: &Path) -> Result<MangoFile, Box<Error>> {
        JsonMangoFile::open(&p)
    }

    pub fn save(&self, p: &Path) {
        self.save_json(p);
    }

    pub fn save_bson(&self, p: &Path) {
        let bson_data = bson::to_bson(&self).unwrap();
        if let bson::Bson::Document(document) = bson_data {
            let mut buf = Vec::new();
            bson::encode_document(&mut buf, &document).unwrap();
            let mut f = File::create(p).unwrap();
            f.write_all(&buf);
        }
    }

    pub fn save_json(&self, p:&Path) {
        JsonMangoFile::save(p, self);
    }

    pub fn add_image(&mut self, image: MangoImage) {
        self.images.push(image);
    }

    pub fn add_image_by_path(&mut self, p: &Path) -> Result<(), std::io::Error> {
        let mut image_file = ImageFile::open(p)?;
        self.images.push(
            image_file.to_mango_image()
        );
        Ok(())
    }

    pub fn get_images(&self) -> Vec<MangoImage> {
        self.images.clone()
    }

    pub fn get_image(&self, index: usize) -> Option<&MangoImage> {
        if &self.images.len() -1 >= index {

            return Some(&self.images[index]);
        }

        None
    }

    pub fn get_image_mut(&mut self, index: usize) -> &mut MangoImage {
        &mut self.images[index]
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn set_name(&mut self, n: String) {
        self.name = n;
    }

    pub fn set_images(&mut self, imgs: Vec<MangoImage>) {
        self.images = imgs;
    }
}

#[cfg(test)]
mod tests {
    use super::MangoFile;
    use encryption;
    use std::path::Path;

    fn create() {
        let mut file = MangoFile::new("test".to_string());
        file.add_image_by_path(Path::new("test.jpg"));
        file.save(Path::new("test.json"));
    }

    #[test]
    fn create_and_open() {
        create();
        let file = MangoFile::open(Path::new("test.json"));
        assert_eq!(file.unwrap().name, "test");
    }

    // TODO move tests below to base64_image.rs
    #[test]
    fn encrypt() {
        let mut file = MangoFile::new("test".to_string());
        file.add_image_by_path(Path::new("test.jpg"));
        let image = file.get_image_mut(0);
        let key = String::from("1234567812345678");
        let encrypted_image = image.clone().encrypt(encryption::EncryptionType::AES128, key.clone());
        let decrypted_image = encrypted_image.unwrap().decrypt(key).unwrap();

        assert_eq!(image.get_image_data(), decrypted_image.get_image_data());
    }

    #[test]
    fn save() {
        let mut file = MangoFile::new("test".to_string());
        file.add_image_by_path(Path::new("test.jpg"));
        let image = file.get_image_mut(0);
        image.save("test_unencrypted.jpg");
    }

    #[test]
    fn  save_bson() {
        use compression::CompressionType;
        use encryption::EncryptionType;
        use image::{MangoImage, ImageFile};

        let mut file = MangoFile::new("test".to_string());
        let mut img = MangoImage::from_file(&mut ImageFile::open(Path::new("test.jpg")).unwrap());
        img.compress_mut(CompressionType::GZIP);
        img.encrypt_mut(EncryptionType::AES128, "1234567812345678".to_lowercase());
        file.add_image(img);
        file.save_bson(Path::new("teste.bson"));
    }
}
