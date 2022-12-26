use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;
use std::ops::Not;
use nom::AsChar;
use crate::util::{get_input_as_stream, get_input_as_string};

pub fn run(path: &str) {

    let input_as_string = get_input_as_string(path);
    let mut chars: Vec<char> = vec![];
    let mut positions: Vec<usize> = vec![];
    // pjbjvjtjlj

    for (i, e) in input_as_string.chars().enumerate() {
        println!("{i} - {e}");
        if chars.len() == 3 && chars.contains(&e).not() {
            chars.push(e);
            positions.push(i);
            break;
        } else if chars.contains(&e) {
            chars.clear();
            positions.clear();
            chars.push(e);
            positions.push(i);
        } else {
            chars.push(e);
            positions.push(i);
        }
    }

    println!("6TH: position is {}", positions.last().unwrap() + 1 )



}
