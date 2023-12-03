use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

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
    let parts = grid.find_parts();
    //println!("{:?}", parts);
     let sol1 = parts.into_iter().reduce(|a, b| a + b).unwrap_or(0);
    
    //



    // Your solution here...
    //let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

struct Grid {
    data: Vec<Vec<char>>,
    row_size: usize,
    col_size: usize
}

impl  Grid {
    fn new(data: Vec<Vec<char>>) -> Grid {
        let row_size = data.len();
        let col_size = data[0].len();
        Grid {
            data,
            row_size,
            col_size
        }
    }
    fn is_valid_part(&self, r: i32, c: i32) -> HasSet<char> {
        let mut is_valid = false;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let r_index = r + i;
                let c_index = c + j;
         
                if r_index < 0 || r_index >= self.row_size as i32 || c_index < 0 || c_index >= self.col_size as i32 {
                    continue;
                }
                let curr_char = self.data[r_index as usize][c_index as usize];
                if curr_char != '.'  && !curr_char.is_digit(10) {
                    is_valid = true

                }
                //println!("r:{} c:{} r_i:{} c_i: {} {} {}", r, c, r_index, c_index, curr_char, is_valid);

            }
        }
        is_valid
    }
    fn find_parts(&self) -> Vec<u32> {
        let mut parts: Vec<u32> = Vec::new();
        let mut runningnumber:u32 = 0;
        let mut is_valid_part = false;
        let mut r_index = 0;
        for row in &self.data {
            let mut c_index = 0;
            for c in row {

                if c.is_numeric() {
                    if self.is_valid_part(r_index, c_index) {
                        is_valid_part = true;
                    }
                    runningnumber = runningnumber * 10 + c.to_digit(10).unwrap();
                } 

                if c_index as usize == self.col_size - 1 && is_valid_part && runningnumber != 0 {
                    parts.push(runningnumber);
                    runningnumber = 0;
                    is_valid_part = false;
                } else if  is_valid_part && runningnumber != 0 && !c.is_numeric() {
                    parts.push(runningnumber);
                    runningnumber = 0;
                    is_valid_part = false;
                } else if c_index as usize == self.col_size - 1 {
                    runningnumber = 0;
                    is_valid_part = false;
                } else if runningnumber != 0 && !c.is_numeric() {
                    runningnumber = 0;
                    is_valid_part = false;
                }
                
                c_index += 1;
            }
            r_index += 1;
  
        }
        parts
    }
}
