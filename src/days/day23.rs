use core::panic;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use super::Day;
use alex_lib::{
    read_lines,
    types::{Coord2d, Coord2dMap, Direction},
};

#[derive(Debug, Clone, Copy)]
enum FieldType {
    Forest,
    Path,
    Slope(char),
}

#[derive(Debug)]
struct FieldInfo {
    chr: char,
    field_type: FieldType,
    total_steps: u64,
    coord: Coord2d,
}

#[derive(Debug)]
pub struct Day23 {
    input: Vec<String>,
    field: Coord2dMap<Rc<RefCell<FieldInfo>>>,
    start: Option<Coord2d>,
    end: Option<Coord2d>,
    visited: HashSet<Coord2d>,
}

impl Day23 {
    pub fn new() -> Day23 {
        Day23 {
            input: Vec::new(),
            field: Coord2dMap::new(),
            start: None,
            end: None,
            visited: HashSet::new(),
        }
    }

    fn parse_input(&mut self) {
        for (y, line) in self.input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Coord2d {
                    x: x as i64,
                    y: y as i64,
                };
                // Detect start position:
                if y == 0 && c == '.' {
                    self.start = Some(coord);
                    self.visited.insert(coord);
                }
                // Detect end position:
                if y == self.input.len() - 1 && c == '.' {
                    self.end = Some(coord);
                }
                self.field.insert(
                    coord,
                    Rc::new(RefCell::new(FieldInfo {
                        chr: c,
                        field_type: match c {
                            '.' => FieldType::Path,
                            '#' => FieldType::Forest,
                            _ => FieldType::Slope(c),
                        },
                        total_steps: 0,
                        coord,
                    })),
                );
            }
        }
    }

    fn print_field(&self) {
        let mut visited_counter = 0;
        for y in 0..self.input.len() {
            for x in 0..self.input[0].len() {
                let coord = Coord2d {
                    x: x as i64,
                    y: y as i64,
                };
                let field = self.field.get(&coord).unwrap().borrow();
                if self.visited.contains(&coord) {
                    print!("O");
                    visited_counter += 1;
                } else {
                    print!("{}", field.chr);
                }
            }
            println!()
        }
        println!("Visited: {}\n", visited_counter);
    }

    fn find_unvisited_neighbour_nodes(&self, act_node: &FieldInfo) -> Vec<Rc<RefCell<FieldInfo>>> {
        let mut res: Vec<Rc<RefCell<FieldInfo>>> = Vec::new();
        let mut next_coords: Vec<Coord2d> = Vec::new();

        match act_node.field_type {
            FieldType::Path => {
                next_coords.push(act_node.coord.up());
                next_coords.push(act_node.coord.right());
                next_coords.push(act_node.coord.down());
                next_coords.push(act_node.coord.left());
            }
            FieldType::Slope(s) => match s {
                '>' => next_coords.push(act_node.coord.right()),
                '^' => next_coords.push(act_node.coord.up()),
                '<' => next_coords.push(act_node.coord.left()),
                'v' => next_coords.push(act_node.coord.down()),
                _ => panic!("Unknown slope type: {}", s),
            },
            _ => {
                panic!("Unsupported field type: {:?}", act_node.field_type)
            }
        }

        for coord in next_coords {
            if let Some(node) = self.field.get(&coord) {
                // add node to the next visitable list of nodes, if not yet visited:
                let n = (*node).borrow();
                if !self.visited.contains(&coord) && n.chr != '#' {
                    res.push(Rc::clone(&node));
                }
            }
        }

        res
    }
}

impl Day for Day23 {
    fn day_nr(&self) -> String {
        String::from("23")
    }
    fn title(&self) -> String {
        String::from("A Long Walk")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day23.txt");
        let input = read_lines("data/day23-test.txt");
        self.input = input
            .iter()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        let mut visit_queue: Vec<Rc<RefCell<FieldInfo>>> = Vec::new();
        let start_pos = self.start.unwrap();
        let end_pos = self.end.unwrap();

        self.print_field();
        println!("Start: {:?}", self.start.unwrap());
        println!("End: {:?}", self.end.unwrap());

        // we put the start node in the queue for the first iteration
        let start_field = self.field.get(&start_pos).unwrap();
        visit_queue.push(Rc::clone(&start_field));

        let mut act_field_rc: Rc<RefCell<FieldInfo>>;
        while visit_queue.len() > 0 {
            {
                // read last node from queue (queue must be sorted, as we pop from the end):
                // node with most steps is at the end:
                act_field_rc = visit_queue.pop().unwrap();
                let mut act_field = act_field_rc.borrow_mut();
                let act_pos = act_field.coord;

                // Found the exit, end!
                // if act_pos == end_pos {
                //     break;
                // }

                // find unvisited nodes that can be visited:
                let next_nodes = self.find_unvisited_neighbour_nodes(&act_field);

                // add them to the queue, and sort the queue so that the nodes with the most steps is at the end:
                for next_node in next_nodes {
                    let mut n = next_node.borrow_mut();
                    if n.total_steps < (act_field.total_steps + 1) {
                        n.total_steps = act_field.total_steps + 1;
                    }
                    visit_queue.push(Rc::clone(&next_node));
                }

                // mark actual node as visited:
                self.visited.insert(act_pos);
            }
            visit_queue.sort_by(|a, b| a.borrow().total_steps.cmp(&b.borrow().total_steps));
        }

        self.print_field();
        println!("End field: {:?}", self.field.get(&end_pos).unwrap());

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }
}
