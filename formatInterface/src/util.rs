use mango_format::CompressionType;

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
