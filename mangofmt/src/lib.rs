//! Mango is a very cool file format for manga and comics!
//!
//! Its a rather pointless little experiment, solving a problem that wasn't really there.
//! I was always a bit bummed out about the fact, that a format like cbz/cbr doesn't store any metadata
//! and is just a archive with images in it.
//!
//! So I made my own format, which stores everything in binary with metadata.
//!
//! Please **refer to the main ReadMe** in the root of the git repository for more general information.
//!
//! ## API Intro
//! This small part should give a tiny little overview of the crate.
//!
//! The 2 Most important Structs for working with mangofmt are
//! [MangoFile](struct.MangoFile.html) & [MangoImage](struct.MangoImage.html).
//! They represent a *File* which can contain multiple *Images*.
//! Both structs contain their own Metadata, and expose a bunch of methods which allow creation,
//! modification, compression, encryption and much more.
//!
//! There are 2 important modules that are hidden from the outside: **compression** & **encryption**.
//! This is done on purpose. They contain the implementation of optional compression & encryption logic,
//! which can get accessed via the [MangoImage api](struct.MangoImage.html).
//!
//! As mentioned the contents of them are *optional*,
//! there are a bunch of feature flags for controlling what you get and what not.
//!
//! The next Part will describe how to add an additional Kind of Compression or Encryption,
//! like AES or GZIP for instance.
//!
//! ## Adding a Encryption/Compression Type
//!
//! Additional Algorithms can be added in easily.
//!
//! You can use the *aes* and *gzip* features as examples.
//!
//! 1. Create a new feature in the *cargo.toml* and add all your dependencies ad optional dependencies.
//! 2. Depending on what you adding create a file in */src/encryption* or *src/compression* for the Algorithm your adding.
//! 3. Annotate your module with *#[cfg(feature = "your feature")]*
//! 4. Add your algorithm to the Enum which contains all types, do not annotate it tho.
//! 5. Add your type to the match statements with annotations as above.
//!    The code in the file you created earlier should be called there.
//!
//! Pull requests for additional encryption/compressions types are welcome, but please write tests.
//!

extern crate base64;
extern crate sha2;
extern crate hex;
extern crate serde;
extern crate serde_json;
extern crate bson;
extern crate serde_bytes;
extern crate serde_cbor;

#[macro_use]
extern crate serde_derive;

mod compression;
mod encryption;
pub mod meta;
pub mod image;
pub mod file;
mod json;

#[doc(inline)]
pub use compression::{CompressionType, CompressionError};
#[doc(inline)]
pub use encryption::{EncryptionType, EncryptionError};
#[doc(inline)]
pub use image::{ImageFile, MangoImage, Mime};
#[doc(inline)]
pub use file::MangoFile;

