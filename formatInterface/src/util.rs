use mango_format::CompressionType;
use mango_format::EncryptionType;

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

pub fn from_enc_type(value: EncryptionType) -> String {
    match value {
        EncryptionType::AES256 => "AES256".to_string(),
        EncryptionType::AES128 => "AES128".to_string(),
    }
}
