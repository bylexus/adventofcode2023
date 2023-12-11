use std::collections::HashSet;

use super::Day;
use alex_lib::{read_lines, types::Coord2d};

/// A Galaxy is a set of unique galaxy coordinates,
/// and the maximal x and y coordinates.
#[derive(Debug, Clone)]
struct Galaxy {
    max_x: i64,
    max_y: i64,
    entries: HashSet<Coord2d>,
}

#[derive(Debug)]
pub struct Day11 {
    input: Vec<String>,
    initial_galaxy: Galaxy,
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 {
            input: Vec::new(),
            initial_galaxy: Galaxy {
                max_x: 0,
                max_y: 0,
                entries: HashSet::new(),
            },
        }
    }

    fn parse_input(&mut self) {
        // 1st, fill galaxy as-is:
        // Only store galaxies, no empty entries.
        // As this is a sparse universe, we need a minimum amount of memory.
        for (y, line) in self.input.iter().enumerate() {
            let y = y as i64;
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                if c == '#' {
                    self.initial_galaxy.entries.insert(Coord2d { x, y });
                    self.initial_galaxy.max_x = self.initial_galaxy.max_x.max(x);
                    self.initial_galaxy.max_y = self.initial_galaxy.max_y.max(y);
                }
            }
        }

        // self._print_galaxy(&self.initial_galaxy);
    }

    fn _print_galaxy(&self, galaxy: &Galaxy) {
        for y in 0..=galaxy.max_y {
            for x in 0..=galaxy.max_x {
                if let Some(_) = galaxy.entries.get(&Coord2d { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
    }

    /// Insert n empty rows below the given y row, moving all
    /// lower entries down by n.
    fn insert_empty_rows(&self, galaxy: &mut Galaxy, y: i64, count: i64) {
        let to_move: Vec<Coord2d> = galaxy
            .entries
            .iter()
            .filter(|c| c.y >= y)
            .map(|c| c.clone())
            .collect();
        for c in to_move {
            galaxy.entries.remove(&c);
            galaxy.entries.insert(Coord2d {
                x: c.x,
                y: c.y + count,
            });
            galaxy.max_y = galaxy.max_y.max(c.y + count);
        }
    }

    /// Insert n empty cols right of the given x col, moving all
    /// farther right entries by n.
    fn insert_empty_cols(&mut self, galaxy: &mut Galaxy, x: i64, count: i64) {
        let to_move: Vec<Coord2d> = galaxy
            .entries
            .iter()
            .filter(|c| c.x >= x)
            .map(|c| c.clone())
            .collect();
        for c in to_move {
            galaxy.entries.remove(&c);
            galaxy.entries.insert(Coord2d {
                x: c.x + count,
                y: c.y,
            });
            galaxy.max_x = galaxy.max_x.max(c.x + count);
        }
    }

    fn is_empty_row(&self, galaxy: &Galaxy, y: i64) -> bool {
        for x in 0..=galaxy.max_x {
            if galaxy.entries.contains(&Coord2d { x, y }) {
                return false;
            }
        }
        true
    }
    fn is_empty_col(&self, galaxy: &Galaxy, x: i64) -> bool {
        for y in 0..=galaxy.max_y {
            if galaxy.entries.contains(&Coord2d { x, y }) {
                return false;
            }
        }
        true
    }

    /// Calculates the minimum distance between all pairs of galaxies.
    /// The 'minimum distance' is simply the Manhattan Distance :-)
    fn calc_dist_sum(&self, galaxy: &Galaxy) -> i64 {
        let coords: Vec<Coord2d> = galaxy.entries.iter().map(|c| c.clone()).collect();

        let mut sum = 0;
        for i1 in 0..coords.len() - 1 {
            for i2 in i1 + 1..coords.len() {
                let c1 = coords[i1];
                let c2 = coords[i2];
                let dist = c1.manhattan_dist(&c2);
                sum += dist;
            }
        }
        sum
    }
}

impl Day for Day11 {
    fn day_nr(&self) -> String {
        String::from("11")
    }
    fn title(&self) -> String {
        String::from("Cosmic Expansion")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day11.txt");
        // let input = read_lines("data/day11-test.txt");
        self.input = input
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut galaxy = self.initial_galaxy.clone();

        // double empty rows:
        // work from bottom to top, insert an empty row below the actual row, if it's empty
        for y in (0..=galaxy.max_y).rev() {
            if self.is_empty_row(&self.initial_galaxy, y) {
                self.insert_empty_rows(&mut galaxy, y + 1, 1);
            }
        }

        // double empty cols:
        // work from right to left, insert an empty row right of the actual row, if it's empty
        for x in (0..=galaxy.max_x).rev() {
            if self.is_empty_col(&self.initial_galaxy, x) {
                self.insert_empty_cols(&mut galaxy, x + 1, 1);
            }
        }

        let solution = self.calc_dist_sum(&galaxy);
        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut galaxy = self.initial_galaxy.clone();

        let enlarge = 1000000 - 1;

        // enlarge empty rows:
        // work from bottom to top, insert empty rows below the actual row, if it's empty
        for y in (0..=galaxy.max_y).rev() {
            if self.is_empty_row(&self.initial_galaxy, y) {
                self.insert_empty_rows(&mut galaxy, y + 1, enlarge);
            }
        }

        // enlarge empty cols:
        // work from right to left, insert empty rows right of the actual row, if it's empty
        for x in (0..=galaxy.max_x).rev() {
            if self.is_empty_col(&self.initial_galaxy, x) {
                self.insert_empty_cols(&mut galaxy, x + 1, enlarge);
            }
        }

        let solution = self.calc_dist_sum(&galaxy);
        String::from(format!("{0}", solution))
    }
}
