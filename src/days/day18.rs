use std::collections::{HashSet, VecDeque};

use super::Day;
use alex_lib::{
    read_lines,
    types::{Coord2d, Coord2dMap, Direction},
};
use regex::Regex;

type FieldEntry = char;

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    steps: usize,
    color: String,
}

#[derive(Debug)]
pub struct Day18 {
    input: Vec<String>,
    instructions: Vec<Instruction>,
    // contains the state of the field: initial hole ('#'), left flood area ('l'), right flood area ('r'), or empty (' ')
    dig_field: Coord2dMap<FieldEntry>,
    // contains a coordinate set of the initial circle area dig holes
    circle_set: HashSet<Coord2d>,
    // contains the colors per coordinate
    colors: Coord2dMap<String>,
}

impl Day18 {
    pub fn new() -> Day18 {
        Day18 {
            input: Vec::new(),
            instructions: Vec::new(),
            dig_field: Coord2dMap::new(),
            circle_set: HashSet::new(),
            colors: Coord2dMap::new(),
        }
    }

    fn parse_input(&mut self) {
        let matcher = Regex::new(r"(R|D|L|U)\s+(\d+)\s+\((.*)\)").unwrap();
        for line in self.input.iter() {
            let captures = matcher.captures(line);
            if let Some(groups) = captures {
                let instr = Instruction {
                    dir: Direction::from(groups[1].chars().next().unwrap()).unwrap(),
                    steps: groups[2].parse::<usize>().unwrap(),
                    color: groups[3].to_string(),
                };
                self.instructions.push(instr);
            }
        }
        // fill border line
        let mut act_coord = Coord2d { x: 0, y: 0 };
        for instr in self.instructions.iter() {
            for _ in 0..instr.steps {
                let new_coord = match instr.dir {
                    Direction::UP => act_coord.up(),
                    Direction::DOWN => act_coord.down(),
                    Direction::LEFT => act_coord.left(),
                    Direction::RIGHT => act_coord.right(),
                };
                self.dig_field.insert(new_coord, '#');
                self.colors.insert(new_coord, instr.color.clone());
                self.circle_set.insert(new_coord);
                act_coord = new_coord;
            }
        }
        // fill empty coords, including a 1-tile border around the field:
        let min_x = self.dig_field.min_x();
        let min_y = self.dig_field.min_y();
        let max_x = self.dig_field.max_x();
        let max_y = self.dig_field.max_y();
        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                if let None = self.dig_field.get(&Coord2d { x, y }) {
                    self.dig_field.insert(Coord2d { x, y }, '.');
                }
            }
        }
    }

    fn print_field(&self) {
        println!(
            "Field dimensions: {}x{}, minx={}, miny={}, maxx={}, maxy={}",
            self.dig_field.width(),
            self.dig_field.height(),
            self.dig_field.min_x(),
            self.dig_field.min_y(),
            self.dig_field.max_x(),
            self.dig_field.max_y()
        );
        for y in self.dig_field.min_y()..=self.dig_field.max_y() {
            for x in self.dig_field.min_x()..=self.dig_field.max_x() {
                let entry = self.dig_field.get(&Coord2d { x, y });
                match entry {
                    Some(c) => print!("{}", c),
                    None => print!(" "),
                };
            }
            println!();
        }
        println!();
    }

    /// Returns the new direction after moving onto a tile from a
    /// specific direction.
    /// act_pos is the tile we're moved to (so where we are now),
    /// act_dir is the direction in which we were heading while moving.
    /// The function returns the direction in which we can move away from the tile.
    fn get_new_dir(&self, act_pos: Coord2d, act_dir: Direction) -> Direction {
        match act_dir {
            Direction::UP => {
                if self.circle_set.contains(&act_pos.up()) {
                    return Direction::UP;
                } else if self.circle_set.contains(&act_pos.left()) {
                    return Direction::LEFT;
                } else if self.circle_set.contains(&act_pos.right()) {
                    return Direction::RIGHT;
                } else {
                    panic!("Cannot walk anywhere");
                }
            }
            Direction::RIGHT => {
                if self.circle_set.contains(&act_pos.right()) {
                    return Direction::RIGHT;
                } else if self.circle_set.contains(&act_pos.up()) {
                    return Direction::UP;
                } else if self.circle_set.contains(&act_pos.down()) {
                    return Direction::DOWN;
                } else {
                    panic!("Cannot walk anywhere");
                }
            }
            Direction::DOWN => {
                if self.circle_set.contains(&act_pos.down()) {
                    return Direction::DOWN;
                } else if self.circle_set.contains(&act_pos.left()) {
                    return Direction::LEFT;
                } else if self.circle_set.contains(&act_pos.right()) {
                    return Direction::RIGHT;
                } else {
                    panic!("Cannot walk anywhere");
                }
            }
            Direction::LEFT => {
                if self.circle_set.contains(&act_pos.left()) {
                    return Direction::LEFT;
                } else if self.circle_set.contains(&act_pos.up()) {
                    return Direction::UP;
                } else if self.circle_set.contains(&act_pos.down()) {
                    return Direction::DOWN;
                } else {
                    panic!("Cannot walk anywhere");
                }
            }
        }
    }

    // fn get_left_of(&self, act_pos: Coord2d, act_dir: Direction) -> Coord2d {
    //     match act_dir {
    //         Direction::UP => act_pos.left(),
    //         Direction::RIGHT => act_pos.up(),
    //         Direction::DOWN => act_pos.right(),
    //         Direction::LEFT => act_pos.down(),
    //     }
    // }
    // fn get_right_of(&self, act_pos: Coord2d, act_dir: Direction) -> Coord2d {
    //     match act_dir {
    //         Direction::UP => act_pos.right(),
    //         Direction::RIGHT => act_pos.down(),
    //         Direction::DOWN => act_pos.left(),
    //         Direction::LEFT => act_pos.up(),
    //     }
    // }

    // fn fill_neighbour_areas(&mut self, act_pos: Coord2d, act_dir: Direction) {
    //     // flood-fill left and right side of the coordinate. Here, left/right is relative to the
    //     // actual heading direction:
    //     let left_coord = self.get_left_of(act_pos, act_dir);
    //     let right_coord = self.get_right_of(act_pos, act_dir);
    //     self.fill_area(left_coord, 'l');
    //     self.fill_area(right_coord, 'r');
    // }

    /// Flood fill given area, starting at coord,
    /// using a queue for filling instead of recursion,
    /// as the recursion would lead to a stack overflow.
    fn fill_area(&mut self, coord: Coord2d) {
        let mut queue = VecDeque::new();
        queue.push_back(coord);

        while queue.len() > 0 {
            let coord = queue.pop_front().unwrap();
            if let Some(c) = self.dig_field.get(&coord) {
                if *c == '.' || *c == ' ' {
                    self.dig_field.insert(coord.clone(), '*');
                    queue.push_back(coord.up());
                    queue.push_back(coord.down());
                    queue.push_back(coord.left());
                    queue.push_back(coord.right());
                }
            }
        }
    }
}

