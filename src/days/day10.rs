use core::panic;
use std::collections::HashSet;

use super::Day;
use adventofcode2023::{
    read_lines,
    types::{Coord2d, Coord2dMap},
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug)]
pub struct Day10 {
    input: Vec<String>,
    pipe_map: Coord2dMap<char>,
    start: Option<Coord2d>,
    circle_pipe: HashSet<Coord2d>,
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {
            input: Vec::new(),
            pipe_map: Coord2dMap::new(),
            start: None,
            circle_pipe: HashSet::new(),
        }
    }

    fn parse_input(&mut self) {
        for (y, line) in self.input.iter().enumerate() {
            let y: i64 = y.try_into().unwrap();
            if y == 0 {
                // fill 4 corners with '.':
                self.pipe_map.insert(Coord2d { x: -1, y: -1 }, '.');
                self.pipe_map.insert(
                    Coord2d {
                        x: -1,
                        y: self.input.len().try_into().unwrap(),
                    },
                    '.',
                );
                self.pipe_map.insert(
                    Coord2d {
                        x: line.len().try_into().unwrap(),
                        y: -1,
                    },
                    '.',
                );
                self.pipe_map.insert(
                    Coord2d {
                        x: line.len().try_into().unwrap(),
                        y: self.input.len().try_into().unwrap(),
                    },
                    '.',
                );
            }
            // insert a border on the left/right:
            self.pipe_map.insert(Coord2d { x: -1, y }, '.');
            self.pipe_map.insert(
                Coord2d {
                    x: line.len().try_into().unwrap(),
                    y,
                },
                '.',
            );
            for (x, chr) in line.chars().enumerate() {
                let x: i64 = x.try_into().unwrap();
                // insert a border on the top:
                if y == 0 {
                    self.pipe_map.insert(Coord2d { x, y: -1 }, '.');
                }
                // Fill one below (will be overridden with real content, if not the last line):
                self.pipe_map.insert(Coord2d { x, y: y + 1 }, '.');

                // fill in the real thing:
                let pos = Coord2d { x, y };
                if chr == 'S' {
                    self.start = Some(pos);
                }
                self.pipe_map.insert(pos, chr);
            }
        }

        self.circle_pipe.insert(self.start.unwrap());
        self.insert_start_pipe(self.start.unwrap());
    }

    /// calc the next pipe pos
    /// Attention: this does NOT check if the next pipe is even possible:
    /// we assume that we are in a valid pipe circle.
    fn next_pipe(&self, act_pos: Coord2d, last_pos: Coord2d) -> Coord2d {
        let act_pipe = self.pipe_map.get(&act_pos).unwrap();

        if *act_pipe == '|' {
            let mut next_pos = act_pos.up();
            if next_pos == last_pos {
                next_pos = act_pos.down();
            }
            return next_pos;
        }

        if *act_pipe == '-' {
            let mut next_pos = act_pos.left();
            if next_pos == last_pos {
                next_pos = act_pos.right();
            }
            return next_pos;
        }

        if *act_pipe == 'L' {
            let mut next_pos = act_pos.up();
            if next_pos == last_pos {
                next_pos = act_pos.right();
            }
            return next_pos;
        }

        if *act_pipe == 'J' {
            let mut next_pos = act_pos.up();
            if next_pos == last_pos {
                next_pos = act_pos.left();
            }
            return next_pos;
        }

        if *act_pipe == '7' {
            let mut next_pos = act_pos.left();
            if next_pos == last_pos {
                next_pos = act_pos.down();
            }
            return next_pos;
        }
        if *act_pipe == 'F' {
            let mut next_pos = act_pos.right();
            if next_pos == last_pos {
                next_pos = act_pos.down();
            }
            return next_pos;
        }

        panic!("Invalid pipe found: {0} at {1}", act_pipe, act_pos);
    }

    fn insert_start_pipe(&mut self, start_pos: Coord2d) {
        // Inserts the correct start pipe tile: The start now is a 'S',
        // but it need to be replaced by the correct pipe:
        let top = self.pipe_map.get(&start_pos.up()).unwrap();
        let right = self.pipe_map.get(&start_pos.right()).unwrap();
        let bottom = self.pipe_map.get(&start_pos.down()).unwrap();
        let left = self.pipe_map.get(&start_pos.left()).unwrap();
        let can_up = ['|', '7', 'F'].contains(top);
        let can_right = ['-', '7', 'J'].contains(right);
        let can_down = ['|', 'J', 'L'].contains(bottom);
        let can_left = ['-', 'F', 'L'].contains(left);
        if can_up && can_down {
            self.pipe_map.insert(start_pos, '|');
        } else if can_up && can_left {
            self.pipe_map.insert(start_pos, 'J');
        } else if can_up && can_right {
            self.pipe_map.insert(start_pos, 'L');
        } else if can_right && can_left {
            self.pipe_map.insert(start_pos, '-');
        } else if can_right && can_down {
            self.pipe_map.insert(start_pos, 'F');
        } else if can_down && can_left {
            self.pipe_map.insert(start_pos, '7');
        } else {
            panic!("Unknown start tile!");
        }
    }

    fn find_entry_pipe(&self, start_pos: Coord2d) -> Coord2d {
        // up
        if let Some(pipe) = self.pipe_map.get(&start_pos.up()) {
            if *pipe == '|' || *pipe == 'F' || *pipe == '7' {
                return start_pos.up();
            }
        }
        // down
        if let Some(pipe) = self.pipe_map.get(&start_pos.down()) {
            if *pipe == '|' || *pipe == 'J' || *pipe == 'L' {
                return start_pos.down();
            }
        }
        // left
        if let Some(pipe) = self.pipe_map.get(&start_pos.left()) {
            if *pipe == '-' || *pipe == 'L' || *pipe == 'F' {
                return start_pos.left();
            }
        }

        // right
        if let Some(pipe) = self.pipe_map.get(&start_pos.right()) {
            if *pipe == '-' || *pipe == '7' || *pipe == 'J' {
                return start_pos.right();
            }
        }

        panic!("No start pipe found");
    }

    fn get_left_of(&self, act_pos: Coord2d, act_dir: Direction) -> Coord2d {
        match act_dir {
            Direction::UP => act_pos.left(),
            Direction::RIGHT => act_pos.up(),
            Direction::DOWN => act_pos.right(),
            Direction::LEFT => act_pos.down(),
        }
    }
    fn get_right_of(&self, act_pos: Coord2d, act_dir: Direction) -> Coord2d {
        match act_dir {
            Direction::UP => act_pos.right(),
            Direction::RIGHT => act_pos.down(),
            Direction::DOWN => act_pos.left(),
            Direction::LEFT => act_pos.up(),
        }
    }

    fn get_new_dir(&self, act_pos: Coord2d, act_dir: Direction) -> Direction {
        let act_pipe = *self.pipe_map.get(&act_pos).unwrap();
        match act_pipe {
            '|' => match act_dir {
                Direction::UP => Direction::UP,
                Direction::RIGHT => Direction::UP,
                Direction::LEFT => Direction::UP,
                Direction::DOWN => Direction::DOWN,
            },
            '-' => match act_dir {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::RIGHT,
                Direction::LEFT => Direction::LEFT,
                Direction::DOWN => Direction::RIGHT,
            },
            'F' => match act_dir {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::RIGHT,
                Direction::LEFT => Direction::DOWN,
                Direction::DOWN => Direction::DOWN,
            },
            'J' => match act_dir {
                Direction::UP => Direction::UP,
                Direction::RIGHT => Direction::UP,
                Direction::LEFT => Direction::LEFT,
                Direction::DOWN => Direction::LEFT,
            },
            '7' => match act_dir {
                Direction::UP => Direction::LEFT,
                Direction::RIGHT => Direction::DOWN,
                Direction::LEFT => Direction::LEFT,
                Direction::DOWN => Direction::DOWN,
            },
            'L' => match act_dir {
                Direction::UP => Direction::UP,
                Direction::RIGHT => Direction::RIGHT,
                Direction::LEFT => Direction::UP,
                Direction::DOWN => Direction::RIGHT,
            },
            _ => {
                panic!("Cannot determine new direction at {0}", act_pos);
            }
        }
    }

    fn count_inner_tiles(&self) -> usize {
        let inner_marker = match self.pipe_map.get(&Coord2d { x: -1, y: -1 }).unwrap() {
            'r' => 'l',
            'l' => 'r',
            _ => panic!("Cannot determine inner marker"),
        };
        self.pipe_map
            .iter()
            .filter(|(_, tile)| **tile == inner_marker)
            .count()
    }

    // Flood fill given area, starting at coord.
    // Stop if:
    // - the coord is outside the map
    // - the coord contains a pipe from the circle
    // - the coord is already filled
    // else, recursively fill the area
    fn fill_area(&mut self, coord: Coord2d, marker: char) {
        // check if we're outside the map:
        if let None = self.pipe_map.get(&coord) {
            return;
        }
        // check if the coord is a circle pipe position:
        if self.circle_pipe.contains(&coord) {
            return;
        }
        // already processed?
        let pipe = self.pipe_map.get(&coord).unwrap();
        if ['r', 'l'].contains(pipe) {
            return;
        }
        // mark myself:
        self.pipe_map.insert(coord, marker);

        self.fill_area(coord.up(), marker);
        self.fill_area(coord.down(), marker);
        self.fill_area(coord.left(), marker);
        self.fill_area(coord.right(), marker);
    }
}

