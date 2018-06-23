use base64;
use bson;
use bson::spec::BinarySubtype;
use bson::Bson::Binary;
use bson::Bson;
use std::fs::File;
use std::path::Path;
use super::Base64Image;
use super::Base64ImageMetadata;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct BSONMangoFile {
    name: String,
    images: Vec<BSONImage>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BSONImage {
    data: Bson,
    meta: Base64ImageMetadata,
}

impl BSONImage {
    pub fn from_mango_image(img: &Base64Image) -> Self {
        Self {
            data: Binary(BinarySubtype::Generic, base64::decode(&img.get_image_data()).unwrap()),
            meta: img.get_meta(),
        }
    }

    pub fn save_bson(&self, p: &Path) {
        let bson_data = bson::to_bson(&self).unwrap();
        if let bson::Bson::Document(document) = bson_data {
            let mut bytes = Vec::new();
            bson::encode_document(&mut bytes, &document);
            let mut f = File::create(p).unwrap();
            f.write_all(&bytes);
        }
    }
}
