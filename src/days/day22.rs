use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
pub struct Day22 {
    input: Vec<String>,
}

impl Day22 {
    pub fn new() -> Day22 {
        Day22 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day22 {
    fn day_nr(&self) -> String {
        String::from("22")
    }
    fn title(&self) -> String {
        String::from("xxxxxx")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day22.txt");
        let input = read_lines("data/day22-test.txt");
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
