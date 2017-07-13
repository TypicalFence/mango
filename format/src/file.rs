use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use json::JsonValue;
use json::object::Object;
use image::FileImage;
use compress::Gz;

pub struct MangoFile {
    name: String,
    images: Vec<String>,
}

impl MangoFile {
    pub fn new(name: String) -> MangoFile {
        MangoFile {
            name: name,
            images: Vec::new(),
        }
    }

    pub fn new_from_file(p: &Path) -> MangoFile {
        unimplemented!()
    }

    pub fn to_json(&self) -> String {
        let mut json_obj: Object = Object::new();

        json_obj.insert("name", JsonValue::from(self.name.clone()));

        let mut images: Vec<String> = Vec::new();
        for img in &self.images {
            images.push(img.clone());
        }
        json_obj.insert("images", JsonValue::from(images));

        JsonValue::from(json_obj).dump()
    }

    pub fn save(&self, p: &Path) {
        let json_string = &self.to_json();
        let mut f = File::create(p).expect("Unable to create file");
        f.write_all(json_string.as_bytes()).expect(
            "Unable to write data",
        );
    }

    pub fn add_image(&mut self, p: &Path) {
        let mut img: FileImage = FileImage::open(p).unwrap();
        let b64 = img.to_base64();
        let comp = Gz::new();
        self.images.push(b64.compress(&comp).get_image())
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
