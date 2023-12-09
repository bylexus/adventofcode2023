use std::{cell::RefCell, rc::Rc};

use super::Day;
use adventofcode2023::read_lines;
use regex::Regex;

#[derive(Debug)]
pub struct Day09 {
    input: Vec<String>,
    data: Vec<Vec<i64>>,
}

impl Day09 {
    pub fn new() -> Day09 {
        Day09 {
            input: Vec::new(),
            data: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        for line in self.input.iter() {
            let mut entries = Vec::new();
            for part in line.split_ascii_whitespace(){
                let nr = part.parse::<i64>().unwrap();
                entries.push(nr);
            }
            if entries.len() > 0 {
                self.data.push(entries);
            }
        }
    }
}

impl Day for Day09 {
    fn day_nr(&self) -> String {
        String::from("09")
    }
    fn title(&self) -> String {
        String::from("Mirage Maintenance")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day09.txt");
        // let input = read_lines("data/day09-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: i64 = 0;
        println!("Data: {:?}", self.data);
        for data_line in &self.data {
            let mut only_zeros = false;
            let mut diff_lines = Vec::new();
            diff_lines.push((*data_line).clone());
            // calc diff lines:
            while !only_zeros {
                let res = calc_diff_line(diff_lines.last().unwrap());
                only_zeros = res.1;
                // if res.0.len() == 0 {
                //     diff_lines.push(Vec::from([0]));
                // } else {
                    diff_lines.push(res.0);
                // }
            }
            // calc resulting last numbers:
            println!("\n\nDiff lines: {:?}", diff_lines);
            for i in (0..(diff_lines.len() - 1)).rev() {
                let act_line = diff_lines[i].clone();
                let next_line = diff_lines[i + 1].clone();
                let last_el_act = act_line.last().unwrap();
                let last_el_next = next_line.last().unwrap();
                diff_lines[i].push(last_el_act + last_el_next);
                println!("   Diff line: {:?}", diff_lines[i]);
            }
            solution += diff_lines[0].last().unwrap();
            println!("Diff lines: {:?}", diff_lines);
        }
        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }
}

fn calc_diff_line(line: &Vec<i64>) -> (Vec<i64>, bool) {
    let mut res = Vec::new();
    let mut only_zeros = true;
    for i in 1..line.len() {
        let diff = line[i] - line[i - 1];
        if diff != 0 {
            only_zeros = false;
        }
        res.push(diff);
    }
    (res, only_zeros)
}
