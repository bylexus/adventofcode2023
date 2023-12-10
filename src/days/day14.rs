use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
pub struct Day14 {
    input: Vec<String>,
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day14 {
    fn day_nr(&self) -> String {
        String::from("14")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day14.txt");
        let input = read_lines("data/day14-test.txt");
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
