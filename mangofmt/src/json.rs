use std::io::prelude::*;
use serde_json;
use image::{MangoImage, Mime};
use meta::MangoImageMetadata;
use file::{MangoFile, ErrorKind, MangoFileError};
use super::{CompressionType, EncryptionType};
use std::fs::File;
use std::path::Path;
use meta::MangoMetadata;

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonMangoFile {
    pub meta: MangoMetadata,
    images: Vec<Base64Image>,
}

impl JsonMangoFile {
    fn new(meta: MangoMetadata, images: Vec<Base64Image>) -> Self {
        Self { meta, images }
    }

    fn get_images(&self) -> Vec<Base64Image> {
        self.images.clone()
    }

    pub fn open(p: &Path) -> Result<MangoFile, MangoFileError> {
        let file = File::open(p);
        if file.is_err() {
            return Err(MangoFileError::convert_io_open(file.err().unwrap()));
        }

        let json_result = serde_json::from_reader(file.unwrap());
        if json_result.is_err() {
            return Err(MangoFileError::with_cause(ErrorKind::DecodeError,
                                                  "couldn't decode JSON to MangoFile",
                                                  json_result.err().unwrap()));
        }

        // convert JsonMangoFile to MangoFile
        let mut mango_imgs = Vec::new();

        let json_file: JsonMangoFile = json_result.unwrap();

        for image in json_file.get_images() {
            mango_imgs.push(Base64Image::to_mango(&image));
        }

        let mut mango_file = MangoFile::new();
        mango_file.set_images(mango_imgs);
        mango_file.set_meta(json_file.meta);

        Ok(mango_file)
    }

    pub fn save(p: &Path, file: &MangoFile) -> Result<(), MangoFileError> {

        let mut base64_imgs = Vec::new();

        for image in file.get_images() {
            base64_imgs.push(Base64Image::from_mango(&image));
        }

        let json_string = serde_json::to_string_pretty(&JsonMangoFile::new(file.get_meta(),
                                                                           base64_imgs));

        if json_string.is_err() {
            return Err(MangoFileError::with_cause(ErrorKind::EncodeError,
                                                  "couldn't encode JSON to MangoFile",
                                                  json_string.err().unwrap()));
        }

        let f = File::create(p);

        if f.is_err() {
            return Err(MangoFileError::convert_io_save(f.err().unwrap()));
        }

        let write = f.unwrap().write_all(json_string.unwrap().as_bytes());
        if write.is_err() {
            return Err(MangoFileError::convert_io_save(write.err().unwrap()));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Base64ImageMetadata {
    pub compression: Option<CompressionType>,
    pub encryption: Option<EncryptionType>,
    #[serde(with = "base64option")]
    pub iv: Option<Vec<u8>>,
    pub filename: String,
    pub checksum: String,
    pub mime: Mime,
}

impl Base64ImageMetadata {
    pub fn from_mango(meta: &MangoImageMetadata) -> Self {
        Self {
            compression: meta.compression.clone(),
            encryption: meta.encryption.clone(),
            iv: meta.iv.clone(),
            filename: meta.filename.clone(),
            checksum: meta.checksum.clone(),
            mime: meta.mime,
        }
    }

    pub fn to_mango(&self) -> MangoImageMetadata {
        MangoImageMetadata {
            compression: self.compression.clone(),
            encryption: self.encryption.clone(),
            iv: self.iv.clone(),
            filename: self.filename.clone(),
            checksum: self.checksum.clone(),
            mime: self.mime,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Base64Image {
    #[serde(with = "base64encoding")]
    pub data: Vec<u8>,
    pub meta: Base64ImageMetadata,
}

impl Base64Image {
    pub fn from_mango(img: &MangoImage) -> Self {
        Self {
            data: img.get_image_data(),
            meta: Base64ImageMetadata::from_mango(&img.get_meta()),
        }
    }

    pub fn to_mango(&self) -> MangoImage {
        MangoImage::new(self.data.clone(), Base64ImageMetadata::to_mango(&self.meta))
    }
}

pub mod base64encoding {
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

pub mod base64option {
    use base64;
    use serde::{Serializer, de, Deserialize, Deserializer};

    pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if bytes.is_some() {
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
        if s.is_some() {
            let bytes = base64::decode(&s.unwrap()).map_err(de::Error::custom)?;
            Ok(Some(bytes))
        } else {
            Ok(None)
        }
    }
}
