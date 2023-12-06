pub mod day_test;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day00;

pub use day_test::DayTest;
pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;
pub use day05::Day05;
pub use day06::Day06;
pub use day00::Day00;

pub trait Day {
    fn day_nr(&self) -> String;
    fn title(&self) -> String;
    fn prepare(&mut self);
    fn solve1(&mut self) -> String;
    fn solve2(&mut self) -> String;
}
