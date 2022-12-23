use std::cmp::{max, min};
use std::collections::{VecDeque, HashMap};
use std::collections::hash_map::Entry;
use std::fmt::{Display, Formatter, write};

use crate::util::get_input_as_string;

/*

 | [L]     [H] [W]
 | [J] [Z] [J] [Q] [Q]
 | [S]             [M] [C] [T] [F] [B]
 | [P]     [H]     [B] [D] [G] [B] [P]
 | [W]     [L] [D] [D] [J] [W] [T] [C]
 | [N] [T] [R] [T] [T] [T] [M] [M] [G]
 | [J] [S] [Q] [S] [Z] [W] [P] [G] [D]
 | [Z] [G] [V] [V] [Q] [M] [L] [N] [R]
 | 1   2   3   4   5   6   7   8   9
--------------------------------------
n| 0123456789..12..16

n = 1 -> 0
n = 2 -> 4 
n = 3 -> 8
n = 4 -> 12
n = 5 -> 16
....  

*/

pub fn run(path: &str) {

    let input = get_input_as_string(path);
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();
    let commands_start: usize;
    for (n, l) in input.lines().enumerate() {
        let parsing_line = parse_matrix(l, n, &mut stacks);
        if let (Err(p)) = parsing_line {
            println!("Matrix parsed, exit at line {p}");
            commands_start = p;
            break;
        }
    }

    for (n, l) in input.lines().enumerate() {
        parse_command(l, n, &mut stacks);
    }


    let result: String = (1..10).into_iter()
        .map(|f| stacks.get(&f))
        .map(|f| f.unwrap().front().unwrap())
        .collect::<String>();

    println!("5TH: {:?}", result);

    let m: Matrix = Matrix {stks: stacks};
    //println!("{:?}", m);
}

fn parse_matrix(line: &str, number: usize, stacks: &mut HashMap<usize, VecDeque<char>>) -> Result<String, usize> {

    let line_vect = line.chars().collect::<Vec<char>>();

    if line.starts_with('[') {
        for (i, v) in line_vect.iter().enumerate() {

            let elem_num = i / 4 + 1;
            if *v == '[' {
                //let stack_option: Option<&mut VecDeque<char>> = stacks.get_mut(&key);
                let mut stack: &mut VecDeque<char> = match stacks.entry(elem_num) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(VecDeque::new()),
                };

                /*if stack_option.is_none {
                    stack = &mut VecDeque::new();
                } else {
                    stack = stack_option.unwrap();
                }*/

                let elem_lett = line_vect.get(i+1).unwrap();
                // println!("stack {} item {}", elem_num, elem_lett);
                stack.push_back(*elem_lett);
                //stacks.insert(key, stack);
            }

        }
        Ok(String::from(format!("Line {number} parsing completed")))
    } else{
        Err(number)
    }
}


fn parse_command(line: &str, number: usize, stacks: &mut HashMap<usize, VecDeque<char>>) {

    if line.starts_with("move") {
        let filtered: Vec<usize> = line.split(" ")
            .map(|f| f.trim().to_string())
            .filter(|f| f.parse::<usize>().is_ok())
            .map(|f| f.parse::<usize>().unwrap())
            .collect();


        let source_key = filtered.get(1).unwrap();
        let destination_key = filtered.get(2).unwrap();

        //dbg!(source_key);
        //dbg!(destination_key);
        let mut clone1 = stacks.clone();
        let mut clone2 = stacks.clone();

        let source = clone1.get_mut(source_key).unwrap();
        let destination = clone2.get_mut(destination_key).unwrap();
        //println!("{source:?}");
        //println!("{destination:?}");

        let to_move = filtered.get(0).unwrap();
        let source_len = source.len();
        let items_number = min(to_move, &source_len);
        //println!("min({},{}) is {}", to_move, source_len, items_number);


        for _ in 0..*items_number {
            let popped = source.pop_front().unwrap();
            destination.push_front(popped);
        }

        //println!("{source:?}");
        //println!("{destination:?}");

    }
}

#[derive(Debug)]
struct Matrix {
    stks: HashMap <usize, VecDeque<char>>
}

/*
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut keys = self.stks.keys().collect::<Vec<_>>();
        keys.sort();

        let mut fin: String = format!("{:?}", &keys);
        let mut tmp: Vec<char> = vec![];
        let mut clone1 = self.stks.clone();

        while (&clone1).into_iter().map(|f|f.1).flatten().count().gt(&0)  {
            let clone2 = clone1.clone();
            for k in keys {
                let elem = clone2.get(k).unwrap().pop_back().unwrap();
                tmp.push(elem);
            };

            fin.push('\n');
            fin.push_str(tmp.clone().iter().collect::<String>().as_str());

        };

        write!(f, "{}", fin)

    }
}
*/