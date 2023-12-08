use super::Day;
use adventofcode2023::read_lines;

#[derive(Debug)]
pub struct Day19 {
    input: Vec<String>,
}

impl Day19 {
    pub fn new() -> Day19 {
        Day19 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day19 {
    fn day_nr(&self) -> String {
        String::from("19")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day19.txt");
        let input = read_lines("data/day19-test.txt");
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
