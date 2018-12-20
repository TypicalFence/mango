//! Contains the MangoFile struct and some related stuff.

use std;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::fmt;
use image::{ImageFile, MangoImage};
use json::JsonMangoFile;
use bson;
use serde_cbor;
use meta::MangoMetadata;

//------------------------------------------------------------------------------
//  Custom Error
//------------------------------------------------------------------------------

/// Holds all possible Errors for MangoFileErrors
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ErrorKind {
    EncodeError,
    DecodeError,
    ReadError,
    WriteError,
    PermissionError,
}

impl ErrorKind {
    pub fn description(self) -> &'static str {
        match self {
            ErrorKind::EncodeError => "error while encoding the MangoFile",
            ErrorKind::DecodeError => "error while decoding the MangoFile",
            ErrorKind::ReadError => "error while reading the MangoFile",
            ErrorKind::WriteError => "error while writing the MangoFile",
            ErrorKind::PermissionError => "permission denied",
        }
    }
}

/// Custom Error for everything concerning MangoFiles
#[derive(Debug)]
pub struct MangoFileError {
    kind: ErrorKind,
    msg: &'static str,
    cause: Option<Box<Error + Send + Sync>>,
}

impl MangoFileError {
    pub fn new(kind: ErrorKind, msg: &'static str) -> Self {
        Self { kind, msg, cause: None }
    }

    pub fn with_cause<E>(kind: ErrorKind, msg: &'static str, cause: E) -> Self
    where E: Into<Box<Error + Send + Sync>>
    {
        Self { kind, msg, cause: Some(cause.into()) }
    }

    pub fn get_kind(self) -> ErrorKind {
        self.kind
    }

    pub fn convert_io_open(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => MangoFileError::with_cause(ErrorKind::ReadError, "not found", error),
            io::ErrorKind::PermissionDenied => MangoFileError::with_cause(ErrorKind::PermissionError, "permission denied", error),
            _ => MangoFileError::with_cause(ErrorKind::PermissionError, "unexpected io error", error),
        }
    }

    pub fn convert_io_save(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::PermissionDenied => MangoFileError::with_cause(ErrorKind::PermissionError, "permission denied", error),
            _ => MangoFileError::with_cause(ErrorKind::PermissionError, "unexpected io error", error),
        }
    }
}


impl fmt::Display for MangoFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       if let Some(ref cause) = self.cause {
           return write!(f, "{} ({}); cause: {}",
                         self.msg, self.kind.description(), cause);
       }

        write!(f, "{} ({})", self.msg, self.kind.description())
    }
}

impl Error for MangoFileError {
    fn description(&self) -> &str {
        self.msg
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| e.as_ref() as &Error)
    }
}

//------------------------------------------------------------------------------
// MangoFile Struct
//------------------------------------------------------------------------------

/// Rrepresents a mango file.
///
/// It can be used to create, save and modify a file of the format.
#[derive(Serialize, Deserialize)]
pub struct MangoFile {
    meta: MangoMetadata,
    images: Vec<MangoImage>,
}

impl MangoFile {
    /// Creates a new Instance
    pub fn new() -> MangoFile {
        MangoFile {
            meta: MangoMetadata::new(),
            images: Vec::new(),
        }
    }

    // TODO fix error (also handle read errors)
    /// Opens a existing .mango file
    pub fn open(p: &Path) -> Result<MangoFile, MangoFileError> {
        // try to open the default format cbor
        let cbor_file = Self::open_cbor(&p);
        if cbor_file.is_ok() {
            return cbor_file;
        }

        // try bson as a fallback
        let bson_file = Self::open_bson(&p);
        if bson_file.is_ok() {
            return bson_file;
        }

        // open json, should the support for this dropped?
        let json_file = Self::open_json(&p);
        if json_file.is_ok() {
            return json_file;
        }

        Err(MangoFileError::new(ErrorKind::DecodeError,
                                   "file is not a  MangoFile"))
    }
    
