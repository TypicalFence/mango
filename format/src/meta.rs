use std::default::Default;
use std::path::Path;
use std::io::Read;
use std::fs::File;
use sha2::{Sha256, Digest};
use hex::ToHex;
use compress::CompressionType;
use encrypt::EncryptionType;

fn get_checksum(file: &mut File) -> String {
    let mut data = Vec::new();
    file.read_to_end(&mut data);
    let mut hasher = Sha256::default();
    hasher.input(&data);
    let checksum = hasher.result();
    checksum.to_hex()
}

pub struct FileImageMetadata {
    path: String,
    checksum: String,
}

impl FileImageMetadata {
    pub fn new(path: &Path) -> Option<FileImageMetadata>{
        match File::open(&path) {
            Ok(mut file) => {
                let checksum = get_checksum(&mut file);
                Some(FileImageMetadata {
                    path: path.to_str().unwrap().to_string(),
                    checksum,
                })
            }
            Err(e) => None,
        }
    }
}

struct Base64ImageMetadata {
    compression: Option<CompressionType>,
    encryption: Option<EncryptionType>,
    filename: String,
    checksum: String,
}
