use super::Day;
use ::adventofcode2023::read_lines;
use regex::Regex;

pub struct Day00 {
    lines: Vec<String>,
}

impl Day00 {
    pub fn new() -> Day00 {
        Day00 { lines: Vec::new() }
    }
}

impl Day for Day00 {
    fn day_nr(&self) -> String {
        String::from("01")
    }
    fn title(&self) -> String {
        String::from("Hello, World!")
    }

    fn prepare(&mut self) {
        self.lines = read_lines("data/00.txt");
        // self.lines = read_lines("data/00-test.txt");
        self.lines.push(String::from(""));
    }

    fn solve1(&self) -> String {
        let mut maxsum: i64 = 0;
        let mut actsum: i64 = 0;

        let re = Regex::new(r"^\d+$").unwrap();

        for line in &self.lines {
            if !re.is_match(line) {
                if actsum > maxsum {
                    maxsum = actsum;
                }
                actsum = 0;
            } else {
                actsum += str::parse::<i64>(line).unwrap();
            }
        }

        String::from(format!("{0}", maxsum))
    }
    fn solve2(&self) -> String {
        let mut sum_per_elve: Vec<i64> = Vec::new();
        let re = Regex::new(r"^\d+$").unwrap();
        let mut actsum: i64 = 0;

        for line in &self.lines {
            if !re.is_match(line) {
                sum_per_elve.push(actsum);
                actsum = 0;
            } else {
                actsum += str::parse::<i64>(line).unwrap();
            }
        }
        sum_per_elve.sort();
        sum_per_elve.reverse();
        let total: i64 = sum_per_elve.iter().take(3).sum();

        String::from(format!("{0}", total))
    }
}
