use std::default::Default;
use std::path::Path;
use std::io::Read;
use std::fs::File;
use std::clone::Clone;
use sha2::{Sha256, Digest};
use hex::ToHex;
use compression::CompressionType;
use encryption::EncryptionType;
use image::Mime;
use json::base64Option;

fn get_checksum(file: &mut File) -> String {
    let mut data = Vec::new();
file.read_to_end(&mut data);
    let mut hasher = Sha256::default();
    hasher.input(&data);
    let checksum = hasher.result();
    checksum.to_hex()
}

#[derive(Serialize, Deserialize)]
pub struct ImageFileMetadata {
    pub path: String,
    pub checksum: String,
    pub mime: Mime,
}

impl ImageFileMetadata {
    pub fn new(path: &Path) -> Option<ImageFileMetadata> {
        match File::open(&path) {
            Ok(mut file) => {
                let checksum = get_checksum(&mut file);
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
            }
            Err(_e) => None,
        }
    }

    pub fn to_base64_metadata(self) -> MangoImageMetadata {
        MangoImageMetadata::from_file_metadata(self)
    }
}

impl Clone for ImageFileMetadata {
    fn clone(&self) -> ImageFileMetadata {
        ImageFileMetadata {
            path: self.path.clone(),
            checksum: self.checksum.clone(),
            mime: self.mime.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MangoImageMetadata {
    pub compression: Option<CompressionType>,
    pub encryption: Option<EncryptionType>,
    #[serde(with = "base64Option")]
    pub iv: Option<Vec<u8>>,
    pub filename: String,
    pub checksum: String,
    pub mime: Mime,
}


impl MangoImageMetadata {
    pub fn from_file_metadata(data: ImageFileMetadata) -> Self {
         Self {
            compression: None,
            encryption: None,
            iv: None,
            //TODO fix filename
            filename: data.path,
            checksum: data.checksum,
            mime: data.mime,
        }
    }
}

impl Clone for MangoImageMetadata {
    fn clone(&self) -> Self {
        Self {
            // TODO just WHY???
            compression: match self.compression.clone() {
                Some(v) => Some(v),
                None => None,
            },
            encryption: match self.encryption.clone() {
                Some(e) => Some(e),
                None => None,
            },
            iv: self.iv.clone(),
            filename: self.filename.clone(),
            checksum: self.checksum.clone(),
            mime: self.mime.clone(),
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
