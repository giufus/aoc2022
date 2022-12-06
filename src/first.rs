
use std::fs;
use std::path::Path;

pub fn run(path: &str) {
    match Path::exists(Path::new(path)) {
        true => {
            let file_content_as_string = fs::read_to_string(path);
            if let Ok(s) = file_content_as_string {
                let mut totals: Vec<usize> = vec![];
                let split = s.split("\n\n");
                for single in split {
                    let tot: usize = single.split("\n").map(|v|v.parse::<usize>().unwrap()).sum();
                    totals.push(tot);
                }
                let max = totals.iter().max().unwrap();
                println!("Max is {:?}", max);
                
            } 
        },
        false => println!("Path {} does not exists", path),
    }
}

