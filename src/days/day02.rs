use adventofcode2023::read_lines;

use super::Day;

#[derive(Debug)]
pub struct Day02 {
    input: Vec<String>,
}

impl Day02 {
    pub fn new() -> Day02 {
        Day02 { input: Vec::new() }
    }
}

impl Day for Day02 {
    fn day_nr(&self) -> String {
        String::from("02")
    }
    fn title(&self) -> String {
        String::from("Day 2: ")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day02.txt");
        let input = read_lines("data/day02-test.txt");
        self.input = input;
    }

    fn solve1(&self) -> String {
        let mut sum: u32 = 0;
        String::from(format!("{0}", sum))
    }
    fn solve2(&self) -> String {
        let mut sum: u64 = 0;
        String::from(format!("{0}", sum))
    }
}