impl Day for Day18 {
    fn day_nr(&self) -> String {
        String::from("18")
    }
    fn title(&self) -> String {
        String::from("Lavaduct Lagoon")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day18.txt");
        // let input = read_lines("data/day18-test.txt");
        self.input = input
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        self.parse_input();
    }

    /// This is actually a very same problem as day 10:
    /// flood fill an unknown area. I will re-use some of
    /// the code from this day.
    fn solve1(&mut self) -> String {
        let solution: i64;

        self.fill_area(Coord2d {
            x: self.dig_field.min_x(),
            y: self.dig_field.min_y(),
        });

        let fill_count: i64 = self.dig_field.iter().filter(|(_, c)| **c == '*').count() as i64;
        solution = self.dig_field.width() * self.dig_field.height() - fill_count;
        // self.print_field();
        String::from(format!("{0}", solution))
    }

    /// OK, this is far too big for a flood fill...
    /// But there is the "Shoelace Algorithm", to calculate the area of an irregular polygon:
    /// https://www.themathdoctors.org/polygon-coordinates-and-areas/
    /// Let's say the we have an n-gon of A1-A2-A3-...-An.
    /// 
    /// Let's say that the coordinates of Ai are (Xi, Yi) and we have all the Xi's and Yi's. 
    /// Then the area A is given by the 'ladder':
    /// 
    ///           |x1   y1|
    ///           |x2   y2|
    ///           |x3   y3|
    ///           |   .   |
    ///           |   .   |
    /// A = 1/2 * |   .   |
    ///           |   .   |
    ///           |   .   |
    ///           |   .   |
    ///           |xn   yn|
    ///           |x1   y1|    <-- Note that this is the first row,
    ///                            repeated. 
    /// 
    /// How do you evaluate the ladder?  I'll explain. 
    /// 
    /// You have to multiply each number of the first column with the number in the second column of the next row.
    /// Sum the products, and call the sum S1. 
    /// 
    /// Then multiply each number of the second column with the number in the first column of the next row. 
    /// Sum the products, and call the sum S2. 
    /// 
    /// Finally, 
    /// 
    /// A = 1/2 * |S1 - S2|
    /// 
    /// where |x| denotes the absolute value.
    /// 
    /// NOTE: I also read some other's source code, for example that one had the final clue:
    /// https://github.com/pkusensei/adventofcode2023/blob/4bad47d109c8b3c72ee44f2fd5964f4e369545fc/d18/src/lib.rs
    fn solve2(&mut self) -> String {
        let mut solution: i64 = 0;
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut coords: Vec<Coord2d> = Vec::new();

        // Re-parse input:
        let matcher = Regex::new(r"#([a-fA-F0-9]{5})(\d)").unwrap();
        for orig_instr in self.instructions.iter() {
            let groups = matcher.captures(&orig_instr.color).unwrap();
            instructions.push(Instruction {
                dir: match groups.get(2).unwrap().as_str() {
                    "0" => Direction::RIGHT,
                    "1" => Direction::DOWN,
                    "2" => Direction::LEFT,
                    "3" => Direction::UP,
                    _ => panic!("Unknown direction"),
                },
                color: "".to_string(),
                steps: usize::from_str_radix(groups.get(1).unwrap().as_str(), 16).unwrap(),
            });
        }

        // println!("Instructions: {:?}", instructions);

        // Fill in all coordinates as the shoelace algorithm requires:
        let mut act_coord = Coord2d {
            x: 0, y: 0
        };
        let mut boundary_length = 0;

        for instr in instructions.iter() {
            match instr.dir {
                Direction::UP => {
                    act_coord = act_coord.up_n(instr.steps as i64)
                }
                Direction::RIGHT => {
                    act_coord = act_coord.right_n(instr.steps as i64)
                }
                Direction::DOWN => {
                    act_coord = act_coord.down_n(instr.steps as i64)
                }
                Direction::LEFT => {
                    act_coord = act_coord.left_n(instr.steps as i64)
                }
            }
            boundary_length += instr.steps as i64;
            coords.push(act_coord);
        }
        // double the first entry:
        coords.push(coords[0]);

        // Now, calc the area using the Shoelace algorithm:
        let mut s1:i64 = 0;
        let mut s2:i64 = 0;
        for i in 0..coords.len()-1 {
            s1 += coords[i].x * coords[i + 1].y;
            s2 += coords[i].y * coords[i + 1].x;
        }
        // also add the boundary length (half of it, as the shoelace includes the boundary on one side),
        // and remove 1 from the solution (start of the boundary was visited twice):
        solution = (s1 - s2).abs() / 2 + boundary_length / 2 + 1;

        String::from(format!("{0}", solution))
    }
}
