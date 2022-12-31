use std::char;
use std::collections::HashMap;
use std::io::BufRead;
use std::iter::{StepBy, Sum};
use std::ops::{Deref, Not, Range};

use crate::util::{get_input_as_stream, get_input_as_string};

pub fn run(path: &str) {
    let input_as_stream = get_input_as_stream(path);

    // parse input to hashmap K: (x,y) V: v
    let mut matrix_map: HashMap<(u32, u32), u32> = HashMap::new();

    let mut rows_num: u32 = 0;
    let mut cols_num: u32 = 0;

    input_as_stream.lines().enumerate()
        .for_each(|(ind, str)| {
            rows_num += 1;
            str.unwrap().chars().enumerate()
                .for_each(|(ind_2, num)| {
                    cols_num += 1;
                    matrix_map.insert((ind as u32, ind_2 as u32), num.to_digit(10).unwrap());
                })
        });

    cols_num = cols_num / rows_num;
    let mut tot: u32 = cols_num + rows_num-1 + cols_num-1 + rows_num - 2;

    // println!("dimensions are {rows_num} : {cols_num}");
    // println!("border positions are {tot}");

    // (0,0) (0,1) (0,2) (0,3).... (0,98)
    // (1,0) (1,1) (1,2) (1,3).... (1,98)
    // (2,0) (2,1) (2,2) (2,3).... (2,98)
    // (3,0) (3,1) (3,2) (3,3).... (2,98)


    // for each key not on the border get value, the item is visible if same row/column values are < than itself
    matrix_map.keys()
        .filter(|(k1, k2)| is_on_the_edge(k1, k2, (&rows_num, &cols_num)).not())
        .for_each(|(k1, k2)| {
            //println!("NOT ON THE EDGE {} {} value is {}", k1, k2, matrix_map.get(&(*k1, *k2)).unwrap());
            // left visibility es. 2,3 -> all the elements to the left of (2,3) must have value < than the value of (2,3)
            let left = (0..*k2).into_iter()
                .map(|i| matrix_map.get(&(*k1, i)).unwrap() < matrix_map.get(&(*k1, *k2)).unwrap())
                .all(|f| true.eq(&f));
            //println!("LEFT OF  {} {} is {}", k1, k2, left);

            // right
            let right = (*k2+1..cols_num).into_iter()
                .map(|i| matrix_map.get(&(*k1, i)).unwrap() < matrix_map.get(&(*k1, *k2)).unwrap())
                .all(|f| true.eq(&f));
            //println!("RIGHT OF  {} {} is {:?}", k1, k2, right);

            // top
            let top = (0..*k1).into_iter()
                .map(|i| matrix_map.get(&(i, *k2)).unwrap() < matrix_map.get(&(*k1, *k2)).unwrap())
                .all(|f| true.eq(&f));
            //println!("TOP OF  {} {} is {:?}", k1, k2, top);

            // bottom
            let bottom = (*k1+1..rows_num).into_iter()
                .map(|i| matrix_map.get(&(i, *k2)).unwrap() < matrix_map.get(&(*k1, *k2)).unwrap())
                .all(|f| true.eq(&f));
            //println!("BOTTOM OF  {} {} is {:?}", k1, k2, bottom);

            tot += if vec![left, right, top, bottom].iter().any(|f| true.eq(f)) { 1 } else { 0 };

        });



    println!("8TH: visible trees are {:?}", tot);
}


fn is_on_the_edge(x: &u32, y: &u32, dimensions: (&u32, &u32)) -> bool {
    *x == 0 || *y == 0 || *x == *(dimensions.0)-1 || *y == *(dimensions.1)-1
}

