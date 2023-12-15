use std::collections::HashMap;

use super::Day;
use alex_lib::read_lines;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Day15 {
    input: Vec<String>,
    input_data: Vec<String>,
}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 {
            input: Vec::new(),
            input_data: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        self.input_data = self
            .input
            .iter()
            .map(|s| s.trim().to_string())
            .reduce(|acc, s| acc.to_string() + s.as_str())
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();
    }

    /// AoC Hash Algorithm:
    ///  - Start with current value = 0.
    ///  - For each char:
    ///     - Determine the ASCII code for the current character of the string.
    ///     - Increase the current value by the ASCII code you just determined.
    ///     - Set the current value to itself multiplied by 17.
    ///     - Set the current value to the remainder of dividing itself by 256.

    fn hash(&self, s: &str) -> u64 {
        let mut act_val = 0;

        for c in s.as_bytes() {
            act_val += *c as u64;
            act_val *= 17;
            act_val %= 256;
        }

        act_val
    }
}

impl Day for Day15 {
    fn day_nr(&self) -> String {
        String::from("15")
    }
    fn title(&self) -> String {
        String::from("Lens Library")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day15.txt");
        // let input = read_lines("data/day15-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        for s in self.input_data.iter() {
            let h = self.hash(s);
            solution += h;
        }
        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        // Map of labels to hashes, for quick lookup
        let mut hashes: HashMap<String, u64> = HashMap::new();
        // List of boxes, containing (label, focus length) options
        let mut boxes: Vec<Vec<Option<(String, u64)>>> = Vec::with_capacity(256);

        for _ in 0..256 {
            // best guess: I initialize each box with a capacity of 20, to
            // reduce reallocations.
            boxes.push(Vec::with_capacity(20));
        }

        let matcher = Regex::new(r"(.*)+(-|=)(\d+)?").unwrap();
        for s in self.input_data.iter() {
            // parse single operation:
            let groups = matcher.captures(s).unwrap();
            let label = String::from(&groups[1]);
            let op = *(&groups[2].chars().next().unwrap());
            let focal_length = match groups.get(3) {
                Some(g) => g.as_str().parse::<u64>().unwrap(),
                None => 0,
            };
            let box_nr = match hashes.get(&label) {
                Some(h) => *h,
                None => {
                    let h = self.hash(&label);
                    hashes.insert(label.clone(), h);
                    h
                }
            } as usize;

            // Proceed depending on the operation:
            match op {
                '=' => {
                    let idx_opt = &boxes[box_nr].iter().position(|entry| {
                        if let Some(e) = entry {
                            e.0 == label
                        } else {
                            false
                        }
                    });
                    match idx_opt {
                        Some(i) => {
                            boxes[box_nr][*i] = Some((label.clone(), focal_length));
                        }
                        None => {
                            boxes[box_nr].push(Some((label.clone(), focal_length)));
                        }
                    }
                }
                '-' => {
                    let idx_opt = &boxes[box_nr].iter().position(|entry| {
                        if let Some(e) = entry {
                            e.0 == label
                        } else {
                            false
                        }
                    });
                    match idx_opt {
                        Some(i) => {
                            boxes[box_nr][*i] = None;
                        }
                        None => {}
                    }
                }
                _ => panic!("Unknown op: {0}", op),
            };
        }

        // Calc outpus sums:
        for (i, b) in boxes.iter().enumerate() {
            let box_nr: u64 = i as u64 + 1;
            let entries = b.iter().filter(|e| e.is_some()).collect_vec();
            for (j, entry) in entries.iter().enumerate() {
               let slot_nr = j as u64 + 1;
               let focal_length = entry.as_ref().unwrap().1;
               solution += box_nr * slot_nr * focal_length;
            }
        }

        String::from(format!("{0}", solution))
    }
}
