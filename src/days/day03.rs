use adventofcode2023::read_lines;

use crate::types::{Coord2d, Coord2dMap};

use super::Day;

#[derive(Debug)]
pub struct Day03 {
    input: Vec<String>,
    numbers: Coord2dMap<String>,
    symbols: Coord2dMap<char>,
}

impl Day03 {
    pub fn new() -> Day03 {
        Day03 {
            input: Vec::new(),
            numbers: Coord2dMap::new(),
            symbols: Coord2dMap::new(),
        }
    }

    fn parse_input(&mut self) {
        for y in 0..self.input.len() {
            let val = String::from(&self.input[y]);
            self.parse_line(&val, y as i64);
        }
    }

    // This is a little parser:
    /// Each line is parsed for numbers and symbols.
    /// If a number of a symbol is found, it is stored in its
    /// corresponding coordinate map: I store numbers and symbols in separate maps,
    /// for faster lookups later.
    fn parse_line(&mut self, line: &String, y: i64) {
        let mut nr_str = String::from("");
        let mut str_start: i64 = 0;

        if line.len() == 0 {
            return;
        }
        for x in 0..line.len() as i64 {
            let chr = line.chars().nth(x as usize).unwrap();
            if chr >= '0' && chr <= '9' {
                // if nr_str is empty, this is the start of a number:
                if nr_str.len() == 0 {
                    str_start = x;
                }
                nr_str.push(chr);
            } else {
                let coord = Coord2d { x, y };
                if chr != '.' {
                    self.symbols.insert(coord, chr);
                }
                // finished number, store it:
                if nr_str.len() > 0 {
                    let coord = Coord2d { x: str_start, y };
                    self.numbers.insert(coord, String::from(nr_str.as_str()));
                    nr_str.clear();
                }
            }
        }
        // finish a number at the end of the line:
        if nr_str.len() > 0 {
            let coord = Coord2d { x: str_start, y };
            self.numbers.insert(coord, String::from(nr_str.as_str()));
            nr_str.clear();
        }
    }

    /// Checks if the given number is a part number: a number is a part number
    /// if it is adjacent to a symbol.
    fn is_part_number(&self, nr_str: &str, nr_coord: &Coord2d) -> bool {
        // check all surrounding coordinates for a symbol:
        for y in (nr_coord.y - 1)..=(nr_coord.y + 1) {
            for x in (nr_coord.x - 1)..=(nr_coord.x + nr_str.len() as i64) {
                let check_entry = self.symbols.get(&Coord2d { x, y });
                if let Some(_) = check_entry {
                    return true;
                }
            }
        }
        false
    }

    /// Returns a list of numbers that are adjacent to the given coordinate.
    /// we need this to find adjacent numbers for gears. coord here would
    /// be a gear coordinate.
    fn find_surrounding_numbers(&self, coord: &Coord2d) -> Vec<String> {
        let mut numbers: Vec<String> = Vec::new();
        for (number_coord, nr_str) in self.numbers.iter() {
            if self.is_adjacent_number(coord, nr_str, number_coord) {
                numbers.push(String::from(nr_str));
            }
        }
        numbers
    }

    /// Checks if the given number/coordinate is adjacent to the given coordinate.
    fn is_adjacent_number(&self, coord: &Coord2d, nr_str: &str, number_coord: &Coord2d) -> bool {
        for y in (number_coord.y - 1)..=(number_coord.y + 1) {
            for x in (number_coord.x - 1)..=(number_coord.x + nr_str.len() as i64) {
                let check_coord = Coord2d { x, y };
                if check_coord == *coord {
                    return true;
                }
            }
        }
        false
    }
}

impl Day for Day03 {
    fn day_nr(&self) -> String {
        String::from("03")
    }
    fn title(&self) -> String {
        String::from("Day 3: Gear Ratios")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day03.txt");
        // let input = read_lines("data/day03-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&self) -> String {
        let mut sum = 0;
        for (number_coord, number_str) in self.numbers.iter() {
            if self.is_part_number(number_str, number_coord) {
                let nr = number_str.parse::<i64>().unwrap();
                sum += nr;
            }
        }
        String::from(format!("{0}", sum))
    }

    fn solve2(&self) -> String {
        let mut sum = 0;

        for (coord, symbol) in self.symbols.iter() {
            // check for surrounding numbers of gear symbol:
            if *symbol == '*' {
                let numbers = self.find_surrounding_numbers(&coord);
                if numbers.len() == 2 {
                    let nr1 = numbers[0].parse::<i64>().unwrap();
                    let nr2 = numbers[1].parse::<i64>().unwrap();
                    sum += nr1 * nr2;
                }
            }
        }
        String::from(format!("{0}", sum))
    }
}