impl Day for Day10 {
    fn day_nr(&self) -> String {
        String::from("10")
    }
    fn title(&self) -> String {
        String::from("Pipe Maze")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day10.txt");
        // let input = read_lines("data/day10-test1.txt");
        // let input = read_lines("data/day10-test2.txt");
        // let input = read_lines("data/day10-test2-1.txt");
        // let input = read_lines("data/day10-test2-2.txt");
        // let input = read_lines("data/day10-test2-3.txt");
        // let input = read_lines("data/day10-test2-4.txt");
        self.input = input
            .iter()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: usize = 0;
        let start_coord = self.start.unwrap();

        // println!("Start: {0}", self.start.unwrap());
        // println!("{0}", self.pipe_map);

        let mut last_pos = start_coord;
        let mut act_pos = self.find_entry_pipe(start_coord);

        // walk the pipes!
        loop {
            self.circle_pipe.insert(act_pos);
            // find the next pipe
            let next_pos = self.next_pipe(act_pos, last_pos);
            if next_pos == start_coord {
                break;
            }
            last_pos = act_pos;
            act_pos = next_pos;
        }

        solution = match self.circle_pipe.len() % 2 {
            0 => self.circle_pipe.len() / 2,
            1 => self.circle_pipe.len() / 2 + 1,
            _ => panic!("Invalid solution: {0}", solution),
        };

        String::from(format!("{0}", solution))
    }

