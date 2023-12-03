use adventofcode2023::read_lines;

use crate::types::{Coord2d, Coord2dMap};

use super::Day;

#[derive(Debug)]
pub struct Day04 {
    input: Vec<String>,
}

impl Day04 {
    pub fn new() -> Day04 {
        Day04 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day04 {
    fn day_nr(&self) -> String {
        String::from("04")
    }
    fn title(&self) -> String {
        String::from("Day 4: xxxxxxxxxxx")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day04.txt");
        // let input = read_lines("data/day04-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&self) -> String {
        let mut sum = 0;
        String::from(format!("{0}", sum))
    }

    fn solve2(&self) -> String {
        let mut sum = 0;
        String::from(format!("{0}", sum))
    }
}
