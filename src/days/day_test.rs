use super::Day;
use crate::types::{Coord2d, Coord2dMap};

pub struct DayTest {
    coords: Coord2dMap<String>,
}

impl DayTest {
    pub fn new() -> DayTest {
        DayTest {
            coords: Coord2dMap::new(),
        }
    }
}

impl Day for DayTest {
    fn day_nr(&self) -> String {
        String::from("00-TEST")
    }
    fn title(&self) -> String {
        String::from("TEST")
    }

    fn prepare(&mut self) {
        // fill some 2d coords:
        for y in 0..30 {
            for x in 0..50 {
                self.coords.insert(
                    Coord2d { x, y },
                    match y > 0 && x % y == 0 {
                        true => "⭐️".to_string(),
                        false => " ".to_string(),
                    },
                );
            }
        }
    }

    fn solve1(&self) -> String {
        String::from(format!(
            "\n{0}\nDimension: {1}x{2}\n",
            self.coords,
            self.coords.width(),
            self.coords.height()
        ))
    }

    fn solve2(&self) -> String {
        String::from(format!("{0}", ""))
    }
}
