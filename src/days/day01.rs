use super::Day;

const MAX_NR: u32 = 1000;

#[derive(Debug)]
pub struct Day01 {
}

impl Day01 {
    pub fn new() -> Day01 {
        Day01 { }
    }
}

impl Day for Day01 {
    fn day_nr(&self) -> String {
        String::from("01")
    }
    fn title(&self) -> String {
        String::from("Hello, World!")
    }

    fn prepare(&mut self) {
        // Nothing to do
    }

    fn solve1(&self) -> String {
        let r: u32 = (1..MAX_NR).filter(|x| x % 3 == 0 || x % 5 == 0).sum();

        String::from(format!("{0}", r))
    }
    fn solve2(&self) -> String {
        String::from(format!("{0}", ""))
    }

}
