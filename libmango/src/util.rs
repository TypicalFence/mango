use mangofmt::CompressionType;
use mangofmt::EncryptionType;
use mangofmt::Mime;
use mangofmt::file::{MangoFileError, ErrorKind};

pub fn to_comp_type(value: String) -> Option<CompressionType> {
    match value.as_ref() {
        "GZIP" => Some(CompressionType::GZIP),
        _ => None
    }
}

pub fn from_comp_type(value: CompressionType) -> String {
    match value {
        CompressionType::GZIP => "GZIP".to_string(),
    }
}


pub fn to_enc_type(value: String) -> Option<EncryptionType> {
    match value.as_ref() {
        "AES128" => Some(EncryptionType::AES128),
        "AES256" => Some(EncryptionType::AES256),
        _ => None
    }
}

pub fn from_enc_type(value: EncryptionType) -> String {
    match value {
        EncryptionType::AES256 => "AES256".to_string(),
        EncryptionType::AES128 => "AES128".to_string(),
    }
}

pub fn from_mime(value: Mime) -> String {
    match value {
        Mime::JPEG => "JPEG".to_string(),
        Mime::PNG => "PNG".to_string(),
    }
}

pub fn handle_mangofile_error(error: MangoFileError) -> i16 {
    match error.get_kind() {
        ErrorKind::DecodeError => 1,
        ErrorKind::EncodeError => 1,
        ErrorKind::ReadError => 2,
        ErrorKind::WriteError => 2,
        ErrorKind::PermissionError => 3,
    }
}
