use base64;
use bson;
use bson::spec::BinarySubtype;
use bson::Bson::Binary;
use bson::Bson;
use std::fs::File;
use std::path::Path;
use image::MangoImage;
use meta::MangoImageMetadata;
use file::MangoFile;
use std::io::prelude::*;
use compression::CompressionType;
use encryption::EncryptionType;
use image::Mime;

#[derive(Serialize, Deserialize, Clone)]
pub struct BSONMangoFile {
    name: String,
    images: Vec<BSONImage>,
}

impl BSONMangoFile {
    pub fn from_mangofile(file: &MangoFile) -> Self {
        let mut bson_imgs = Vec::new();

        for image in file.get_images() {
            bson_imgs.push(BSONImage::from_mango_image(&image))
        }

        Self {
            name: file.get_name(),
            images: bson_imgs,
        }
    }

    pub fn save(&self, p: &Path) {
        let bson_data = bson::to_bson(&self).unwrap();
        if let bson::Bson::Document(document) = bson_data {
            let mut bytes = Vec::new();
            bson::encode_document(&mut bytes, &document);
            let mut f = File::create(p).unwrap();
            f.write_all(&bytes);
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct BsonImageMetadata {
    pub compression: Option<CompressionType>,
    pub encryption: Option<EncryptionType>,
    pub iv: Option<Bson>,
    pub filename: String,
    pub checksum: String,
    pub mime: Mime,
}

impl BsonImageMetadata {
    pub fn from_mango(meta: MangoImageMetadata) -> Self {
        let mut iv = None;

        if meta.iv.is_some() {
            iv = Some(Binary(BinarySubtype::Generic, meta.iv.unwrap()));
        }

        Self {
            compression: meta.compression,
            encryption: meta.encryption,
            filename: meta.filename,
            checksum: meta.checksum,
            mime: meta.mime,
            iv,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct BSONImage {
    data: Bson,
    meta: BsonImageMetadata,
}

impl BSONImage {
    pub fn from_mango_image(img: &MangoImage) -> Self {
        Self {
            data: Binary(BinarySubtype::Generic, img.get_image_data()),
            meta: BsonImageMetadata::from_mango(img.get_meta()),
        }
    }
}