    /// Opens a MangoFile which uses bson as for serialization.
    ///
    /// You probably don't want to use this function, if you don't know the serialization format,
    /// use just [open](#method.open) instead.
    pub fn open_bson(p: &Path) -> Result<MangoFile, MangoFileError> {
        let file = File::open(p);

        if file.is_err() {
            return Err(MangoFileError::convert_io_open(file.err().unwrap()));
        }

        let document = bson::decode_document(&mut file.unwrap());

        if document.is_err() {
            return Err(MangoFileError::with_cause(ErrorKind::DecodeError,
                                                  "couldn't decode BSON Document",
                                                  document.err().unwrap()));
        }

        let mangofile = bson::from_bson(bson::Bson::Document(document.unwrap()));

        if mangofile.is_err() {
            return Err(MangoFileError::with_cause(ErrorKind::DecodeError,
                                                  "couldn't convert BSON Document to MangoFile",
                                                  mangofile.err().unwrap()));
        }

        Ok(mangofile.unwrap())
    }

    /// Opens a MangoFile which uses json as for serialization.
    ///
    /// You probably don't want to use this function, if you don't know the serialization format,
    /// use just [open](#method.open) instead.
    pub fn open_json(p: &Path) -> Result<MangoFile, MangoFileError> {
        JsonMangoFile::open(&p)
    }

    /// Opens a MangoFile which uses cborn as for serialization.
    ///
    /// You probably don't want to use this function, if you don't know the serialization format,
    /// use just [open](#method.open) instead.
    pub fn open_cbor(p: &Path) -> Result<MangoFile, MangoFileError> {
        let file = File::open(p);

        if file.is_err() {
            return Err(MangoFileError::convert_io_open(file.err().unwrap()));
        }

        let mut bytes = Vec::new();
        if file.unwrap().read_to_end(&mut bytes).is_err() {
            return Err(MangoFileError::new(ErrorKind::ReadError, "could not read file"));
        };

        let mangofile = serde_cbor::from_slice(&bytes);

        if mangofile.is_err() {
            return Err(MangoFileError::with_cause(ErrorKind::DecodeError,
                                                  "couldn't decode CBOR",
                                                  mangofile.err().unwrap()));
        }

        Ok(mangofile.unwrap())
    }

    /// Saves a .mango file with the default serialization format. (currently cbor)
    pub fn save(&self, p: &Path) -> Result<(), MangoFileError> {
        // use cbor as the default format
        // (lowest overhead)
        self.save_cbor(p)?;
        Ok(())
    }
    
    /// Saves a .mango file with the bson serialization format.    
    pub fn save_bson(&self, p: &Path) -> Result<(), MangoFileError> {
        let bson_data = bson::to_bson(&self);

        if bson_data.is_err() {
            return Err(MangoFileError::with_cause(ErrorKind::EncodeError, "couldn't encode to BSON", bson_data.err().unwrap()));
        }

        if let bson::Bson::Document(document) = bson_data.unwrap() {
            let mut buf = Vec::new();

            let encode = bson::encode_document(&mut buf, &document);
            if encode.is_err() {
                return Err(MangoFileError::with_cause(ErrorKind::EncodeError, "couldn't encode to BSON", encode.err().unwrap()));
            }

            let mut file = File::create(p);
            if file.is_err() {
                return Err(MangoFileError::convert_io_save(file.err().unwrap()));
            }
            let write = file.unwrap().write_all(&buf);
            if write.is_err() {
                return Err(MangoFileError::convert_io_save(write.err().unwrap()));
            }
        }

        Ok(())
    }
 
    /// Saves a .mango file with the json serialization format.
    ///
    /// **Important:** you should not use this for anything but debugging,
    /// because it gets saved to plaintext, and the image data will get encoded with base64.
    /// This will lead to an huge overhead, resulting in huge file sizes.
    ///
    /// But the function is currently here because its nice for debugging because you can easily look
    /// at json files, and see if everything works how it should.
    ///
    /// There are currently no plans to deprecate this serialization format.
    pub fn save_json(&self, p:&Path) -> Result<(), MangoFileError> {
        JsonMangoFile::save(p, self)?;
        Ok(())
    }

    /// Saves a .mango file with the cbor serialization format. (default format)
    pub fn save_cbor(&self, p: &Path) -> Result<(), MangoFileError> {
        let bytes = serde_cbor::to_vec(&self);
        if bytes.is_err() {
            return Err(MangoFileError::with_cause(ErrorKind::EncodeError, "couldn't encode to CBOR", bytes.err().unwrap()));
        }

        let file = File::create(p);
        if file.is_err() {
            return Err(MangoFileError::convert_io_save(file.err().unwrap()));
        }

        let write = file.unwrap().write_all(&bytes.unwrap());
        if write.is_err() {
            return Err(MangoFileError::convert_io_save(write.err().unwrap()));
        }

        Ok(())
    }

