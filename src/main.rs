#![feature(iter_array_chunks)]
mod first;
mod second;
mod third;
mod fourth;
mod fifth;
mod sixth;
mod util;

use std::env;

fn main() {
    println!("Advento of Code 2022");

    let args: Vec<String> = env::args().collect();

    let first_arg = args.get(1);
    let day: i8 = match first_arg {
        Some(n) => {
            let to_num = n.parse::<i8>();
            match to_num {
                Ok(num) if num <= 25 => num,
                Ok(_) => panic!("Max number is 25!"),
                Err(_) => 0,
            }
        }
        None => 0,
    };

    if day > 0 {
        let adv_day: Advent = Advent::get_enum_by_number(day);
        let adv_input = adv_day.get_input();

        match adv_day {
            Advent::First => {
                first::run(adv_input);
                first::run_v2(adv_input);
            }
            Advent::Second => second::run(adv_input),
            Advent::Third => third::run(adv_input),
            Advent::Fourth => fourth::run(adv_input),
            Advent::Fifth => fifth::run(adv_input),
            Advent::Sixth => sixth::run(adv_input),
            Advent::Seventh => todo!(),
            Advent::Eighth => todo!(),
            Advent::Ninth => todo!(),
            Advent::Tenth => todo!(),
            Advent::Eleventh => todo!(),
            Advent::Twelfth => todo!(),
            Advent::Thirteenth => todo!(),
            Advent::Fourteenth => todo!(),
            Advent::Fifteenth => todo!(),
            Advent::Sixteenth => todo!(),
            Advent::Seventeenth => todo!(),
            Advent::Eighteenth => todo!(),
            Advent::Nineteenth => todo!(),
            Advent::Twentieth => todo!(),
            Advent::TwentyFirst => todo!(),
            Advent::TwentySecond => todo!(),
            Advent::TwentyThird => todo!(),
            Advent::TwentyFourth => todo!(),
            Advent::TwentyFifth => todo!(),
        }
    } else {
        first::run(Advent::First.get_input());
        first::run_v2(Advent::First.get_input());
        second::run(Advent::Second.get_input());
        third::run(Advent::Third.get_input());
        fourth::run(Advent::Fourth.get_input());
        fifth::run(Advent::Fifth.get_input());
        sixth::run(Advent::Sixth.get_input());
    }
}

enum Advent {
    First = 1,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
    Thirteenth,
    Fourteenth,
    Fifteenth,
    Sixteenth,
    Seventeenth,
    Eighteenth,
    Nineteenth,
    Twentieth,
    TwentyFirst,
    TwentySecond,
    TwentyThird,
    TwentyFourth,
    TwentyFifth,
}

impl Advent {
    fn get_enum_by_number(num: i8) -> Self {
        match num {
            1 => Advent::First,
            2 => Advent::Second,
            3 => Advent::Third,
            4 => Advent::Fourth,
            5 => Advent::Fifth,
            6 => Advent::Sixth,
            7 => Advent::Seventh,
            8 => Advent::Eighth,
            9 => Advent::Ninth,
            10 => Advent::Tenth,
            11 => Advent::Eleventh,
            12 => Advent::Twelfth,
            13 => Advent::Thirteenth,
            14 => Advent::Fourteenth,
            15 => Advent::Fifteenth,
            16 => Advent::Sixteenth,
            17 => Advent::Seventeenth,
            18 => Advent::Eighteenth,
            19 => Advent::Nineteenth,
            20 => Advent::Twentieth,
            21 => Advent::TwentyFirst,
            22 => Advent::TwentySecond,
            23 => Advent::TwentyThird,
            24 => Advent::TwentyFourth,
            25 => Advent::TwentyFifth,
            _ => panic!("Max number is 25!"),
        }
    }

    fn get_input(&self) -> &str {
        match *self {
            Advent::First => "src/inputs/first_input.json",
            Advent::Second => "src/inputs/second_input.json",
            Advent::Third => "src/inputs/third_input.json",
            Advent::Fourth => "src/inputs/fourth_input.json",
            Advent::Fifth => "src/inputs/fifth_input.json",
            Advent::Sixth => "src/inputs/sixth_input.json",
            Advent::Seventh => todo!(),
            Advent::Eighth => todo!(),
            Advent::Ninth => todo!(),
            Advent::Tenth => todo!(),
            Advent::Eleventh => todo!(),
            Advent::Twelfth => todo!(),
            Advent::Thirteenth => todo!(),
            Advent::Fourteenth => todo!(),
            Advent::Fifteenth => todo!(),
            Advent::Sixteenth => todo!(),
            Advent::Seventeenth => todo!(),
            Advent::Eighteenth => todo!(),
            Advent::Nineteenth => todo!(),
            Advent::Twentieth => todo!(),
            Advent::TwentyFirst => todo!(),
            Advent::TwentySecond => todo!(),
            Advent::TwentyThird => todo!(),
            Advent::TwentyFourth => todo!(),
            Advent::TwentyFifth => todo!(),
        }
    }
}
