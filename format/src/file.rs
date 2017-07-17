use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use compress::Gz;
use image::{FileImage, Base64Image};

#[derive(Serialize, Deserialize)]
pub struct MangoFile {
    name: String,
    images: Vec<Base64Image>,
}

impl MangoFile {
    pub fn new(name: String) -> MangoFile {
        MangoFile {
            name: name,
            //TODO change to Base64Image
            images: Vec::new(),
        }
    }

    pub fn new_from_file(p: &Path) -> MangoFile {
        unimplemented!()
    }

    pub fn save(&self, p: &Path) {
        //TODO error handling
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        let mut f = File::create(p).expect("Unable to create file");
        f.write_all(json_string.as_bytes()).expect(
            "Unable to write data",
        );
    }

    //TODO error handling
    pub fn add_image(&mut self, p: &Path) {
        let compressor = Gz::new();
        let mut image_file = FileImage::open(p).unwrap();
        self.images.push(
            image_file.to_base64().compress(&compressor),
        );
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn set_name(&mut self, n: String) {
        self.name = n;
    }
}

#[cfg(test)]
mod tests {
    use super::MangoFile;
    use std::path::Path;

    #[test]
    fn create() {
        let mut file = MangoFile::new("test".to_string());
        file.add_image(Path::new("test.jpg"));
        file.save(Path::new("test.json"));
    }
}
