use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use super::Day;
use alex_lib::{
    read_lines,
    types::{Coord2d, Direction},
};

#[derive(Debug)]
struct FieldInfo {
    coord: Coord2d,
    cost: i64,
    min_total_cost: i64,
    visited: bool,
    directions: Vec<Direction>,
}

#[derive(Debug)]
pub struct Day17 {
    input: Vec<String>,
    field: Vec<Vec<Rc<RefCell<FieldInfo>>>>,
}

impl Day17 {
    pub fn new() -> Day17 {
        Day17 {
            input: Vec::new(),
            field: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        for (y, line) in self.input.iter().enumerate() {
            let mut l = Vec::new();
            for (x, c) in line.chars().enumerate() {
                l.push(Rc::new(RefCell::new(FieldInfo {
                    coord: Coord2d {
                        x: x as i64,
                        y: y as i64,
                    },
                    cost: c.to_digit(10).unwrap() as i64,
                    min_total_cost: i64::MAX,
                    visited: false,
                    directions: Vec::new(),
                })));
            }
            self.field.push(l);
        }
    }
    fn print_field(&self) {
        for y in 0..self.field.len() {
            for x in 0..self.field[0].len() {
                let n = (*self.field[y][x]).borrow();
                print!("{}", n.cost);
            }
            println!();
        }
        println!();
    }

    /// Checks if the last 3 directions are the same in the given dir vector as the wanted
    /// direction: If yes, that means we cannot travel further in that direction.
    fn check_allowed_dir(&self, dir_vec: &Vec<Direction>, wanted_dir: Direction) -> bool {
        if dir_vec.len() < 3 {
            return true;
        }
        if dir_vec[dir_vec.len() - 1] != wanted_dir {
            return true;
        }
        if dir_vec[dir_vec.len() - 2] != wanted_dir {
            return true;
        }
        if dir_vec[dir_vec.len() - 3] != wanted_dir {
            return true;
        }
        return false;
    }

    /// Returns the field info for a given coordinate, or None if out of bounds
    fn get_field_info(&self, coord: Coord2d) -> Option<Rc<RefCell<FieldInfo>>> {
        if coord.x < 0 || coord.y < 0 {
            return None;
        }
        if coord.x >= self.field[0].len() as i64 || coord.y >= self.field.len() as i64 {
            return None;
        }
        Some(Rc::clone(&self.field[coord.y as usize][coord.x as usize]))
    }

    fn find_unvisited_neighbour_nodes(
        &self,
        act_node: &FieldInfo,
    ) -> Vec<(Rc<RefCell<FieldInfo>>, Direction)> {
        let mut res: Vec<(Rc<RefCell<FieldInfo>>, Direction)> = Vec::new();

        // up:
        if let Some(node) = self.get_field_info(act_node.coord.up()) {
            let n = (*node).borrow();
            if self.check_allowed_dir(&act_node.directions, Direction::UP) && !(n.visited) {
                res.push((Rc::clone(&node), Direction::UP));
            }
        }
        // right:
        if let Some(node) = self.get_field_info(act_node.coord.right()) {
            let n = (*node).borrow();
            if self.check_allowed_dir(&act_node.directions, Direction::RIGHT) && !(n.visited) {
                res.push((Rc::clone(&node), Direction::RIGHT));
            }
        }
        // down:
        if let Some(node) = self.get_field_info(act_node.coord.down()) {
            let n = (*node).borrow();
            if self.check_allowed_dir(&act_node.directions, Direction::DOWN) && !(n.visited) {
                res.push((Rc::clone(&node), Direction::DOWN));
            }
        }
        // left:
        if let Some(node) = self.get_field_info(act_node.coord.left()) {
            let n = (*node).borrow();
            if self.check_allowed_dir(&act_node.directions, Direction::LEFT) && !(n.visited) {
                res.push((Rc::clone(&node), Direction::LEFT));
            }
        }

        res
    }

    fn find_smallest_unvisited_node(&self) -> Option<Rc<RefCell<FieldInfo>>> {
        let mut res: Option<Rc<RefCell<FieldInfo>>> = None;

        for y in 0..self.field.len() {
            for x in 0..self.field[0].len() {
                let n = (*self.field[y][x]).borrow();
                if n.visited {
                    continue;
                }
                if res.is_none() {
                    res = Some(Rc::clone(&self.field[y][x]));
                } else if res.is_some() {
                    if n.min_total_cost < (*res.clone().unwrap()).borrow().min_total_cost {
                        res = Some(Rc::clone(&self.field[y][x]));
                    }
                }
            }
        }
        res
    }
}

impl Day for Day17 {
    fn day_nr(&self) -> String {
        String::from("17")
    }
    fn title(&self) -> String {
        String::from("Clumsy Crucible")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day17.txt");
        let input = read_lines("data/day17-test.txt");
        self.input = input
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        // self.print_field();
        let mut solution: i64 = 0;
        let start_node = self.field[0][0].clone();
        (*start_node).borrow_mut().visited = true;
        (*start_node).borrow_mut().min_total_cost = 0;

        // we put the start node including its directions in the queue for the first iteration
        // visit_queue.push(Rc::clone(&start_node));
        let mut act_node_opt: Option<Rc<RefCell<FieldInfo>>> = Some(start_node.clone());

        while act_node_opt.is_some() {
            let act_node = act_node_opt.take().unwrap();

            // mark actual node as visited:
            (*act_node).borrow_mut().visited = true;

            // find unvisited nodes that can be visited with the actual count of 3 same dir steps:
            let next_nodes = self.find_unvisited_neighbour_nodes(&(*act_node).borrow());

            // mark each unvisited node with the lowest cost, and add the direction to it to the directions array
            for (next_node, next_dir) in next_nodes {
                let mut n = (*next_node).borrow_mut();
                let new_cost = n.cost + act_node.borrow().min_total_cost;
                if new_cost < n.min_total_cost {
                    n.min_total_cost = new_cost;
                    n.directions = (*act_node.borrow()).directions.clone();
                    n.directions.push(next_dir);
                }else if new_cost == n.min_total_cost {
                    if n.directions.len() >= (*act_node.borrow()).directions.len() {
                        n.directions = (*act_node.borrow()).directions.clone();
                        n.directions.push(next_dir);
                    }
                }
            }

            act_node_opt = self.find_smallest_unvisited_node();
        }

        let end_node = self.field[self.field.len() - 1][self.field[0].len() - 1].clone();
        println!("end node: {:?}", end_node);
        solution = (*end_node).borrow().min_total_cost;

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }
}
