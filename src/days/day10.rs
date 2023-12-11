use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, vec};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day10.txt").unwrap();
    let landscape = parse_input(input);
    let (dist, map) = landscape.find_farthest_distance();
    let inside_space = find_inside_space(map);
    (Solution::from(dist), Solution::from(inside_space))
}

fn fill(next_row:isize, next_col: isize,  map: &mut Vec<Vec<Pipe>>) {
    if next_row < 0 || next_col < 0 || next_row >= map.len() as isize || next_col >= map[0].len() as isize{
        return;
    }

    if map[next_row as usize][next_col as usize] != Pipe::Empty {
        return;
    } else {
        map[next_row as usize][next_col as usize] = Pipe::Horizontal;
    }
    fill(next_row - 1, next_col, map);
    fill(next_row + 1, next_col, map);
    fill(next_row, next_col - 1, map);
    fill(next_row, next_col + 1, map);
}
fn find_inside_space(map: Vec<Vec<Pipe>>) -> usize {
    let mut count = 0;
    let mut padded_map = map.iter().map(|row| {
        let mut padded_row = row.clone();
        padded_row.insert(0, Pipe::Empty);
        padded_row.push(Pipe::Empty);
        padded_row
    }).collect::<Vec<Vec<Pipe>>>();
    let empty_row = vec![Pipe::Empty; padded_map[0].len()];
    padded_map.insert(0, empty_row.clone());
    padded_map.push(empty_row.clone());
    //padded_map.iter().for_each(|row| {row.iter().for_each(|pipe| if pipe == &Pipe::Empty {print!(".")} else {print!("I")}); println!()});
    //tiled map
    let mut tiled_map = vec![vec![Pipe::Empty; padded_map[0].len() * 3]; padded_map.len() * 3];
    let mut i = 0;
    for row in padded_map {
        let mut j = 0;
        for pipe in row {
            let tile = pipe.tile();
            for r in 0..3 {
                for c in 0..3 {
                    tiled_map[i + r][j + c] = tile[r][c].clone();
                }
            }
            j += 3; 
        }
        i += 3;

    }

   tiled_map.iter().for_each(|row| {row.iter().for_each(|pipe| if pipe == &Pipe::Empty {print!(" ")} else {print!("I")}); println!()});
    fill(0,0, &mut tiled_map);
    //tiled_map.iter().for_each(|row| {row.iter().for_each(|pipe| if pipe == &Pipe::Empty {print!(".")} else {print!("I")}); println!()});
    let mut i = 0;
    while i < tiled_map.len() {
        let mut j = 0;
        while j < tiled_map[0].len() {

            let mut full_empty  = true;
            for r in 0..3 {
                for c in 0..3 {
                    if tiled_map[i+r][j+c] != Pipe::Empty {
                        full_empty = false;
                    }
                }
            }
            if full_empty {
                count += 1;
            }
            
            j += 3;
        }
        i += 3;
    }

    count 
}

fn parse_input(input: String) -> Landscape {
    let mut vec = Vec::new();
    let mut start = (0,0);
    let mut x = 0;
    for line in input.lines() {
        let mut y = 0;
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(Pipe::Empty),
                '-' => row.push(Pipe::Horizontal),
                '|' => row.push(Pipe::Vertical),
                'F' => row.push(Pipe::NorthWest),
                '7' => row.push(Pipe::NorthEast),
                'L' => row.push(Pipe::SouthWest),
                'J' => row.push(Pipe::SouthEast),
                'S' => { start.0 = x; start.1 = y; row.push(Pipe::Start) },
                _ => panic!("Invalid character in input: {}", c),
            }
            y += 1;
        }
        x += 1;
        vec.push(row);
    }
    Landscape { x: vec.len(), y: vec[0].len(), vec: vec, start_x: start.0, start_y: start.1 }
}
#[derive(Debug)]
struct Landscape {
    vec: Vec<Vec<Pipe>>,
    x: usize,
    y: usize,
    start_x: usize,
    start_y: usize,

}
impl Landscape {

