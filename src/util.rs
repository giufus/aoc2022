
use std::path::Path;
use std::{fs};

pub fn get_input_as_string(path: &str) -> String {
    match Path::exists(Path::new(path)) {
        true => {
            fs::read_to_string(path).unwrap()
        },
        false => panic!("Path {} does not exists", path),
    }
}