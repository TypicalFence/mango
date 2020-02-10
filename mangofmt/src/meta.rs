//! All Metadata Stuff.

use compression::CompressionType;
use encryption::EncryptionType;
use hex::ToHex;
use image::Mime;
use json::base64option;
use sha2::{Digest, Sha256};
use std::clone::Clone;
use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn get_checksum(file: &mut File) -> Option<String> {
    let mut data = Vec::new();

    if file.read_to_end(&mut data).is_err() {
        return None;
    }

    let mut hasher = Sha256::default();
    hasher.input(&data);
    let checksum = hasher.result();
    Some(checksum.to_hex())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageFileMetadata {
    pub path: String,
    pub checksum: String,
    pub mime: Mime,
}

impl ImageFileMetadata {
    pub fn new(path: &Path) -> Option<ImageFileMetadata> {
        match File::open(&path) {
            Ok(mut file) => {
                let checksum_opt = get_checksum(&mut file);
                if checksum_opt.is_some() {
                    let checksum = checksum_opt.unwrap();
                    match Mime::get_from_path(path) {
                        Ok(mime) => {
                            let path = path.to_str().unwrap().to_string();
                            Some(ImageFileMetadata {
                                path,
                                checksum,
                                mime,
                            })
                        }
                        Err(_e) => None,
                    }
                } else {
                    None
                }
            }
            Err(_e) => None,
        }
    }

    pub fn to_base64_metadata(&self) -> MangoImageMetadata {
        MangoImageMetadata::from_file_metadata(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MangoImageMetadata {
    pub compression: Option<CompressionType>,
    pub encryption: Option<EncryptionType>,
    #[serde(with = "base64option")]
    pub iv: Option<Vec<u8>>,
    pub filename: String,
    pub checksum: String,
    pub mime: Mime,
}

impl MangoImageMetadata {
    pub fn from_file_metadata(data: ImageFileMetadata) -> Self {
        let filename: String = data
            .path
            .split('/')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();

        Self {
            compression: None,
            encryption: None,
            iv: None,
            filename,
            checksum: data.checksum,
            mime: data.mime,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Language {
    EN,
    JP,
    DE,
    FR,
    IT,
    CN,
    ES,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MangoMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub source: Option<String>,
    pub translation: Option<String>,
    pub language: Option<Language>,
    pub volume: Option<i16>,
    pub chapter: Option<i16>,
    pub year: Option<i16>,
}

impl MangoMetadata {
    pub fn new() -> Self {
        Self {
            title: None,
            author: None,
            publisher: None,
            source: None,
            translation: None,
            language: None,
            volume: None,
            chapter: None,
            year: None,
        }
    }
}

impl Default for MangoMetadata {
    fn default() -> Self {
        Self::new()
    }
}
