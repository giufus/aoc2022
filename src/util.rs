
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




#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::PathBuf};

    use super::*;


    #[test]
    fn test_file_read_as_string() {
        
        // given
        let path = "temp.txt";

        let mut temp_file = File::create(path).unwrap();
        let write = temp_file.write(b"something");
        

        // when
        match write {
            Ok(_) => {
                let srcdir = PathBuf::from(path);
                println!("PATH IS {:?}", fs::canonicalize(&srcdir));
                let filecontent_as_string = get_input_as_string(path);
                assert_eq!(filecontent_as_string.as_str(), "something");

            },
            Err(_) => panic!("test failed"),
        }
        

        // then
        match fs::remove_file(path) {
            Ok(_) => assert!(true),
            Err(_) => panic!("cannot remove file!"),
        }

    }

}