    /// Adds a MangoImage to the file
    ///
    /// use add_image_by_path for a neat shortcut
    pub fn add_image(&mut self, image: MangoImage) {
        self.images.push(image);
    }

    /// Adds a MangoImage to the file by Path
    pub fn add_image_by_path(&mut self, p: &Path) -> Result<(), std::io::Error> {
        let mut image_file = ImageFile::open(p)?;
        self.images.push(
            image_file.to_mango_image()
        );
        Ok(())
    }

    /// Gets all images of the file
    pub fn get_images(&self) -> Vec<MangoImage> {
        self.images.clone()
    }

    /// Gets one image of the file
    pub fn get_image(&self, index: usize) -> Option<&MangoImage> {
        if &self.images.len() -1 >= index {

            return Some(&self.images[index]);
        }

        None
    }

    /// Gets a mutable image from the file
    pub fn get_image_mut(&mut self, index: usize) -> &mut MangoImage {
        &mut self.images[index]
    }

    /// Gets a copy of the metadata of the file
    pub fn get_meta(&self) -> MangoMetadata {
        self.meta.clone()
    }

    /// Gets a reference of the metadata of the file
    pub fn get_meta_ref(&self) -> &MangoMetadata {
        &self.meta
    }

    /// Gets a mutable reference of the metadata of the file
    pub fn get_meta_mut(&mut self) -> &mut MangoMetadata {
        &mut self.meta
    }

    // TODO is this really needed?
    /// Sets a new Metadata object
    pub fn set_meta(&mut self, meta: MangoMetadata) {
        self.meta = meta;
    }

    /// Sets the images of the file
    pub fn set_images(&mut self, imgs: Vec<MangoImage>) {
        self.images = imgs;
    }
}

#[cfg(test)]
mod tests {
    use super::MangoFile;
    use std::path::Path;
    use encryption;

    fn create() {
        let mut file = MangoFile::new();
        file.get_meta_mut().title = Some("test".to_string());
        let added = file.add_image_by_path(Path::new("test.jpg"));
        assert!(added.is_ok());
        let save = file.save(Path::new("test.json"));
        assert!(save.is_ok());
    }

    #[test]
    fn create_and_open() {
        create();
        let file = MangoFile::open(Path::new("test.json"));
        assert_eq!(file.unwrap().get_meta().title, Some("test".to_string()));
    }

    // TODO move tests below to base64_image.rs
    #[test]
    #[cfg(feature = "aes")]
    fn encrypt() {
        let mut file = MangoFile::new();
        file.add_image_by_path(Path::new("test.jpg"));
        let image = file.get_image_mut(0);
        let key = String::from("1234567812345678");
        let encrypted_image = image.clone().encrypt(encryption::EncryptionType::AES128, key.clone());
        let decrypted_image = encrypted_image.unwrap().decrypt(key).unwrap();

        assert_eq!(image.get_image_data(), decrypted_image.get_image_data());
    }

    #[test]
    fn save() {
        let mut file = MangoFile::new();
        let added = file.add_image_by_path(Path::new("test.jpg"));
        assert!(added.is_ok());

        let image = file.get_image_mut(0);
        let save = image.save("test_unencrypted.jpg");
        assert!(save.is_ok());
    }

    fn get_full_file() -> MangoFile {
        use compression::CompressionType;
        use encryption::EncryptionType;
        use image::{MangoImage, ImageFile};

        let mut file = MangoFile::new();
        let mut img = MangoImage::from_file(&mut ImageFile::open(Path::new("test.jpg")).unwrap());
        img.compress_mut(CompressionType::GZIP);
        img.encrypt_mut(EncryptionType::AES256, String::from("1234567812345678"));
        file.add_image(img);
        file
    }

    #[test]
    fn save_cbor() {
        let file = get_full_file();
        let save = file.save_cbor(Path::new("save.cbor"));
        assert!(save.is_ok());

    }

    #[test]
    fn  save_json() {
        let file = get_full_file();
        let save = file.save_json(Path::new("save.json"));
        assert!(save.is_ok());
    }

    #[test]
    fn  save_bson() {
        let file = get_full_file();
        let save = file.save_bson(Path::new("save.bson"));
        assert!(save.is_ok());
    }
}
