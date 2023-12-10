use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
pub struct Day17 {
    input: Vec<String>,
}

impl Day17 {
    pub fn new() -> Day17 {
        Day17 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day17 {
    fn day_nr(&self) -> String {
        String::from("17")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day17.txt");
        let input = read_lines("data/day17-test.txt");
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