    fn get_element(&self, x: isize, y: isize) -> Option<&Pipe> {
        if x < 0 || y < 0 || x >= self.x as isize || y >= self.y as isize {
            return None;
        }
        Some(&self.vec[x as usize][y as usize])
    }
    fn find_surrounding_elements(&self, pipe: &Pipe, x: usize, y: usize, surround_pipes_check:Vec<Direction>) -> Vec<Direction> {

        let mut valid_directions = Vec::new();

        for direction in surround_pipes_check {
            let (r,c) = direction.tuple();
            if let Some(found_pipe) = self.get_element(x as isize + r, y as isize + c) {
                if found_pipe != &Pipe::Empty {
                    match (pipe, direction, found_pipe) {
                        (Pipe::NorthWest,Direction::East, Pipe::Horizontal) => valid_directions.push(direction),
                        (Pipe::NorthWest, Direction::East, Pipe::NorthEast) => valid_directions.push(direction),
                        (Pipe::NorthWest, Direction::East, Pipe::SouthEast) => valid_directions.push(direction),
                        (Pipe::NorthWest, Direction::South, Pipe::Vertical) => valid_directions.push(direction),
                        (Pipe::NorthWest, Direction::South, Pipe::SouthWest) => valid_directions.push(direction),
                        (Pipe::NorthWest, Direction::South, Pipe::SouthEast) => valid_directions.push(direction),

                        (Pipe::NorthEast,Direction::West, Pipe::Horizontal) => valid_directions.push(direction),
                        (Pipe::NorthEast, Direction::West, Pipe::NorthWest) => valid_directions.push(direction),
                        (Pipe::NorthEast, Direction::West, Pipe::SouthWest) => valid_directions.push(direction),
                        (Pipe::NorthEast, Direction::South, Pipe::Vertical) => valid_directions.push(direction),
                        (Pipe::NorthEast, Direction::South, Pipe::SouthWest) => valid_directions.push(direction),
                        (Pipe::NorthEast, Direction::South, Pipe::SouthEast) => valid_directions.push(direction),

                        (Pipe::SouthWest,Direction::East, Pipe::Horizontal) => valid_directions.push(direction),    
                        (Pipe::SouthWest, Direction::East, Pipe::NorthEast) => valid_directions.push(direction),
                        (Pipe::SouthWest, Direction::East, Pipe::SouthEast) => valid_directions.push(direction),
                        (Pipe::SouthWest, Direction::North, Pipe::Vertical) => valid_directions.push(direction),
                        (Pipe::SouthWest, Direction::North, Pipe::NorthWest) => valid_directions.push(direction),
                        (Pipe::SouthWest, Direction::North, Pipe::NorthEast) => valid_directions.push(direction),

                        (Pipe::SouthEast,Direction::West, Pipe::Horizontal) => valid_directions.push(direction),
                        (Pipe::SouthEast, Direction::West, Pipe::NorthWest) => valid_directions.push(direction),
                        (Pipe::SouthEast, Direction::West, Pipe::SouthWest) => valid_directions.push(direction),
                        (Pipe::SouthEast, Direction::North, Pipe::Vertical) => valid_directions.push(direction),
                        (Pipe::SouthEast, Direction::North, Pipe::NorthWest) => valid_directions.push(direction),
                        (Pipe::SouthEast, Direction::North, Pipe::NorthEast) => valid_directions.push(direction),

                        (Pipe::Horizontal, Direction::East, Pipe::NorthEast) => valid_directions.push(direction),
                        (Pipe::Horizontal, Direction::East, Pipe::SouthEast) => valid_directions.push(direction),
                        (Pipe::Horizontal, Direction::West, Pipe::NorthWest) => valid_directions.push(direction),
                        (Pipe::Horizontal, Direction::West, Pipe::SouthWest) => valid_directions.push(direction),
                        (Pipe::Horizontal, Direction::East, Pipe::Horizontal) => valid_directions.push(direction),
                        (Pipe::Horizontal, Direction::West, Pipe::Horizontal) => valid_directions.push(direction),

                        (Pipe::Vertical, Direction::North, Pipe::NorthWest) => valid_directions.push(direction),
                        (Pipe::Vertical, Direction::North, Pipe::NorthEast) => valid_directions.push(direction),
                        (Pipe::Vertical, Direction::South, Pipe::SouthWest) => valid_directions.push(direction),
                        (Pipe::Vertical, Direction::South, Pipe::SouthEast) => valid_directions.push(direction),
                        (Pipe::Vertical, Direction::North, Pipe::Vertical) => valid_directions.push(direction),
                        (Pipe::Vertical, Direction::South, Pipe::Vertical) => valid_directions.push(direction),


                        _ => {},
                    }
                }
            }
        }
        valid_directions
    }
    fn find_farthest_distance (&self) -> (usize, Vec<Vec<Pipe>>) {
        let mut distance = 0;
        let mut empty_map = vec![vec![Pipe::Empty; self.y]; self.x];

        //I hardcoded this because I was lazy
        empty_map[self.start_x][self.start_y] = Pipe::NorthEast;
        let directions = self.find_surrounding_elements(&Pipe::NorthEast, self.start_x, self.start_y, vec![Direction::North, Direction::South, Direction::West, Direction::East]);
        let mut direction = directions[0];
        let mut x = self.start_x;
        let mut y = self.start_y; 
        loop {
            (x,y) = self.next(x, y, &direction);
            distance += 1;
            if let Some(pipe) = self.get_element(x as isize, y as isize) {
                empty_map[x][y] = pipe.clone();
                let surrounding_pipes = match direction {
                    Direction::North => vec![Direction::North, Direction::West, Direction::East],
                    Direction::South => vec![Direction::South, Direction::West, Direction::East],
                    Direction::West => vec![Direction::North, Direction::South, Direction::West],
                    Direction::East => vec![Direction::North, Direction::South, Direction::East],
                };
                let directions = self.find_surrounding_elements(pipe, x, y, surrounding_pipes);
                if directions.len() == 0 || pipe == &Pipe::Start {
                    break;
                }
                direction = directions[0];

            }
            if x == self.start_x && y == self.start_y {
                break;
            }
        }
        //empty_map.iter().for_each(|row| println!("{:?}", row));
        (distance / 2 + 1, empty_map)
        
    }
    fn next(&self, x: usize, y: usize, direction: &Direction) -> (usize, usize) {
        match direction {
            Direction::North => (x-1, y),
            Direction::South => (x+1, y),
            Direction::West => (x, y-1),
            Direction::East => (x, y+1),
        }

    }
 }
