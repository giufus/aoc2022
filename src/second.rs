/*

--- Day 2: Rock Paper Scissors ---
The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

Your puzzle answer was 10994.

--- Part Two ---
The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

Your puzzle answer was 12526.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

You can also [Share] this puzzle.


COLUMN 1     |     COLUMN 2             |   PART TWO
---------------------------
A = ROCK     | X = ROCK    | POINTS: 1  |   X = I LOSE
B = PAPER    | Y = PAPER   | POINTS: 2  |   Y = WE DRAW
C = SCISSOR  | Z = SCISSOR | POINTS: 3  |   Z = I WIN
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
    let mut second_acc = 0;
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

                    let trick = trick(&left, &right);
                    let new_part_score: i64 = me.get(trick.as_str()).unwrap().as_i64().unwrap();
                    // println!("get is {get}");
                    second_acc += new_part_score;
                    match trick.as_str() {
                        "X" => second_acc += 1,
                        "Y" => second_acc += 2,
                        "Z" => second_acc += 3,
                        _ => {}
                    }
                }
            }
        }
        false => println!("Path {} does not exists", path),
    }

    println!("2nd: FINAL SCORE IS {acc}");
    println!("2nd: TRICKED FINAL SCORE IS {second_acc}");
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

    let from_str = serde_json::from_str(matrix);
    match from_str {
        Ok(m) => m,
        Err(_) => panic!("cannot initialize score matrix from {matrix}" ),
    }
}

/**
 * get a trick value for right 
 */
fn trick(oppo: &str, me: &str) -> String {

    match oppo {
        "A" => {
            match me {
                "X" => "Z",
                "Y" => "X",
                "Z" => "Y",
                _ => me,
            }
        },
        "B" => {
            match me {
                "X" => "X",
                "Y" => "Y",
                "Z" => "Z",
                _ => me,
            }
        },
        "C" => {
            match me {
                "X" => "Y",
                "Y" => "Z",
                "Z" => "X",
                _ => me,
            }
        },
        _  => {
            match me {
                _ => me,
            }
        },
    }.to_string()
}