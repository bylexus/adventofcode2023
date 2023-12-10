use core::panic;

use super::Day;
use adventofcode2023::{
    read_lines,
    types::{Coord2d, Coord2dMap},
};

#[derive(Debug)]
pub struct Day10 {
    input: Vec<String>,
    pipe_map: Coord2dMap<char>,
    start: Option<Coord2d>,
    visited: Coord2dMap<bool>,
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {
            input: Vec::new(),
            pipe_map: Coord2dMap::new(),
            start: None,
            visited: Coord2dMap::new(),
        }
    }

    fn parse_input(&mut self) {
        for (y, line) in self.input.iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                let pos = Coord2d {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                if chr == 'S' {
                    self.start = Some(pos);
                }
                self.pipe_map.insert(pos, chr);
            }
        }
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
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 1;
        let start_coord = self.start.unwrap();
        self.visited.insert(start_coord, true);

        println!("Start: {0}", self.start.unwrap());
        println!("{0}", self.pipe_map);

        let mut last_pos = start_coord;
        let mut act_pos = self.find_entry_pipe(start_coord);

        println!("Entry: {0}", act_pos);

        // walk the pipes!
        loop {
            // find the next pipe
            solution += 1;
            let next_pos = self.next_pipe(act_pos, last_pos);
            let next_pipe = self.pipe_map.get(&next_pos).unwrap();
            if *next_pipe == 'S' {
                break;
            }
            last_pos = act_pos;
            act_pos = next_pos;
        }

        solution = match solution % 2 {
            0 => solution / 2,
            1 => solution / 2 + 1,
            _ => panic!("Invalid solution: {0}", solution),
        };

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }
}