// struct Movement {
//     direction: Direction,
//     pipe: Pipe

// }
#[derive(Debug,PartialEq,Clone,Copy, Hash)]
enum Direction {
    North ,
    South,
    West,
    East,
}
impl Direction {
    fn tuple (&self) -> (isize, isize) {
        match self {
            Direction::North => (-1,0),
            Direction::South => (1,0),
            Direction::West => (0,-1),
            Direction::East => (0,1),
        }
    }
}
#[derive(Debug,PartialEq, Clone)]
enum Pipe {
    Empty,
    Horizontal,
    Vertical,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Start,
    Filled

}
impl Pipe {
    fn tile(&self) -> Vec<Vec<Pipe>> {
        let vec = vec![vec![Pipe::Empty; 3]; 3];
        match &self {
            Pipe::Empty => vec,
            Pipe::Vertical => vec![vec![Pipe::Empty, Pipe::Filled, Pipe::Empty],
                                     vec![Pipe::Empty, Pipe::Filled, Pipe::Empty],
                                     vec![Pipe::Empty, Pipe::Filled, Pipe::Empty]],
            Pipe::Horizontal => vec![vec![Pipe::Empty, Pipe::Empty, Pipe::Empty],
                                    vec![Pipe::Filled, Pipe::Filled, Pipe::Filled],
                                    vec![Pipe::Empty, Pipe::Empty, Pipe::Empty]],
            Pipe::NorthWest => vec![vec![Pipe::Empty, Pipe::Empty, Pipe::Empty],
                                    vec![Pipe::Empty, Pipe::Filled, Pipe::Filled],
                                    vec![Pipe::Empty, Pipe::Filled, Pipe::Empty]],
            Pipe::SouthWest => vec![vec![Pipe::Empty, Pipe::Filled, Pipe::Empty],
                                    vec![Pipe::Empty, Pipe::Filled, Pipe::Filled],
                                    vec![Pipe::Empty, Pipe::Empty, Pipe::Empty]],
            Pipe::NorthEast => vec![vec![Pipe::Empty, Pipe::Empty, Pipe::Empty],
                                    vec![Pipe::Filled, Pipe::Filled, Pipe::Empty],
                                    vec![Pipe::Empty, Pipe::Filled, Pipe::Empty]],
            Pipe::SouthEast => vec![vec![Pipe::Empty, Pipe::Filled, Pipe::Empty],
                                    vec![Pipe::Filled, Pipe::Filled, Pipe::Empty],
                                    vec![Pipe::Empty, Pipe::Empty, Pipe::Empty]],
            _ => vec
        }
    }
}



