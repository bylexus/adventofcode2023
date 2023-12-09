use std::{sync::mpsc::channel, thread};

use super::Day;
use adventofcode2023::read_lines;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct MapEntry {
    input_start: i64,
    output_start: i64,
    range: i64,
}

#[derive(Debug, Clone)]
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
    }
}

impl Day for Day05 {
    fn day_nr(&self) -> String {
        String::from("05")
    }
    fn title(&self) -> String {
        String::from("If You Give A Seed A Fertilizer")
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
            for map in self.material_maps.iter() {
                act_val = find_mapped_value(map, act_val);
            }
            final_values.push(act_val);
        }
        let min = final_values.iter().min().unwrap();
        String::from(format!("{0}", *min))
    }

    fn solve2(&mut self) -> String {
        let mut final_values: Vec<i64> = Vec::new();
        let final_seeds = merge_seeds(self.seeds.clone());
        let (tx, rx) = channel();
        for (start, end) in final_seeds.iter() {
            let tx_thread = tx.clone();
            let material_maps = self.material_maps.clone();
            let start = *start;
            let end = *end;
            thread::spawn(move || {
                let mut min: i64 = i64::MAX;
                for seed in start..end {
                    let mut act_val = seed;
                    for map in material_maps.iter() {
                        act_val = find_mapped_value(map, act_val);
                    }
                    if act_val < min {
                        min = act_val;
                    }
                }
                tx_thread.send(min).unwrap();
            });
        }
        drop(tx);
        for val in rx.iter() {
            final_values.push(val);
        }

        let min = final_values.iter().min().unwrap();
        String::from(format!("{0}", *min))
    }
}

/// Merge the seeds table into a overlap-removed seed table.
/// We check each seed if it overlaps with the existing seeds,
/// and split / remove them to non-overlapping versions.
/// At the end, we return a non-overlapping (start, end) pair list.
fn merge_seeds(mut seeds: Vec<i64>) -> Vec<(i64, i64)> {
    let mut final_seeds: Vec<(i64, i64)> = Vec::new();

    // takt the last 2 elements (start and len of seed), until
    // there are no more pairs:
    while seeds.len() > 0 {
        let seed_len = seeds.pop().unwrap();
        let mut seed_start = seeds.pop().unwrap();
        let mut seed_end = seed_start + seed_len - 1;

        // first seed goes to the final list anyway:
        if final_seeds.len() == 0 {
            final_seeds.push((seed_start, seed_end));
            continue;
        }

        // Overlaps can happen as follows:
        //      |--------final seed---------|
        //           |..... within ....|
        // |...........| start overlap
        //               end overlap    |.............|
        // |.....................................| <-- total overlap
        // |..| <-- non-overlap
        //                        non-overlap -->  |...|
        for fseed in final_seeds.iter() {
            // insertion seed fits within the target feed: skip it completely, already
            // covered by the actual seed:
            if seed_start >= fseed.0 && seed_end <= fseed.1 {
                seed_end = -1;
                break;
            }
            // does it overlap somehow?
            if seed_start < fseed.1 && seed_end > fseed.0 {
                // extract front part:
                if seed_start < fseed.0 {
                    seeds.push(seed_start);
                    seeds.push(fseed.0 - seed_start);
                }
                // extract end part:
                if seed_end > fseed.1 {
                    seeds.push(fseed.1 + 1);
                    seeds.push(seed_end - fseed.1);
                }
                seed_start = -1;
                seed_end = -1;
                break;
            } else {
                // no overlap, continue to the next final seed:
                continue;
            }
        }
        // no overlap, add it to final seeds:
        if seed_start >= 0 && seed_end > 0 && seed_end >= seed_start {
            final_seeds.push((seed_start, seed_end));
        }
    }

    // tuples with first and last value of a seed
    final_seeds
}

fn find_mapped_value(map: &MaterialMap, val: i64) -> i64 {
    for entry in map.mappings.iter() {
        if val >= entry.input_start && val <= entry.input_start + entry.range {
            return entry.output_start + (val - entry.input_start);
        }
    }
    val
}
