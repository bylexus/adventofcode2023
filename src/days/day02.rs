use super::Day;

const MAX_NR: u32 = 1000;

#[derive(Debug)]
pub struct Day02 {
}

impl Day02 {
    pub fn new() -> Day02 {
        Day02 { }
    }
}

impl Day for Day02 {
    fn day_nr(&self) -> String {
        String::from("01")
    }
    fn title(&self) -> String {
        String::from("Problem 1")
    }

    fn prepare(&mut self) {
        // Nothing to do
    }

    fn solve1(&self) -> String {
        String::from(format!("{0}", ""))
    }
    fn solve2(&self) -> String {
        String::from(format!("{0}", ""))
    }
}
