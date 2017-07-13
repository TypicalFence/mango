use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use base64::encode;
use flate2::Compression;
use flate2::write::GzEncoder;

pub fn img_to_base64(p: &Path) -> String {
    let mut file = File::open(p).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let muh_base64 = encode(&vec);
    muh_base64.replace("\r\n", "")
}

pub fn img_compress_to_base64(p: &Path) -> String {
    let mut file = File::open(p).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let mut e = GzEncoder::new(Vec::new(), Compression::Best);
    e.write(&vec).unwrap();
    let compressed = e.finish().unwrap();
    let muh_base64 = encode(&compressed);
    muh_base64.replace("\r\n", "")
}
