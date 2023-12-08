use std::collections::HashMap;

use super::Day;
use adventofcode2023::{read_lines, scm};
use regex::Regex;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct Day08 {
    input: Vec<String>,
    dirs: String,
    nodes: HashMap<String, Node>,
}

impl Day08 {
    pub fn new() -> Day08 {
        Day08 {
            input: Vec::new(),
            dirs: String::new(),
            nodes: HashMap::new(),
        }
    }

    fn parse_input(&mut self) {
        // 1st line: dirs
        self.dirs = self.input[0].clone();

        // 3rd.. lines: nodes
        let matcher = Regex::new(r"(\w+)\s+=\s+\((\w+),\s+(\w+)\)").unwrap();
        for i in 2..self.input.len() {
            let line = self.input.get(i).unwrap();
            if let Some(group) = matcher.captures(line) {
                let name = group.get(1).unwrap().as_str().to_string();
                let left = group.get(2).unwrap().as_str().to_string();
                let right = group.get(3).unwrap().as_str().to_string();
                self.nodes.insert(
                    name.clone(),
                    Node {
                        name: name.clone(),
                        left,
                        right,
                    },
                );
            }
        }
    }
}

impl Day for Day08 {
    fn day_nr(&self) -> String {
        String::from("08")
    }
    fn title(&self) -> String {
        String::from("Day 8: Haunted Wasteland")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day08.txt");
        // let input = read_lines("data/day08-test.txt");
        // let input = read_lines("data/day08-test2.txt");
        // Test Input for part 2:
        // let input = read_lines("data/day08-test1-2.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        let mut dir_index = 0;

        let mut act_node = self.nodes.get("AAA").unwrap();
        while act_node.name != "ZZZ" {
            let dir = self.dirs.chars().nth(dir_index).unwrap();
            match dir {
                'R' => act_node = self.nodes.get(&act_node.right).unwrap(),
                'L' => act_node = self.nodes.get(&act_node.left).unwrap(),
                _ => panic!("Unknown direction"),
            }
            dir_index = (dir_index + 1) % self.dirs.len();
            solution += 1;
        }

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        // Idea:
        // Calc each 'xxA' node's path length separately,
        // then find the smallest common multiplier
        let mut start_nodes: Vec<String> = Vec::new();
        let mut path_lengths: Vec<u64> = Vec::new();

        // find all 'xxA' start nodes:
        for name in self.nodes.keys() {
            if name.chars().last().unwrap() == 'A' {
                start_nodes.push(name.clone());
            }
        }

        for node_name in start_nodes {
            let mut act_node = self.nodes.get(&node_name).unwrap();
            let mut path_length = 0;
            let mut dir_index = 0;
            while act_node.name.chars().last().unwrap() != 'Z' {
                let dir = self.dirs.chars().nth(dir_index).unwrap();
                match dir {
                    'R' => act_node = self.nodes.get(&act_node.right).unwrap(),
                    'L' => act_node = self.nodes.get(&act_node.left).unwrap(),
                    _ => panic!("Unknown direction"),
                }
                dir_index = (dir_index + 1) % self.dirs.len();
                path_length += 1;
            }
            path_lengths.push(path_length);
        }
        let mut a = path_lengths[0];
        for i in 1..path_lengths.len() {
            a = scm(a, path_lengths[i]);
        }
        String::from(format!("{0}", a))
    }
}
