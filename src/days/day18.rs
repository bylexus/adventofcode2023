use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
pub struct Day18 {
    input: Vec<String>,
}

impl Day18 {
    pub fn new() -> Day18 {
        Day18 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day18 {
    fn day_nr(&self) -> String {
        String::from("18")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day18.txt");
        let input = read_lines("data/day18-test.txt");
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
