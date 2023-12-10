use alex_lib::types::{Coord2dMap, Coord2d};

use super::Day;

enum TestValue {
    ValueA(String),
    ValueB(i64),
    None,
}

pub struct DayTest {
    coords: Coord2dMap<String>,
    a_value: TestValue,
}

impl DayTest {
    pub fn new() -> DayTest {
        DayTest {
            coords: Coord2dMap::new(),
            a_value: TestValue::None,
        }
    }

    fn create_value(&self, input: i64) -> TestValue {
        if input < 0 {
            return TestValue::ValueA(input.to_string());
        } else {
            return TestValue::ValueB(input);
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

        self.a_value = self.create_value(42);
    }

    fn solve1(&mut self) -> String {
        String::from(format!(
            "\n{0}\nDimension: {1}x{2}\n",
            self.coords,
            self.coords.width(),
            self.coords.height()
        ))
    }

    fn solve2(&mut self) -> String {
        String::from(format!("{0}", ""))
    }
}
