use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
pub struct Day12 {
    input: Vec<String>,
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day12 {
    fn day_nr(&self) -> String {
        String::from("12")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day12.txt");
        let input = read_lines("data/day12-test.txt");
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