#[test]
fn test_1() {
    const TEST1:&str = ".....
.S-7.
.|.|.
.L-J.
.....";
    let landscape = parse_input(TEST1.to_string());
    assert_eq!(landscape.vec[0][0], Pipe::Empty);
    let (dist, map) = landscape.find_farthest_distance();
    let inside_space = find_inside_space(map);
    assert_eq!(dist, 4);
    assert_eq!(inside_space, 1);

}
#[test]
fn test_2() {
    const TEST2:&str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    let landscape = parse_input(TEST2.to_string());

    assert_eq!(landscape.vec[0][0], Pipe::Horizontal);
    let (dist, map) = landscape.find_farthest_distance();
    let inside_space = find_inside_space(map);
    assert_eq!(dist, 4);
    assert_eq!(inside_space, 1);
}
#[test]
fn test_3() {
    const TEST3:&str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let landscape = parse_input(TEST3.to_string());

    assert_eq!(landscape.vec[0][0], Pipe::Empty);
    let (dist, map) = landscape.find_farthest_distance();

    assert_eq!(dist, 8);
}

#[test]
fn test_4() {
    const TEST4:&str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    let landscape = parse_input(TEST4.to_string());

    assert_eq!(landscape.vec[0][0], Pipe::NorthEast);
    let (dist, map) = landscape.find_farthest_distance();
    assert_eq!(dist, 8);
}
#[test]
fn test_5() {
    const TEST5:&str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    let landscape = parse_input(TEST5.to_string());
    let (dist, map) = landscape.find_farthest_distance();
    let inside_space = find_inside_space(map);
    assert_eq!(inside_space, 4);

}

#[test]
fn test_6() {
    const TEST6:&str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    let landscape = parse_input(TEST6.to_string());
    let (dist, map) = landscape.find_farthest_distance();
    let inside_space = find_inside_space(map);
    assert_eq!(inside_space, 8);

}

#[test]
fn test_7() {
    const TEST7: &str ="..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
let landscape = parse_input(TEST7.to_string());
let (dist, map) = landscape.find_farthest_distance();
let inside_space = find_inside_space(map);
assert_eq!(inside_space, 4);
}

#[test]
fn test_8() {
    const TEST8: &str ="FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
let landscape = parse_input(TEST8.to_string());
let (dist, map) = landscape.find_farthest_distance();
let inside_space = find_inside_space(map);
assert_eq!(inside_space, 10);
}