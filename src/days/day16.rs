use log::{debug, error, log_enabled, info, Level};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex}, thread::available_parallelism,
};

use alex_lib::{
    read_lines,
    threads::ThreadPool,
    types::{Coord2d, Direction},
};

use super::Day;

/// For each field cell, we need to keep track
/// if the field was visited, and from which directions:
/// a field does not need to be visited again if it is entered
/// twice from the same direction (but can be visited from a non-visited direction)
#[derive(Debug)]
struct VisitedInfo {
    visited_count: u64,
    visited_dirs: Vec<Direction>,
}

#[derive(Debug)]
pub struct Day16 {
    input: Vec<String>,
    field: Vec<Vec<char>>,
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16 {
            input: Vec::new(),
            field: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        for line in self.input.iter() {
            let mut l = Vec::new();
            for c in line.chars() {
                l.push(c);
            }
            self.field.push(l);
        }
    }
}

impl Day for Day16 {
    fn day_nr(&self) -> String {
        String::from("16")
    }
    fn title(&self) -> String {
        String::from("The Floor Will Be Lava")
    }

    fn prepare(&mut self) {
        env_logger::init();
        let input = read_lines("data/day16.txt");
        // let input = read_lines("data/day16-test.txt");
        self.input = input
            .iter()
            .filter(|f| !f.is_empty())
            .map(|f| f.to_string())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let solution: u64;

        let start_pos = Coord2d { x: 0, y: 0 };
        let start_dir = Direction::RIGHT;
        solution = calc_energy(start_pos, start_dir, &self.field);

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let solution: u64;
        let mut start_positions: Vec<(Coord2d, Direction)> = Vec::new();

        // start from top row:
        for x in 0..self.field[0].len() as i64 {
            start_positions.push((Coord2d { x, y: 0 }, Direction::DOWN));
        }
        // start from bottom row:
        for x in 0..self.field[0].len() as i64 {
            start_positions.push((
                Coord2d {
                    x,
                    y: self.field.len() as i64 - 1,
                },
                Direction::UP,
            ));
        }
        // start from left row:
        for y in 0..self.field.len() as i64 {
            start_positions.push((Coord2d { x: 0, y }, Direction::RIGHT));
        }
        // start from right row:
        for y in 0..self.field.len() as i64 {
            start_positions.push((
                Coord2d {
                    x: self.field[0].len() as i64 - 1,
                    y,
                },
                Direction::LEFT,
            ));
        }

        let parallels = match available_parallelism() {
            Ok(n) => n.get(),
            Err(_) => 1
        };
        let mut tpool = ThreadPool::new(parallels);
        let results = Arc::new(Mutex::new(Vec::new()));


        // now, for each start position, calculate the energy
        for (start_pos, start_dir) in start_positions {
            let res = results.clone();
            let field = self.field.clone();
            tpool.enqueue(move |_| {
                let ret = calc_energy(start_pos, start_dir, &field);
                let mut res_arr = res.lock().unwrap();
                res_arr.push(ret);
            });
        }
        tpool.graceful_shutdown();

        // fetch the max solution out of our shared solution pool:
        solution = *results.lock().unwrap().iter().max().unwrap();

        String::from(format!("{0}", solution))
    }
}

/// executes a single move:
/// the coord is the new coordinate we are to step in,
/// the direction is the actual heading direction.
/// We just fill the queue with the next step(s), and return.
fn execute_move(
    coord: Coord2d,
    direction: Direction,
    move_q: &mut VecDeque<(Coord2d, Direction)>,
    visited: &mut HashMap<Coord2d, VisitedInfo>,
    field: &Vec<Vec<char>>,
) {
    // we're out of the field, so we do nothing:
    if coord.x < 0
        || coord.y < 0
        || coord.y >= field.len() as i64
        || coord.x >= field[0].len() as i64
    {
        return;
    }

    // if we were here from the same direction already, stop here:
    if let Some(v) = visited.get(&coord) {
        if v.visited_dirs.contains(&direction) {
            return;
        }
    }
    // Mark pos as visited:
    match visited.get_mut(&coord) {
        Some(v) => {
            v.visited_count += 1;
            v.visited_dirs.push(direction);
        }
        None => {
            visited.insert(
                coord,
                VisitedInfo {
                    visited_count: 1,
                    visited_dirs: vec![direction],
                },
            );
        }
    }
    // execute the move:
    let floor = field[coord.y as usize][coord.x as usize];
    match floor {
        // . --> move forward
        '.' => match direction {
            Direction::UP => {
                move_q.push_back((coord.up(), direction));
            }
            Direction::RIGHT => {
                move_q.push_back((coord.right(), direction));
            }
            Direction::DOWN => {
                move_q.push_back((coord.down(), direction));
            }
            Direction::LEFT => {
                move_q.push_back((coord.left(), direction));
            }
        },
        // - --> move forward if headed horizontal, spawn 2 dirs if headed vertical
        '-' => {
            if direction == Direction::UP || direction == Direction::DOWN {
                move_q.push_back((coord.right(), Direction::RIGHT));
                move_q.push_back((coord.left(), Direction::LEFT));
            } else if direction == Direction::RIGHT {
                move_q.push_back((coord.right(), direction));
            } else if direction == Direction::LEFT {
                move_q.push_back((coord.left(), direction));
            } else {
                panic!("Unknown direction");
            }
        }
        '|' => {
            if direction == Direction::LEFT || direction == Direction::RIGHT {
                move_q.push_back((coord.up(), Direction::UP));
                move_q.push_back((coord.down(), Direction::DOWN));
            } else if direction == Direction::UP {
                move_q.push_back((coord.up(), direction));
            } else if direction == Direction::DOWN {
                move_q.push_back((coord.down(), direction));
            } else {
                panic!("Unknown direction");
            }
        }
        '/' => match direction {
            Direction::UP => {
                move_q.push_back((coord.right(), Direction::RIGHT));
            }
            Direction::RIGHT => {
                move_q.push_back((coord.up(), Direction::UP));
            }
            Direction::DOWN => {
                move_q.push_back((coord.left(), Direction::LEFT));
            }
            Direction::LEFT => {
                move_q.push_back((coord.down(), Direction::DOWN));
            }
        },
        '\\' => match direction {
            Direction::UP => {
                move_q.push_back((coord.left(), Direction::LEFT));
            }
            Direction::RIGHT => {
                move_q.push_back((coord.down(), Direction::DOWN));
            }
            Direction::DOWN => {
                move_q.push_back((coord.right(), Direction::RIGHT));
            }
            Direction::LEFT => {
                move_q.push_back((coord.up(), Direction::UP));
            }
        },
        _ => panic!("Unknown floor: {}", floor),
    }
}

/// Starts the beam from the given start position and direction,
/// and calculates the energy of the beam.
///
/// Instead of a recursive algorithm, I use a queue here: I insert the start position into the queue,
/// and calc the next steps until the queue is empty.
/// Each move calc might add more steps to the queue (0, 1 or 2 more), depending
/// on the mirror it steps on on its way.
fn calc_energy(start_pos: Coord2d, start_dir: Direction, field: &Vec<Vec<char>>) -> u64 {
    let mut move_q = VecDeque::new();
    // self.visited = HashMap::new();
    let mut visited = HashMap::new();

    move_q.push_back((start_pos, start_dir));
    while move_q.len() > 0 {
        let (move_to, direction) = move_q.pop_front().unwrap();
        execute_move(move_to, direction, &mut move_q, &mut visited, field);
    }

    visited.len() as u64
}
