use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let mut sol1: i64 = 0;
    let mut sol2: i64 = 0;
    let input = read_to_string("input/days/day09.txt").unwrap();
    for line in input.lines() {
        let mut nums = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
        sol1 += just_go_until_zeroes(nums.clone());
        nums.reverse();
        sol2 += just_go_until_zeroes(nums);      
    }
    (Solution::from(sol1), Solution::from(sol2))
}
fn just_go_until_zeroes(nums: Vec<i64>) -> i64 {
    let mut new_nums = Vec::new();
    let last_number = nums.last().unwrap();
    for (current,next) in nums.iter().zip(nums.iter().skip(1)) {
        new_nums.push(*next - *current);

    }
    if new_nums.iter().all(|n| *n == 0) {
        return *last_number;
    }
    last_number + just_go_until_zeroes(new_nums)
}