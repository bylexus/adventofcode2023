use super::Day;
use alex_lib::{read_lines, types::Coord2d};
use itertools::Itertools;

#[derive(Debug)]
pub struct Day21 {
    input: Vec<String>,
    field: Vec<Vec<char>>,
    start_pos: Option<Coord2d>,
}

impl Day21 {
    pub fn new() -> Day21 {
        Day21 {
            input: Vec::new(),
            field: Vec::new(),
            start_pos: None,
        }
    }

    fn parse_input(&mut self) {
        for (y, line) in self.input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    self.start_pos = Some(Coord2d {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
            self.field.push(line.chars().collect());
        }
    }

    fn in_bounds(&self, pos: &Coord2d) -> bool {
        pos.y >= 0
            && pos.x >= 0
            && pos.y < self.field.len() as i64
            && pos.x < self.field[0].len() as i64
    }

    fn get_next_pos(&self, pos: &Coord2d) -> Vec<Coord2d> {
        let mut next_pos: Vec<Coord2d> = Vec::new();
        if !self.in_bounds(pos) {
            return next_pos;
        }
        let up = pos.up();
        if self.in_bounds(&up) && self.field[up.y as usize][up.x as usize] != '#' {
            next_pos.push(up);
        }
        let right = pos.right();
        if self.in_bounds(&right) && self.field[right.y as usize][right.x as usize] != '#' {
            next_pos.push(right);
        }
        let down = pos.down();
        if self.in_bounds(&down) && self.field[down.y as usize][down.x as usize] != '#' {
            next_pos.push(down);
        }
        let left = pos.left();
        if self.in_bounds(&left) && self.field[left.y as usize][left.x as usize] != '#' {
            next_pos.push(left);
        }

        next_pos
    }

    // Wraps around at the field's edges
    fn get_wrapped_coord(&self, pos: &Coord2d) -> Coord2d {
        let height = self.field.len() as i64;
        let width = self.field[0].len() as i64;
        let mut new_x = pos.x;
        let mut new_y = pos.y;
        // new_x = 10, width = 10 --> new_x = 0
        if new_x >= width {
            new_x = new_x % width;
        } else if new_x < 0 {
            // e.g.:
            // -  new_x = -1, width = 10 --> new_x = 9
            // -  new_x = -2, width = 10 --> new_x = 8
            // -  new_x = -8, width = 5 --> new_width = 2
            new_x = (new_x % width);
            if new_x < 0 {
                new_x += width;
            }
        }
        if new_y >= height {
            new_y = new_y % height;
        } else if new_y < 0 {
            new_y = (new_y % height);
            if new_y < 0 {
                new_y += height;
            }
        }

        Coord2d { x: new_x, y: new_y }
    }

    fn get_next_infinite_pos(&self, pos: &Coord2d) -> Vec<Coord2d> {
        let mut next_pos: Vec<Coord2d> = Vec::new();
        let up = pos.up();
        let up_wrapped = self.get_wrapped_coord(&up);
        if self.field[up_wrapped.y as usize][up_wrapped.x as usize] != '#' {
            next_pos.push(up);
        }
        let right = pos.right();
        let right_wrapped = self.get_wrapped_coord(&right);
        if self.field[right_wrapped.y as usize][right_wrapped.x as usize] != '#' {
            next_pos.push(right);
        }
        let down = pos.down();
        let down_wrapped = self.get_wrapped_coord(&down);
        if self.field[down_wrapped.y as usize][down_wrapped.x as usize] != '#' {
            next_pos.push(down);
        }
        let left = pos.left();
        let left_wrapped = self.get_wrapped_coord(&left);
        if self.field[left_wrapped.y as usize][left_wrapped.x as usize] != '#' {
            next_pos.push(left);
        }

        next_pos
    }

    fn print_field(&self, visited: &Vec<Coord2d>) {
        for (y, line) in self.field.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if visited.contains(&Coord2d {
                    x: x as i64,
                    y: y as i64,
                }) {
                    print!("O");
                } else {
                    print!("{}", c);
                }
            }
            println!()
        }
        println!()
    }
}

impl Day for Day21 {
    fn day_nr(&self) -> String {
        String::from("21")
    }
    fn title(&self) -> String {
        String::from("Step Counter")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day21.txt");
        let input = read_lines("data/day21-test.txt");
        self.input = input
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        let mut working_pos: Vec<Coord2d> = Vec::new();
        let mut next_pos: Vec<Coord2d> = Vec::new();
        let steps = 64;

        working_pos.push(self.start_pos.unwrap());

        for i in 0..steps {
            for pos in working_pos.iter() {
                next_pos.append(&mut self.get_next_pos(pos));
            }
            working_pos = next_pos.iter().unique().cloned().collect();
            // self.print_field(&working_pos);
            next_pos = Vec::new();
        }
        solution = working_pos.iter().unique().count() as u64;

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        let mut working_pos: Vec<Coord2d> = Vec::new();
        let mut next_pos: Vec<Coord2d> = Vec::new();
        // let steps = 64;
        let steps = 5000;

        working_pos.push(self.start_pos.unwrap());

        for i in 0..steps {
            for pos in working_pos.iter() {
                next_pos.append(&mut self.get_next_infinite_pos(pos));
            }
            working_pos = next_pos.iter().unique().cloned().collect();
            // self.print_field(&working_pos);
            next_pos = Vec::new();
        }
        solution = working_pos.iter().unique().count() as u64;

        String::from(format!("{0}", solution))
    }
}
