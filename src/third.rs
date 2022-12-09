

/*

Priorities
|---------------|
| a..z, A..Z    |
| 1..26, 27..52 |
|---------------|

*/
use crate::util;
use std::{collections::HashSet};

pub fn run(path: &str) { 

    let file_content_as_string = util::get_input_as_string(path);
    let mut tot: u32 = 0;
    let split = file_content_as_string.split("\n");
    for s in split {
        let s_len = s.len();
        let first_half  = &s[0..s_len/2];
        let second_half = &s[s_len/2..];

        let set1: HashSet<char> = first_half.chars().collect();
        let set2: HashSet<char> = second_half.chars().collect();

        let intersection = set1.intersection(&set2);

        
        // println!("{set1:?} - {set2:?} ---> {intersection:?}"); 

        let fold = intersection.fold(0, |acc, x| acc + get_priority_v2(&x));

        tot += fold;

        // println!("PARTIAL: {fold} ; TOTAL: {tot}");
    }

    println!("3rd: TOT PRIORITY IS {:?}", tot);

}

// it does not work, use the other
fn get_priority_v1(c: char) -> u32 {
    let digit = c.to_digit(10).unwrap_or(0);
    // println!("PRIORITY of {} is {}", c, digit);
    digit
}

/*
 A - DECIMAL UNICODE is 65 --> 65 - X = 27 --> X = 38
 a - DECIMAL UNICODE is 97 --> 97 - X = 1 --> X = 96 
*/
fn get_priority_v2(c: &char) -> u32 {

    let digit = *c as u32;

    let scaled = match digit {
        a if c.to_owned().is_lowercase() => digit - 96,
        A if c.to_owned().is_uppercase() => digit - 38,
        _ => 0,
    };

    // println!("PRIORITY of {} is {}, scaled is {}", c, digit, scaled);
    
    scaled
}