    /**
     * Idee:
     * - Durchlaufen der Röhre, mit Heading-Direction
     * - markieren der Nachbar-Flächen als "links" oder "rechts" der Röhre
     * - dann Herausfinden der Aussenfläche: links oder rechts?
     * - somit gilt: alle "links"-Flächen sind innen, alle "rechts" flächen aussen (oder umgekehrt)
     */
    fn solve2(&mut self) -> String {
        let start_coord = self.start.unwrap();

        // find start direction
        let mut act_dir = self.get_new_dir(start_coord, Direction::DOWN);
        // println!("Start: {0}, dir: {1:?}", start_coord, act_dir);

        // Walk the pipes, fill left / right areas
        let mut act_pos = start_coord;
        loop {
            // flood-fill left and right side of the pipe. Here, left/right is relative to the
            // actual heading direction:
            let left_coord = self.get_left_of(act_pos, act_dir);
            let right_coord = self.get_right_of(act_pos, act_dir);
            self.fill_area(left_coord, 'l');
            self.fill_area(right_coord, 'r');

            // do I need to turn, because I'm im a corner?
            act_dir = self.get_new_dir(act_pos, act_dir);
            // flood-fill again after turn, left and right side of the pipe. Here, left/right is relative to the
            // actual heading direction:
            let left_coord = self.get_left_of(act_pos, act_dir);
            let right_coord = self.get_right_of(act_pos, act_dir);
            self.fill_area(left_coord, 'l');
            self.fill_area(right_coord, 'r');

            // now, move forward
            match act_dir {
                Direction::UP => act_pos = act_pos.up(),
                Direction::RIGHT => act_pos = act_pos.right(),
                Direction::DOWN => act_pos = act_pos.down(),
                Direction::LEFT => act_pos = act_pos.left(),
            }

            if act_pos == start_coord {
                break;
            }
        }


        let solution: usize = self.count_inner_tiles();
        String::from(format!("{0}", solution))
    }
}
