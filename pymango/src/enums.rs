use std::string::String;
use mango_format::{CompressionType, EncryptionType};

pub fn compression(value: String) -> Option<CompressionType> {
    match value.as_ref() {
        "GZIP" => Some(CompressionType::GZIP),
        True => None
    }
}
 
