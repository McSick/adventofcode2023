use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use regex_lite::Regex;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day01.txt").unwrap();
    /* Solution 1 */
    let mut sol1: i32 = 0;
    for line in input.lines() {
        //loop over each character in the line
        let mut first = -1;
        let mut second = -1;
        for c in line.chars() {
            let some_digit =  match c {
                '0'..='9' => Some(c.to_digit(10).unwrap() as i32),     
                _ => None
            };
            if let Some(d) = some_digit {
                if first == -1 {
                    first = d;
                    second = d;
                } else {
                    second = d;
                } 
            }
   
        }
        let result = first * 10 + second;
        sol1 += result;
    }
    /* solution 2 */
    let mut sol2: i32 = 0;
    for line in input.lines() {
        let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d{1})").unwrap();
        let rebackwards = Regex::new(r"(\d{1}|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
        
        let f_match = re.find(line).unwrap().as_str();
        let line_backwards = line.chars().rev().collect::<String>();
        let b_match = rebackwards.find(line_backwards.as_str()).unwrap().as_str();

        let first = as_digit(f_match);
        let second = as_digit(b_match);

        let result = first * 10 + second; 
        sol2 += result;
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn as_digit(word: &str) -> i32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "enin" => 9,
        "thgie" => 8,
        "neves" => 7,
        "xis" => 6,
        "evif" => 5,
        "ruof" => 4,
        "eerht" => 3,
        "owt" => 2,
        "eno" => 1,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => panic!("Unexpected word: {}", word)
    }
}