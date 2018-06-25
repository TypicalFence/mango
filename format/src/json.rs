use std;
use std::io::prelude::*;
use serde_json;
use image::{MangoImage, Mime};
use meta::MangoImageMetadata;
use file::MangoFile;
use super::{CompressionType, EncryptionType};
use std::fs::File;
use std::path::Path;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonMangoFile {
    pub name: String,
    images: Vec<Base64Image>,
}

impl JsonMangoFile {
    fn new(name: String, images: Vec<Base64Image>) -> Self {
        Self { name, images }
    }

    pub fn get_images(&self) -> Vec<Base64Image> {
        self.images.clone()
    }

    pub fn open(p: &Path) -> Result<MangoFile, Box<Error>> {
        let file = File::open(p)?;

        let img: JsonMangoFile = serde_json::from_reader(file)?;

        let mut mango_imgs = Vec::new();

        for image in img.get_images() {
            mango_imgs.push(Base64Image::to_mango(image));
        }

        let mut mango_file = MangoFile::new(img.name);
        mango_file.set_images(mango_imgs);

        Ok(mango_file)
    }

    pub fn save(p: &Path, file: &MangoFile) -> Result<(), std::io::Error> {

        let mut base64_imgs = Vec::new();

        for image in file.get_images() {
            base64_imgs.push(Base64Image::from_mango(image));
        }

        let json_string =
            serde_json::to_string_pretty(&JsonMangoFile::new(file.get_name(), base64_imgs))?;
        let mut f = File::create(p)?;
        f.write_all(json_string.as_bytes())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Base64ImageMetadata {
    pub compression: Option<CompressionType>,
    pub encryption: Option<EncryptionType>,
    #[serde(with = "base64Option")]
    pub iv: Option<Vec<u8>>,
    pub filename: String,
    pub checksum: String,
    pub mime: Mime,
}

impl Base64ImageMetadata {
    pub fn from_mango(meta: MangoImageMetadata) -> Self {
        Self {
            compression: meta.compression,
            encryption: meta.encryption,
            iv: meta.iv,
            filename: meta.filename,
            checksum: meta.checksum,
            mime: meta.mime,
        }
    }

    pub fn to_mango(meta: Self) -> MangoImageMetadata {
        MangoImageMetadata {
            compression: meta.compression,
            encryption: meta.encryption,
            iv: meta.iv,
            filename: meta.filename,
            checksum: meta.checksum,
            mime: meta.mime,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Base64Image {
    #[serde(with = "base64Encoding")]
    pub data: Vec<u8>,
    pub meta: Base64ImageMetadata,
}

impl Base64Image {
    pub fn from_mango(img: MangoImage) -> Self {
        Self {
            data: img.get_image_data(),
            meta: Base64ImageMetadata::from_mango(img.get_meta()),
        }
    }

    pub fn to_mango(img: Self) -> MangoImage {
        MangoImage::new(img.data, Base64ImageMetadata::to_mango(img.meta))
    }
}

pub mod base64Encoding {
    use base64;
    use serde::{Serializer, de, Deserialize, Deserializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(bytes).replace("\r\n", ""))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        base64::decode(&s).map_err(de::Error::custom)
    }
}

pub mod base64Option {
    use base64;
    use serde::{Serializer, de, Deserialize, Deserializer};

    pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if (bytes.is_some()) {
            let b = bytes.clone().unwrap();
            serializer.serialize_some(&base64::encode(&b).replace("\r\n", ""))
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <Option<String>>::deserialize(deserializer)?;
        if (s.is_some()) {
            let bytes = base64::decode(&s.unwrap()).map_err(de::Error::custom)?;
            Ok(Some(bytes))
        } else {
            Ok(None)
        }
    }
}
