use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

use adventofcode2023::read_lines;

use crate::types::{Coord2d, Coord2dMap};

use super::Day;

#[derive(Debug)]
enum EntryType {
    Symbol(char),
    Number(String),
    Other(char),
}
impl Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(c) => write!(f, "{}", c),
            Self::Other(_) => write!(f, "_"),
            Self::Number(str) => write!(f, "{}", str),
        }
    }
}

#[derive(Debug)]
pub struct Day03 {
    input: Vec<String>,
    data: Coord2dMap<EntryType>,
}

impl Day03 {
    pub fn new() -> Day03 {
        Day03 {
            input: Vec::new(),
            data: Coord2dMap::new(),
        }
    }

    fn parse_input(&mut self) {
        for y in 0..self.input.len() {
            let val = String::from(&self.input[y]);
            self.parse_line(&val, y as i64);
        }
    }

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
                if chr == '.' {
                    self.data.insert(Coord2d { x, y }, EntryType::Other(chr));
                } else {
                    self.data.insert(Coord2d { x, y }, EntryType::Symbol(chr));
                }
                // finished number, store it:
                if nr_str.len() > 0 {
                    self.data.insert(
                        Coord2d { x: str_start, y },
                        EntryType::Number(String::from(nr_str.as_str())),
                    );
                    nr_str.clear();
                }
            }
        }
        // finish a number at the end of the line:
        if nr_str.len() > 0 {
            self.data.insert(
                Coord2d { x: str_start, y },
                EntryType::Number(String::from(nr_str.as_str())),
            );
            nr_str.clear();
        }
    }

    fn is_part_number(&self, start_coord: &Coord2d, number: EntryType) -> bool {
        match number {
            EntryType::Number(nr_str) => {
                // check all surrounding coordinates for a symbol:
                for y in (start_coord.y - 1)..=(start_coord.y + 1) {
                    for x in (start_coord.x - 1)..=(start_coord.x + nr_str.len() as i64) {
                        let check_entry = self.data.get(&Coord2d { x, y });
                        if let Some(EntryType::Symbol(_)) = check_entry {
                            return true;
                        }
                    }
                }

                false
            }
            _ => false,
        }
    }

    fn find_surrounding_numbers(&self, coord: &Coord2d) -> Vec<String> {
        let mut numbers: Vec<String> = Vec::new();
        for (entry_coords, entry) in self.data.iter() {
            if let EntryType::Number(n) = entry {
                if self.is_adjacent_number(coord, entry_coords, entry) {
                    numbers.push(String::from(n));
                }
            }
        }
        numbers
    }

    fn is_adjacent_number(
        &self,
        coord: &Coord2d,
        entry_coords: &Coord2d,
        number: &EntryType,
    ) -> bool {
        match number {
            EntryType::Number(nr_str) => {
                for y in (entry_coords.y - 1)..=(entry_coords.y + 1) {
                    for x in (entry_coords.x - 1)..=(entry_coords.x + nr_str.len() as i64) {
                        let check_coord = Coord2d { x, y };
                        if check_coord == *coord {
                            return true;
                        }
                    }
                }
                false
            }
            _ => false,
        }
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
        for (coord, entry) in self.data.iter() {
            if let EntryType::Number(n) = entry {
                if self.is_part_number(&coord, EntryType::Number(String::from(n))) {
                    let nr = n.parse::<i64>().unwrap();
                    sum += nr;
                }
            }
        }
        String::from(format!("{0}", sum))
    }

    fn solve2(&self) -> String {
        let mut sum = 0;

        for (coord, entry) in self.data.iter() {
            if let EntryType::Symbol(s) = entry {
                // check for surrounding numbers of gears:
                if *s == '*' {
                    let numbers = self.find_surrounding_numbers(&coord);
                    if numbers.len() == 2 {
                        let nr1 = numbers[0].parse::<i64>().unwrap();
                        let nr2 = numbers[1].parse::<i64>().unwrap();
                        sum += nr1 * nr2;
                    }
                }
            }
        }
        String::from(format!("{0}", sum))
    }
}
