use std::fs;
use std::path::Path;

pub fn run(path: &str) {
    match Path::exists(Path::new(path)) {
        true => {
            let file_content_as_string = fs::read_to_string(path);
            if let Ok(s) = file_content_as_string {
                let mut totals: Vec<usize> = vec![];
                let split = s.split("\n\n");
                for single in split.clone() {
                    let tot: usize = single
                        .split("\n")
                        .map(|v| v.parse::<usize>().unwrap())
                        .sum();
                    totals.push(tot);
                }

                let max = totals.clone().into_iter().max().unwrap();
                totals.sort();
                totals.reverse();
                let top_three_sum: usize = totals[0..3].iter().sum();
                println!("1st: MAX IS {:?}", max);
                println!(
                    "1st: TOP 3 ARE {:?}, SUM IS {:?}",
                    totals[0..3].to_vec(),
                    top_three_sum
                );
            }
        }
        false => println!("Path {} does not exists", path),
    }
}
