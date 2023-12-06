use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    //let _test_races = vec![Race::new(7,9), Race::new(15,40), Race::new(30, 200)];
    //let _test_races2 = Race::new(71530, 940200);
    let races = read_to_string("input/days/day06_part1.txt").unwrap().lines().map(Race::from).collect::<Vec<Race>>();
    let num_wins = races.into_iter().map(|r| r.find_num_wins()).collect::<Vec<u64>>();

    let races2 =Race::from(read_to_string("input/days/day06_part2.txt").unwrap().as_str());
    
    let sol1:u64 = num_wins.into_iter().product();
    let sol2 = races2.find_num_wins();

    (Solution::from(sol1), Solution::from(sol2))
}
impl From<&str> for Race {
    fn from(input:&str) -> Self {
        let (t_str, d_str) = input.split_once(",").unwrap();
        Race::new(t_str.parse::<u64>().unwrap(), d_str.parse::<u64>().unwrap())
    }
}
struct Race {
    time: u64,
    record_distance: u64
}
impl Race {
    fn new(time: u64, record_distance:u64) -> Race {
        Race {
            time,
            record_distance
        }
    }
    fn find_num_wins(&self) -> u64 {
        let mut wins = 0;
        for i in 1..self.time {
            let distance = (self.time - i) * i;
            if distance > self.record_distance {
                wins += 1;
            }
        }
        wins
    }
}