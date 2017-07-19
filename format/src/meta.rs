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


fn get_checksum(file: &mut File) -> String {
    let mut data = Vec::new();
    file.read_to_end(&mut data);
    let mut hasher = Sha256::default();
    hasher.input(&data);
    let checksum = hasher.result();
    checksum.to_hex()
}

#[derive(Serialize, Deserialize)]
pub struct FileImageMetadata {
    pub path: String,
    pub checksum: String,
    pub mime: Mime,
}

impl FileImageMetadata {
    pub fn new(path: &Path) -> Option<FileImageMetadata> {
        match File::open(&path) {
            Ok(mut file) => {
                let checksum = get_checksum(&mut file);
                match Mime::get_from_path(path) {
                    Ok(mime) => {
                        let path = path.to_str().unwrap().to_string();
                        Some(FileImageMetadata {
                            path,
                            checksum,
                            mime,
                        })
                    }
                    Err(e) => None,
                }
            }
            Err(e) => None,
        }
    }

    pub fn to_base64_metadata(self) -> Base64ImageMetadata {
        Base64ImageMetadata::from_file_metadata(self)
    }
}

impl Clone for FileImageMetadata {
    fn clone(&self) -> FileImageMetadata {
        FileImageMetadata {
            path: self.path.clone(),
            checksum: self.checksum.clone(),
            mime: self.mime.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Base64ImageMetadata {
    pub compression: Option<CompressionType>,
    pub encryption: Option<EncryptionType>,
    pub filename: String,
    pub checksum: String,
    pub mime: Mime,
}


impl Base64ImageMetadata {
    pub fn from_file_metadata(data: FileImageMetadata) -> Base64ImageMetadata {
        Base64ImageMetadata {
            compression: Option::from(Option::None),
            encryption: Option::from(Option::None),
            //TODO fix filename
            filename: data.path,
            checksum: data.checksum,
            mime: data.mime,
        }
    }
}

impl Clone for Base64ImageMetadata {
    fn clone(&self) -> Base64ImageMetadata {
        Base64ImageMetadata {
            compression: match self.compression.clone() {
                Some(v) => Some(v),
                None => None,
            },
            encryption: match self.encryption.clone() {
                Some(e) => Some(e),
                None => None,
            },
            filename: self.filename.clone(),
            checksum: self.checksum.clone(),
            mime: self.mime.clone(),
        }
    }
}
