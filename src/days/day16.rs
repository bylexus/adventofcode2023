
use super::Day;
use adventofcode2023::{read_lines, };

#[derive(Debug)]
pub struct Day16 {
    input: Vec<String>,
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16 {
            input: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
    }
}

impl Day for Day16 {
    fn day_nr(&self) -> String {
        String::from("16")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day16.txt");
        let input = read_lines("data/day16-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }
}
