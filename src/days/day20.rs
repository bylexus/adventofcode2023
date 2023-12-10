use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
pub struct Day20 {
    input: Vec<String>,
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day20 {
    fn day_nr(&self) -> String {
        String::from("20")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day20.txt");
        let input = read_lines("data/day20-test.txt");
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
