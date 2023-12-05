use std::collections::HashMap;

use super::Day;
use adventofcode2023::read_lines;
use regex::Regex;

#[derive(Debug)]
struct MapEntry {
    input_start: i64,
    output_start: i64,
    range: i64,
}

#[derive(Debug)]
struct MaterialMap {
    name: String,
    mappings: Vec<MapEntry>,
}

#[derive(Debug)]
pub struct Day05 {
    input: Vec<String>,
    material_maps: Vec<MaterialMap>,
    seeds: Vec<i64>,
}

impl Day05 {
    pub fn new() -> Day05 {
        Day05 {
            input: Vec::new(),
            material_maps: Vec::new(),
            seeds: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        let mut input_iter = self.input.iter();
        // 1st line: Seeds:
        let seed_line = input_iter.next().unwrap();
        let seed_re = Regex::new(r"seeds: (.*)").unwrap();
        let seeds = seed_re
            .captures(seed_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let seeds = seeds
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        // println!("Seeds: {:?}", seeds);
        self.seeds = seeds;

        // skip 2nd line:
        input_iter.next();

        // process all blocks of lines, an empty line marks the end of a block:
        let mapping_match = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();
        let mut start_of_block = true;
        let mut act_map = MaterialMap {
            mappings: Vec::new(),
            name: String::new(),
        };
        for line in input_iter {
            if start_of_block && !line.is_empty() {
                act_map.name = line.clone();
                start_of_block = false;
                continue;
            }
            if !start_of_block && line.is_empty() {
                // end of block: store it
                self.material_maps.push(act_map);
                act_map = MaterialMap {
                    mappings: Vec::new(),
                    name: String::new(),
                };
                start_of_block = true;
                continue;
            }
            let captures = mapping_match.captures(line);
            if let Some(captures) = captures {
                // must be a line with number mappings
                let entry = MapEntry {
                    input_start: captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                    output_start: captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    range: captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                };
                act_map.mappings.push(entry);
            }
        }
        // capture the last entry, too:
        if !start_of_block {
            self.material_maps.push(act_map);
        }
        // println!("Material maps: {:?}", self.material_maps);
    }
}

impl Day for Day05 {
    fn day_nr(&self) -> String {
        String::from("05")
    }
    fn title(&self) -> String {
        String::from("Day 5: xxxx")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day05.txt");
        // let input = read_lines("data/day05-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut final_values: Vec<i64> = Vec::new();
        for seed in self.seeds.iter() {
            let mut act_val = *seed;
            // print!("Input: {0}, ", act_val);
            for map in self.material_maps.iter() {
                act_val = find_mapped_value(map, act_val);
                // print!("{0}, ", act_val);
            }
            // println!("Output: {0}\n", act_val);
            final_values.push(act_val);
        }
        let min = final_values.iter().min().unwrap();
        String::from(format!("{0}", *min))
    }

    fn solve2(&mut self) -> String {
        let mut sum = 0;
        String::from(format!("{0}", sum))
    }
}

fn find_mapped_value(map: &MaterialMap, val: i64) -> i64 {
    for entry in map.mappings.iter() {
        if val >= entry.input_start && val <= entry.input_start + entry.range {
            return entry.output_start + (val - entry.input_start);
        }
    }
    val
}
