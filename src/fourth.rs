/*

        Es.   2 - 7 , 3 - 5
              5 - 8 , 4 - 8
        x1 <= y2 && y1 <= x2

        2 <= 5  &&  3 <= 7
        5 <= 8  &&  4 <= 8
*/

use std::{io::BufRead, fmt::Display};

use crate::util::{get_input_as_stream, get_input_as_string};

pub fn run(path: &str) {
    let get_input_as_stream = get_input_as_stream(path);

    let inclusives: usize  = get_input_as_stream.lines()
        .map(|f| f.unwrap_or(String::from("0-0,99-99")))
        .map(|f| f.split(",").zip(f.split(",").skip(1))
                          .map(|g|(Assignment::from(g.0), Assignment::from(g.1))).next())
        .map(|f| overlaps(f.unwrap()))
        .filter(|f| true.eq(f))
        .count();

        println!("4th: Included assignments are {inclusives}")
}


#[derive(Debug)]
struct Assignment {
    min: String,
    max: String
}

fn overlaps(input: (Assignment, Assignment)) -> bool {
    let result = input.0.min.le(&input.1.max) && input.1.min.le(&input.0.max);
    // println!("{} {} overlaps: {}", input.0, input.1, result);
    result
}

impl From<&str> for Assignment {
    fn from(value: &str) -> Self {
        let assignments = value.split("-").zip(value.split("-").skip(1))
            .map(|item| Assignment {min: item.0.to_string(), max: item.1.to_string()})
            .next().unwrap();
        assignments
    }
}

impl  Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}