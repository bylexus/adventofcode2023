use std::collections::HashMap;

use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
pub struct Day14 {
    input: Vec<String>,
    field: Vec<Vec<char>>,
    field2: Vec<Vec<char>>,
    field_hashes: HashMap<String, u64>,
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 {
            input: Vec::new(),
            field: Vec::new(),
            field2: Vec::new(),
            field_hashes: HashMap::new(),
        }
    }

    fn parse_input(&mut self) {
        for line in self.input.iter() {
            let mut act_line = Vec::new();
            for c in line.chars() {
                act_line.push(c);
            }
            self.field.push(act_line);
        }
        self.field2 = self.field.clone();
    }

    fn _print_field(&self) {
        for line in self.field.iter() {
            for c in line.iter() {
                print!("{0}", c);
            }
            println!("")
        }
        println!("\n")
    }

    fn move_stone_north(&mut self, x: usize, mut y: usize) -> usize {
        let mut last_y = y;
        while y > 0 {
            y = y - 1;
            if self.field[y][x] == '.' {
                self.field[y + 1][x] = '.';
                self.field[y][x] = 'O';
                last_y = y;
            } else {
                break;
            }
        }
        last_y
    }
    fn move_stone_south(&mut self, x: usize, mut y: usize) -> usize {
        let mut last_y = y;
        while y < self.field.len() - 1 {
            y = y + 1;
            if self.field[y][x] == '.' {
                self.field[y - 1][x] = '.';
                self.field[y][x] = 'O';
                last_y = y;
            } else {
                break;
            }
        }
        last_y
    }
    fn move_stone_west(&mut self, mut x: usize, y: usize) -> usize {
        let mut last_x = x;
        while x > 0 {
            x = x - 1;
            if self.field[y][x] == '.' {
                self.field[y][x + 1] = '.';
                self.field[y][x] = 'O';
                last_x = x;
            } else {
                break;
            }
        }
        last_x
    }

    fn move_stone_east(&mut self, mut x: usize, y: usize) -> usize {
        let mut last_x = x;
        while x < self.field[y].len() - 1 {
            x = x + 1;
            if self.field[y][x] == '.' {
                self.field[y][x - 1] = '.';
                self.field[y][x] = 'O';
                last_x = x;
            } else {
                break;
            }
        }
        last_x
    }
    fn calc_load(&self, field: &Vec<Vec<char>>) -> u64 {
        let mut sum = 0;
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if field[y][x] == 'O' {
                    sum += (field.len() - y) as u64;
                }
            }
        }
        sum
    }
}

impl Day for Day14 {
    fn day_nr(&self) -> String {
        String::from("14")
    }
    fn title(&self) -> String {
        String::from("Parabolic Reflector Dish")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day14.txt");
        // let input = read_lines("data/day14-test.txt");
        self.input = input
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let solution: u64;
        // self._print_field();

        // move all rocks:
        for y in 0..self.field.len() {
            for x in 0..self.field[y].len() {
                if self.field[y][x] == 'O' {
                    // let new_y = self.move_stone_north(x, y);
                    // solution += (self.field.len() - new_y) as u64;
                    self.move_stone_north(x, y);
                }
            }
        }

        // self._print_field();
        solution = self.calc_load(&self.field);
        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let solution: u64;
        let mut cycle_count = 0;
        let cycles = 1000000000;

        self.field = self.field2.clone();

        while cycle_count < cycles {
            cycle_count += 1;
            // move all rocks north:
            for y in 0..self.field.len() {
                for x in 0..self.field[y].len() {
                    if self.field[y][x] == 'O' {
                        self.move_stone_north(x, y);
                    }
                }
            }
            // move all rocks west:
            for x in 0..self.field[0].len() {
                for y in 0..self.field.len() {
                    if self.field[y][x] == 'O' {
                        self.move_stone_west(x, y);
                    }
                }
            }
            // move all rocks south:
            for y in (0..self.field.len()).rev() {
                for x in 0..self.field[y].len() {
                    if self.field[y][x] == 'O' {
                        self.move_stone_south(x, y);
                    }
                }
            }
            // move all rocks east:
            for x in (0..self.field[0].len()).rev() {
                for y in 0..self.field.len() {
                    if self.field[y][x] == 'O' {
                        self.move_stone_east(x, y);
                    }
                }
            }

            // calc hash of actual field:
            let hash = format!("{:?}", self.field);
            // if we detect a cycle, we fast-forward the pointer to the last position before the
            // upper cycle limit, and only process the remaining part of the cycle:
            match self.field_hashes.get(&hash) {
                Some(first_seen) => {
                    let cycle_len = cycle_count - *first_seen;
                    cycle_count = (cycles - cycle_count) / cycle_len * cycle_len + cycle_count;
                    self.field_hashes.clear();
                }
                None => {
                    self.field_hashes.insert(hash, cycle_count);
                }
            }
        }
        let load = self.calc_load(&self.field);
        solution = load;

        String::from(format!("{0}", solution))
    }
}
