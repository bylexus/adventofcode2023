use super::Day;
use adventofcode2023::read_lines;

#[derive(Debug)]
pub struct Day25 {
    input: Vec<String>,
}

impl Day25 {
    pub fn new() -> Day25 {
        Day25 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day25 {
    fn day_nr(&self) -> String {
        String::from("25")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day25.txt");
        let input = read_lines("data/day25-test.txt");
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
