use crate::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day03.txt").unwrap();
    let rows = input.lines();
    //input file into 2d vector
    let mut data: Vec<Vec<char>> = Vec::new();
    for row in rows {
        let mut chars: Vec<char> = Vec::new();
        for c in row.chars() {
            chars.push(c);
        }
        data.push(chars);
    }
    let grid = Grid::new(data);
    let (parts, gear_parts) = grid.find_parts();
    let sol1 = parts.into_iter().reduce(|a, b| a + b).unwrap_or(0);
    let mut sol2: u32 = 0;
    for (_, parts) in gear_parts {
        if parts.len() == 2 {
            sol2 += parts.into_iter().product::<u32>();
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

struct Grid {
    data: Vec<Vec<char>>,
    row_size: usize,
    col_size: usize,
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Grid {
        let row_size = data.len();
        let col_size = data[0].len();
        Grid {
            data,
            row_size,
            col_size,
        }
    }
    fn is_valid_part(&self, r: i32, c: i32) -> (bool, HashSet<(i32, i32)>) {
        let mut is_valid = false;
        let mut gears: HashSet<(i32, i32)> = HashSet::new();

        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let r_index = r + i;
                let c_index = c + j;

                if r_index < 0
                    || r_index >= self.row_size as i32
                    || c_index < 0
                    || c_index >= self.col_size as i32
                {
                    continue;
                }
                let curr_char = self.data[r_index as usize][c_index as usize];
                if curr_char != '.' && !curr_char.is_digit(10) {
                    if curr_char == '*' {
                        gears.insert((r_index, c_index));
                    }

                    is_valid = true;
                }
            }
        }
        (is_valid, gears)
    }
    fn find_parts(&self) -> (Vec<u32>, HashMap<(i32, i32), Vec<u32>>) {
        let mut parts: Vec<u32> = Vec::new();
        let mut runningnumber: u32 = 0;
        let mut is_valid_part = false;
        let mut r_index = 0;
        let mut gears = HashSet::new();
        let mut gears_parts: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
        for row in &self.data {
            let mut c_index = 0;
            for c in row {
                if c.is_numeric() {
                    let (is_valid, added_gears) = self.is_valid_part(r_index, c_index);
                    if is_valid {
                        is_valid_part = true;
                        gears.extend(added_gears);
                    }
                    runningnumber = runningnumber * 10 + c.to_digit(10).unwrap();
                }

                if c_index as usize == self.col_size - 1 && is_valid_part && runningnumber != 0 {
                    parts.push(runningnumber);

                    for gear in &gears {
                        gears_parts
                            .entry(*gear)
                            .or_insert(Vec::new())
                            .push(runningnumber);
                    }
                    runningnumber = 0;
                    is_valid_part = false;
                    gears.clear();
                } else if is_valid_part && runningnumber != 0 && !c.is_numeric() {
                    parts.push(runningnumber);
                    for gear in &gears {
                        gears_parts
                            .entry(*gear)
                            .or_insert(Vec::new())
                            .push(runningnumber);
                    }
                    runningnumber = 0;
                    is_valid_part = false;
                    gears.clear();
                } else if c_index as usize == self.col_size - 1 {
                    runningnumber = 0;
                    is_valid_part = false;
                    gears.clear();
                } else if runningnumber != 0 && !c.is_numeric() {
                    runningnumber = 0;
                    is_valid_part = false;
                    gears.clear();
                }

                c_index += 1;
            }
            r_index += 1;
        }
        (parts, gears_parts)
    }
}
