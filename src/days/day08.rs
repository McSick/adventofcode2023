use crate::{Solution, SolutionPair};
use regex_lite::Regex;
use std::{collections::HashMap, fs::read_to_string};
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/days/day08.txt").unwrap();
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    lines.next(); // Skip the new line
    let mut graph: HashMap<Node, (Node, Node)> = HashMap::new();
    for line in lines {
        let re = Regex::new(
            r"(?<source>[0-9a-zA-Z]{3})\s=\s\((?<left>[0-9a-zA-Z]{3}),\s(?<right>[0-9a-zA-Z]{3})\)",
        )
        .unwrap();
        if let Some(caps) = re.captures(line) {
            graph.insert(
                Node::from_str(caps.name("source").map_or("", |m| m.as_str())),
                (
                    Node::from_str(caps.name("left").map_or("", |m| m.as_str())),
                    Node::from_str(caps.name("right").map_or("", |m| m.as_str())),
                ),
            );
        }
    }
    let directions: Vec<char> = directions.chars().collect();

    //Part1
    let sol1 = steps_until_end(&graph, &directions, &Node::from_str("AAA"));

    //Part2
    let starting_nodes = graph
        .keys()
        .filter(|k| match k {
            Node::Start(_) => true,
            _ => false,
        })
        .collect::<Vec<&Node>>();
    let mut sol2 = 1;
    for node in starting_nodes {
        let steps = steps_until_end(&graph, &directions, node);
        sol2 = lcm(sol2, steps);
    }
    (Solution::from(sol1), Solution::from(sol2))
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
fn steps_until_end(
    graph: &HashMap<Node, (Node, Node)>,
    directions: &Vec<char>,
    start: &Node,
) -> u64 {
    let mut loop_count = 0;
    let mut current_node = start.clone();
    loop {
        let (left, right) = graph.get(&current_node).unwrap();
        if directions[loop_count % directions.len()] == 'L' {
            current_node = left.clone();
        } else {
            current_node = right.clone();
        }
        loop_count += 1;
        match current_node {
            Node::End(_) => {
                break;
            }
            _ => {}
        };
    }
    loop_count as u64
}
#[derive(Eq, Hash, Clone, Debug)]
enum Node {
    Start(String),
    End(String),
    Middle(String),
}
impl Node {
    fn from_str(s: &str) -> Node {
        let is_start = s.rfind('A').unwrap_or(0) == 2;
        let is_end = s.rfind('Z').unwrap_or(0) == 2;
        match (is_start, is_end) {
            (true, _) => Node::Start(s.to_string()),
            (_, true) => Node::End(s.to_string()),
            _ => Node::Middle(s.to_string()),
        }
    }
    fn unwrap_value(&self) -> String {
        match self {
            Node::Start(s) => s.clone(),
            Node::End(s) => s.clone(),
            Node::Middle(s) => s.clone(),
        }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        let self_value = self.unwrap_value();
        let other_value = other.unwrap_value();
        self_value == other_value
    }
}
