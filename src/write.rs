use std::io::prelude::*;
use std::path::Path;
use std::fs;
use std::fs::File;

#[macro_use]
use serde_derive::Serialize;

use crate::temperature::Temperature;

/// Data structure that holds the write bits together
#[derive(Serialize)]
pub struct TempWrite<'a> {
    file_path: &'a Path,
    input: Temperature,
    encoding: String,
}

enum FileFormat {
    Json,
    Yaml,
    Toml
}

/// Allows user to write to any kind of searializable format said user wants
/// e.g. JSON, YAML, TOML, etc.
/// Note that this behavior is not yet implemented!
trait Writer {
    fn write_to_file(&self);
    fn to_file(&self);
}

impl<'a> TempWrite<'a> {
    pub fn new(t: Temperature) -> TempWrite<'a> {
        TempWrite {
            file_path: Path::new("export.json"),
            input: t,
            encoding: "json".to_string(),
        }
    }

    pub fn write_to_json(&self) {
        let result = self.does_file_exist();

        if result == false {
            self.create_file();
        } else {
            self.delete_file();
            self.create_file();
        }

        self.to_json();
    }

    fn to_json(&self) {
        let serialized = serde_json::to_string(&self.input).expect("Could not serialize into JSON!");
        let mut file = File::open(self.file_path).expect("File does not exist!");
        let x = file.write_all(serialized.as_bytes());
        
        match x {
            Ok(_) => (),
            Err(_) => panic!("Could not write to file!")
        }
    }

    fn does_file_exist(&self) -> bool {
        self.file_path.exists()
    }

    fn create_file(&self) -> File {
        File::create(self.file_path).unwrap()
    }

    fn delete_file(&self) {
        fs::remove_file(self.file_path).unwrap();
    }
}