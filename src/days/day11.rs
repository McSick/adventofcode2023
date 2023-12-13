use std::{cmp, collections::HashSet, fmt};


use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day11.txt").expect("Could not read file");
    let mut galaxy = parse_input(input);
    let distances = find_shortest_distances(&galaxy);
    galaxy.expansion_factor = 1_000_000;
    let distnaces_part_2 = find_shortest_distances(&galaxy);

    let sol1: u64 = distances.into_iter().sum();
    let sol2: u64 = distnaces_part_2.into_iter().sum();
    (Solution::from(sol1), Solution::from(sol2))
}
fn find_shortest_distances(galaxy: &Galaxy ) -> Vec<u64> {
    let mut distances = Vec::new();
    //loop over all combination of galaxies
    for i in 0..galaxy.galaxies.len() {
        for j in i+1..galaxy.galaxies.len() {
            distances.push(galaxy.find_shortest_distance(galaxy.galaxies[i], galaxy.galaxies[j]))
        }
    }
    distances

}
fn expanding_places(rows: HashSet<usize>, cols: HashSet<usize>, grid: &mut Vec<Vec<Space>>) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if !rows.contains(&row) && !cols.contains(&col) {
                grid[row][col] = Space::Expansion;
            }
        }
    }
}
fn parse_input(input: String) -> Galaxy {
    let mut grid = Vec::new();
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut galaxies = Vec::<(usize, usize)>::new();
    let mut expand_rows = HashSet::new();
    let mut expand_cols = HashSet::new();
    let mut r_index = 0;
    for line in input.lines() {
        let mut row = Vec::new();
        let mut c_index = 0;
        for c in line.chars() {
            let spacetile = match c {
                '.' => Space::Empty,
                '#' => Space::Galaxy,
                _ => panic!("Invalid input"),
            };
            if spacetile == Space::Galaxy {
                rows.insert(r_index);
                cols.insert(c_index);
                galaxies.push((r_index, c_index));
            }
            row.push(spacetile);
            c_index += 1;
        }
        r_index += 1;
        grid.push(row);
    }
    expanding_places(rows, cols, &mut grid);
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == Space::Expansion {
                expand_rows.insert(row);
                expand_cols.insert(col);
            }
        }
        
    }

    Galaxy {
        map: grid,
        galaxies: galaxies,
        expand_rows: expand_rows,
        expand_cols: expand_cols,
        expansion_factor: 2,
    }
}
#[derive(Debug)]
struct Galaxy {
    map: Vec<Vec<Space>>,
    galaxies: Vec<(usize, usize)>,
    expand_rows: HashSet<usize>,
    expand_cols: HashSet<usize>,
    expansion_factor: u64,


}
impl Galaxy {
    fn find_shortest_distance(&self, g1: (usize,usize),g2:(usize, usize)) -> u64 {

        let lower_row = cmp::min(g1.0, g2.0);
        let upper_row = cmp::max(g1.0, g2.0);
        let lower_col = cmp::min(g1.1, g2.1);
        let upper_col = cmp::max(g1.1, g2.1);

        let dist = ((upper_row - lower_row) + (upper_col - lower_col)) as u64;

        let rows:HashSet<usize> = HashSet::from_iter((lower_row..upper_row).collect::<Vec<usize>>());
        let cols:HashSet<usize> = HashSet::from_iter((lower_col..upper_col).collect::<Vec<usize>>());
        let expand_rows = rows.intersection(&self.expand_rows).count() as u64;
        let expand_cols = cols.intersection(&self.expand_cols).count() as u64;

        dist + expand_rows * (&self.expansion_factor-1) + expand_cols * (&self.expansion_factor-1)
    }
}

impl fmt::Display for Galaxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.map {
            for col in row {
                match col {
                    Space::Empty => write!(f, ".")?,
                    Space::Galaxy => write!(f, "#")?,
                    Space::Expansion => write!(f, "E")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }

}


#[derive(Debug, PartialEq, Eq, Hash)]
enum Space {
    Galaxy,
    Empty,
    Expansion
    
}

#[test]
fn sample_input() {
    let input = String::from("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
    let grid = parse_input(input);
    println!("{}", grid);
    let distances = find_shortest_distances(&grid);
    println!("{:?}", distances);
    assert_eq!(distances.into_iter().sum::<u64>(), 374);

}