use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: i64 = 0;
    let mut sol2: i64 = 0;
    let input = read_to_string("input/days/day05.txt").unwrap();
    let maps = input.split("\n\n").collect::<Vec<&str>>();
    if let Some((_, seed_string)) = maps[0].split_once(": ") {
        let seeds = seed_string.split_whitespace().collect::<Vec<&str>>().iter().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let seed_to_soil_maps = maps[1].split("\n").skip(1).collect::<Vec<&str>>().iter().map(|n| FunctionMap::new(n)).collect::<Vec<FunctionMap>>();
        let soil_to_fertilizer_maps = maps[2].split("\n").skip(1).collect::<Vec<&str>>().iter().map(|n| FunctionMap::new(n)).collect::<Vec<FunctionMap>>();
        let fertilizer_to_water_maps = maps[3].split("\n").skip(1).collect::<Vec<&str>>().iter().map(|n| FunctionMap::new(n)).collect::<Vec<FunctionMap>>();
        let water_to_light_maps = maps[4].split("\n").skip(1).collect::<Vec<&str>>().iter().map(|n| FunctionMap::new(n)).collect::<Vec<FunctionMap>>();
        let light_to_temperature_maps = maps[5].split("\n").skip(1).collect::<Vec<&str>>().iter().map(|n| FunctionMap::new(n)).collect::<Vec<FunctionMap>>();
        let temperature_to_humidity_maps = maps[6].split("\n").skip(1).collect::<Vec<&str>>().iter().map(|n| FunctionMap::new(n)).collect::<Vec<FunctionMap>>();
        let humidity_to_location_maps = maps[7].split("\n").skip(1).collect::<Vec<&str>>().iter().map(|n| FunctionMap::new(n)).collect::<Vec<FunctionMap>>();
        let farming_plan = FarmingPlan {
            maps: vec![
                seed_to_soil_maps,
                soil_to_fertilizer_maps,
                fertilizer_to_water_maps,
                water_to_light_maps,
                light_to_temperature_maps,
                temperature_to_humidity_maps,
                humidity_to_location_maps,
            ]
        };
        //Solve for part 1
        let mut smallest_location = None;
        for seed in seeds.clone() {
            let location  = farming_plan.get_location(seed);
            if smallest_location.is_none() || location < smallest_location.unwrap() {
                smallest_location = Some(location);
            }
        }
        sol1 = smallest_location.unwrap_or(0);

        //Solve for part 2
        let mut part_2_seeds = Vec::new();
        let mut seed_iter = seeds.iter();
        while let Some(start) = seed_iter.next() {
            let count = seed_iter.next().unwrap();
            for i in 0..*count {
                part_2_seeds.push(start + i);
            }
        }
    
        let mut smallest_location = None;
        for seed in part_2_seeds {
            let location  = farming_plan.get_location(seed);
            if smallest_location.is_none() || location < smallest_location.unwrap() {
                smallest_location = Some(location);
            }
        }
        sol2 = smallest_location.unwrap_or(0);
    }

    (Solution::from(sol1), Solution::from(sol2))
}
#[derive(Debug)]
struct FarmingPlan {
    maps: Vec<Vec<FunctionMap>>,
}

impl FarmingPlan {
    fn get_location(&self, seed: i64) -> i64 {
        let mut current_number = seed;
        for current_map in self.maps.iter() {
            for functions in current_map.into_iter() {
                if functions.contains(current_number) {
                    current_number = functions.map(current_number).unwrap();
                    break;
                }
            }
        }
        current_number
    }
    
}
#[derive(Debug)]
struct FunctionMap {
    source_start: i64,
    dest_start: i64,
    length: i64,
}

impl FunctionMap {
    fn new(input: &str) -> FunctionMap {
        let data_vect = input.trim_end().split_whitespace().collect::<Vec<&str>>().into_iter().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        FunctionMap {
            source_start: data_vect[1],
            dest_start: data_vect[0],
            length: data_vect[2],
        }
    }
    fn contains(&self, source: i64) -> bool {
        source >= self.source_start && source < self.source_start + self.length
    }
    fn map(&self, source: i64) -> Option<i64> {
        if self.contains(source) {
            Some(self.dest_start + (source - self.source_start))
        } else {
            None
        }
    }
}