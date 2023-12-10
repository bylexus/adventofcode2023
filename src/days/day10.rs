use core::panic;
use std::collections::HashSet;

use alex_lib::{types::{Coord2dMap, Coord2d, Direction}, read_lines};

use super::Day;

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
        let lines: i64 = self.input.len().try_into().unwrap();
        for (y, line) in self.input.iter().enumerate() {
            let y: i64 = y.try_into().unwrap();
            let line_len: i64 = line.len().try_into().unwrap();
            if y == 0 {
                // fill 4 corners with '.':
                self.pipe_map.insert(Coord2d { x: -1, y: -1 }, '.');
                self.pipe_map.insert(Coord2d { x: -1, y: lines }, '.');
                self.pipe_map.insert(Coord2d { x: line_len, y: -1 }, '.');
                self.pipe_map.insert(
                    Coord2d {
                        x: line_len,
                        y: lines,
                    },
                    '.',
                );
            }
            // insert a border on the left/right:
            self.pipe_map.insert(Coord2d { x: -1, y }, '.');
            self.pipe_map.insert(Coord2d { x: line_len, y }, '.');
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

    fn insert_start_pipe(&mut self, start_pos: Coord2d) {
        // Inserts the correct start pipe tile: The start now is a 'S',
        // but it need to be replaced by the pipe fitting in the start position:
        // we look at the surrounding pipes, and decide based on them.
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

    /// Returns the new direction after moving onto a tile from a
    /// specific direction.
    /// act_pos is the tile we're moved to (so where we are now),
    /// act_dir is the direction in which we were heading while moving.
    /// The function returns the direction in which we can move away from the tile.
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

    /// This function is called after all flood-filling is done: all non-circle tiles are now
    /// marked with either 'l' or 'r', depending on their position relative to the circle.
    /// Because the tile a -1, -1 is always the outer tile, we can determine which of 'r' or 'l'
    /// contains inner tile (just the opposite of the -1,-1 mark.)
    /// It returns the number of tiles that are contained by the circle pipe.
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

    fn fill_neighbour_areas(&mut self, act_pos: Coord2d, act_dir: Direction) {
        // flood-fill left and right side of the coordinate. Here, left/right is relative to the
        // actual heading direction:
        let left_coord = self.get_left_of(act_pos, act_dir);
        let right_coord = self.get_right_of(act_pos, act_dir);
        self.fill_area(left_coord, 'l');
        self.fill_area(right_coord, 'r');
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
        // check if the actual coord is a circle pipe position:
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

        // fill my surroundings:
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

        // find start direction
        let mut act_dir = self.get_new_dir(start_coord, Direction::DOWN);

        // println!("Start: {0}", self.start.unwrap());
        // println!("{0}", self.pipe_map);

        // walk the pipes!
        // Note that we don't need to check if the next tile is valid: The data is always
        // valid, so if we start from the given Start position, it's sure that we're on a circle path.
        let mut act_pos = start_coord;
        loop {
            self.circle_pipe.insert(act_pos);
            // do I need to turn, because I'm im a corner?
            act_dir = self.get_new_dir(act_pos, act_dir);
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
            self.fill_neighbour_areas(act_pos, act_dir);

            // do I need to turn, because I'm im a corner?
            let next_dir = self.get_new_dir(act_pos, act_dir);
            if next_dir != act_dir {
                // flood-fill again after turn, left and right side of the pipe. Here, left/right is relative to the
                // actual heading direction:
                act_dir = next_dir;
                self.fill_neighbour_areas(act_pos, act_dir);
            }

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
        // println!("Start: {0}", start_coord);
        // println!("{0}", self.pipe_map);

        let solution: usize = self.count_inner_tiles();
        String::from(format!("{0}", solution))
    }
}
