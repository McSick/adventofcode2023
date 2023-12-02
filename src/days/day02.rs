use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, collections::HashMap};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day02.txt").unwrap();
    let games = input.lines();
    let mut sol1: i32 = 0;
    let mut sol2: i32 = 0;
    for gametext in games {
        let parsed = gametext.split(":").collect::<Vec<&str>>();
        let mut game = Game::new(parsed[0]);
        let draws = parsed[1].split(";").collect::<Vec<&str>>();
        //3 blue, 4 red
        for draw in draws {
            let colors = draw.split(",").collect::<Vec<&str>>();
            let mut map = HashMap::new();
            for color in colors {
                let sanitized = color.trim();
                let count = sanitized.split(" ").collect::<Vec<&str>>();
                map.insert(count[1].to_string(), count[0].parse::<i32>().unwrap());
            }
            let draw = Draw {
                map: map
            };
            game.draws.push(draw);
        }
        //println!("Game {:?} is valid: {}", game, game.is_valid(12, 13, 14));
        if game.is_valid(12, 13, 14) {
            sol1 += game.id;
        }
        let min_cubes = game.min_num_cubes();
        sol2 += min_cubes.0 * min_cubes.1 * min_cubes.2

    }

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug)]
struct Game {
    id:i32,
    draws: Vec<Draw>
}
impl Game {
    
    fn new(text: &str) -> Game {
        let parsed = text.split(" ").collect::<Vec<&str>>();

        Game {
            id: parsed[1].parse::<i32>().unwrap(),
            draws: Vec::new()
        }
    }
    fn is_valid(&self, r: i32, g:i32, b:i32) -> bool {
        for draw in &self.draws {
            if !draw.is_valid(r, g, b) {
                return false;
            }
        }
        true
    }
    fn min_num_cubes(&self) -> (i32,i32,i32) {
        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;
        for draw in &self.draws {
            if let Some(count) = draw.map.get("red") {
                if *count > min_r {
                    min_r = *count;
                }
            }
            if let Some(count) = draw.map.get("green") {
                if *count > min_g {
                    min_g = *count;
                }
            }
            if let Some(count) = draw.map.get("blue") {
                if *count > min_b {
                    min_b = *count;
                }
            }
        }
        (min_r, min_g, min_b)
    }

}
#[derive(Debug)]
struct Draw {
    map: HashMap<String, i32>
}

impl Draw {
    fn is_valid(&self, r: i32, g:i32, b:i32) -> bool {
        let mut r_ok = false;
        let mut g_ok = false;
        let mut b_ok = false;
        if let Some(count) = self.map.get("red") {
            if r >= *count {
                r_ok = true;
            }
        } else {
            r_ok = true;
        }
        if let Some(count) = self.map.get("green") {
            if g >= *count {
                g_ok = true;
            }
        } else {
            g_ok = true;
        }
        if let Some(count) = self.map.get("blue") {
            if b >= *count {
                b_ok = true;
            }
        } else {
            b_ok = true;
        }
        r_ok && g_ok && b_ok
    }
}
