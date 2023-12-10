use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
pub struct Day23 {
    input: Vec<String>,
}

impl Day23 {
    pub fn new() -> Day23 {
        Day23 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day23 {
    fn day_nr(&self) -> String {
        String::from("23")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day23.txt");
        let input = read_lines("data/day23-test.txt");
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
