pub mod day01;
pub mod day02;
pub mod day00;

pub use day01::Day01;
pub use day02::Day02;
pub use day00::Day00;

pub trait Day {
    fn day_nr(&self) -> String;
    fn title(&self) -> String;
    fn prepare(&mut self);
    fn solve1(&self) -> String;
    fn solve2(&self) -> String;
}
