use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;
use std::ops::Not;
use crate::util::{get_input_as_stream, get_input_as_string};

pub fn run(path: &str) {
    let input_as_string = get_input_as_string(path);
    run_parse(&input_as_string, 4);
    run_parse(&input_as_string, 14);
}


fn run_parse(input: &str, len_of_seq: usize) -> usize {
    let mut result: usize = 0;
    let input_slice = input.chars().collect::<Vec<char>>();
    for (index, element) in input_slice.windows(len_of_seq).enumerate() {

        //println!("{} - {:?}", index+len_of_seq, element);
        if let Some(n) = has_no_duplicates(index, element, len_of_seq) {
            println!("6TH: sequence found at {n} is {element:?}");
            result = n;
            break;
        }
    }

    result
}

fn has_no_duplicates(index: usize, element: &[char], len_of_seq: usize) -> Option<usize> {
    let as_set: HashSet<&char> = element.iter().collect();
    match as_set.len() {
        l if l == len_of_seq => Some(index + len_of_seq),
        _ => None
    }
}


#[cfg(test)]
mod tests {
    use crate::sixth::run_parse;

    #[test]
    fn test_short_seq_abcabcde_return_seven() {
        assert_eq!(run_parse("abcabcde", 4), 7);
    }

    #[test]
    fn test_long_seq_abcabcdefghijklmnopqrstuvz_return_seventeen() {
        assert_eq!(run_parse("zgczgchtfwqjsdlnb", 14), 17);
    }
}