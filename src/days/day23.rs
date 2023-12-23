use core::panic;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::Day;
use alex_lib::{
    read_lines,
    types::{Coord2d, Coord2dMap},
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
    coord: Coord2d,
}

#[derive(Debug)]
pub struct Day23 {
    input: Vec<String>,
    field: Coord2dMap<Rc<RefCell<FieldInfo>>>,
    start: Option<Coord2d>,
    end: Option<Coord2d>,
    path_memo: RefCell<HashMap<Coord2d, Option<u64>>>,
}

impl Day23 {
    pub fn new() -> Day23 {
        Day23 {
            input: Vec::new(),
            field: Coord2dMap::new(),
            start: None,
            end: None,
            path_memo: RefCell::new(HashMap::new()),
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
                        coord,
                    })),
                );
            }
        }
    }

    fn print_field(&self) {
        for y in 0..self.input.len() {
            for x in 0..self.input[0].len() {
                let coord = Coord2d {
                    x: x as i64,
                    y: y as i64,
                };
                let field = self.field.get(&coord).unwrap().as_ref().borrow();
                print!("{}", field.chr);
            }
            println!()
        }
        println!();
    }

    fn find_unvisited_neighbour_nodes(
        &self,
        act_node: &FieldInfo,
        visited: &HashSet<Coord2d>,
        treat_slopes_as_path: bool,
    ) -> Vec<Rc<RefCell<FieldInfo>>> {
        let mut res: Vec<Rc<RefCell<FieldInfo>>> = Vec::new();
        let mut next_coords: Vec<Coord2d> = Vec::new();

        if treat_slopes_as_path {
            next_coords.push(act_node.coord.up());
            next_coords.push(act_node.coord.right());
            next_coords.push(act_node.coord.down());
            next_coords.push(act_node.coord.left());
        } else {
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
        }

        for coord in next_coords {
            if let Some(node) = self.field.get(&coord) {
                // add node to the next visitable list of nodes, if not yet visited:
                let n = (*node.as_ref()).borrow();
                if !visited.contains(&coord) && n.chr != '#' {
                    res.push(Rc::clone(&node));
                }
            }
        }

        res
    }

    fn find_longest_path(
        &self,
        field: &FieldInfo,
        visited: &HashSet<Coord2d>,
        treat_slopes_as_path: bool,
    ) -> u64 {
        if field.coord == self.end.unwrap() {
            return visited.len() as u64;
        }
        // if let Some(memo) = self.path_memo.borrow().get(&field.coord) {
        //     if let Some(memo) = *memo {
        //         return memo;
        //     }
        // }

        // let actual_memo_coords = {
        //     let m = self.path_memo.borrow();
        //     m.keys().cloned().collect::<Vec<Coord2d>>()
        // };

        let mut my_visited = visited.clone();
        my_visited.insert(field.coord);
        let next_nodes =
            self.find_unvisited_neighbour_nodes(field, &my_visited, treat_slopes_as_path);
        let mut max = 0;
        for n in next_nodes {
            max = std::cmp::max(
                max,
                self.find_longest_path(&(*n).borrow(), &my_visited, treat_slopes_as_path),
            );
            // let mut memo = self.path_memo.borrow_mut();
            // for c in actual_memo_coords.iter() {
            //     if let Some(m) = memo.get_mut(c) {
            //         m.take();
            //     }
            // }
        }
        // self.path_memo.borrow_mut().insert(field.coord, Some(max));
        return max;
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
        let input = read_lines("data/day23.txt");
        // let input = read_lines("data/day23-test.txt");
        self.input = input
            .iter()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        // We cannot use Djikstra here, as we're looking for the LONGEST path.
        // So we need a backtracking algorithm, which tracks ALL available paths....
        // I use a recursive dfs approach:
        // 1. start at the start position, mark it as visited
        // 2. recursively calculate the path length of the next available nodes,
        //    skipping the ones in my local visited list: the visited list is the local node's
        //    list of previous nodes.
        // This leads to many paths and copied visited lists, but who cares :-)

        let solution: u64;
        let start_pos = self.start.unwrap();
        let start_field_rc = self.field.get(&start_pos).unwrap();
        let start_field = start_field_rc.as_ref().borrow();
        let mut start_visited = HashSet::new();
        start_visited.insert(start_pos);

        self.print_field();
        println!("Start: {:?}", self.start.unwrap());
        println!("End: {:?}", self.end.unwrap());

        solution = self.find_longest_path(&start_field, &start_visited, false);

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let solution: u64;
        let start_pos = self.start.unwrap();
        let start_field_rc = self.field.get(&start_pos).unwrap();
        let start_field = start_field_rc.as_ref().borrow();
        let mut start_visited = HashSet::new();
        start_visited.insert(start_pos);

        self.print_field();
        println!("Start: {:?}", self.start.unwrap());
        println!("End: {:?}", self.end.unwrap());

        solution = self.find_longest_path(&start_field, &start_visited, true);

        String::from(format!("{0}", solution))
    }
}
