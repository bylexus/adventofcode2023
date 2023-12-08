use super::Day;
use adventofcode2023::read_lines;

#[derive(Debug)]
pub struct Day24 {
    input: Vec<String>,
}

impl Day24 {
    pub fn new() -> Day24 {
        Day24 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day24 {
    fn day_nr(&self) -> String {
        String::from("24")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day24.txt");
        let input = read_lines("data/day24-test.txt");
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
