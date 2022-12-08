/*

COLUMN 1     |     COLUMN 2
---------------------------
A = ROCK     | X = ROCK    | POINTS: 1
B = PAPER    | Y = PAPER   | POINTS: 2
C = SCISSOR  | Z = SCISSOR | POINTS: 3
---------------------------

  | A  B  C |-> opponent
--|---------|-
X | 3  0  6 |
Y | 6  3  0 |
Z | 0  6  3 |
--|---------|-
|
V

m
e
*/

use serde_json::{Result, Value};
use std::collections::HashMap;
use std::path::Path;
use std::{any, fs};

pub fn run(path: &str) {
    let matrix = init_score();
    let mut acc = 0;
    match Path::exists(Path::new(path)) {
        true => {
            let file_content_as_string = fs::read_to_string(path);
            if let Ok(s) = file_content_as_string {
                for row in s.split("\n") {
                    let row_split = row.split(" ").collect::<Vec<&str>>();
                    let left = *(row_split.get(0).unwrap());
                    let right = *(row_split.get(1).unwrap());
                    // println!("{left} and {right}");
                    let me = matrix.get(left).unwrap().as_object().unwrap();
                    // println!("me is {me:?}");
                    let part_score: i64 = me.get(right).unwrap().as_i64().unwrap();
                    // println!("get is {get}");
                    acc += part_score;
                    match right {
                        "X" => acc += 1,
                        "Y" => acc += 2,
                        "Z" => acc += 3,
                        _ => {}
                    }
                }
            }
        }
        false => println!("Path {} does not exists", path),
    }

    println!("2nd: FINAL SCORE IS {acc}");
}

fn init_score() -> HashMap<String, Value> {
    let matrix = r#"{
        "A": {
            "X": 3,
            "Y": 6,
            "Z": 0
        },
        "B": {
            "X": 0,
            "Y": 3,
            "Z": 6
        },
        "C": {
            "X": 6,
            "Y": 0,
            "Z": 3
        }
    }"#;

    serde_json::from_str(matrix).unwrap()
}
