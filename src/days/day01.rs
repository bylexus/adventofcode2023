use adventofcode2023::read_lines;
use regex::Regex;

use super::Day;

#[derive(Debug)]
pub struct Day01 {
    input: Vec<String>,
    number_pattern: Regex,
}

impl Day01 {
    pub fn new() -> Day01 {
        Day01 {
            input: Vec::new(),
            number_pattern: regex::Regex::new(
                r"(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)",
            )
            .unwrap(),
        }
    }
}

impl Day for Day01 {
    fn day_nr(&self) -> String {
        String::from("01")
    }
    fn title(&self) -> String {
        String::from("Day 1: Trebuchet?!")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day01.txt");
        // let input = read_lines("data/day01-test2.txt");
        self.input = input;
    }

    fn solve1(&self) -> String {
        let mut sum: u32 = 0;
        for line in self.input.iter() {
            let first = line.chars().find(|c| c.is_numeric());
            let last = line.chars().rev().find(|c| c.is_numeric());
            if first.is_some() && last.is_some() {
                let nr: u32 = 10 * (first.unwrap().to_digit(10).unwrap())
                    + (last.unwrap().to_digit(10).unwrap());
                sum += nr;
            }
        }
        String::from(format!("{0}", sum))
    }
    fn solve2(&self) -> String {
        let mut sum: u64 = 0;
        for line in self.input.iter() {
            let all = find_nrs(&self.number_pattern, line);
            if all.len() > 0 {
                let first_str = all[0].as_str();
                let last_str = all[all.len() - 1].as_str();
                let first = str_to_nr(first_str);
                let last = str_to_nr(last_str);
                // if all.len() == 1 {
                //     println!("single entry: {0}: {1}", all[0], line);
                //     println!("{0} {1}", first, last);
                // }
                sum += (10 * first) + last;
            }
        }
        String::from(format!("{0}", sum))
    }
}

fn find_nrs(re: &Regex, line: &str) -> Vec<String> {
    // attention: we cannot just simply find all regex sub-groups, as there are some
    // mean entries like "eighthree" or "sevenine"..... So I loop over all smaller substrings, removing
    // one character at at time from the beginning, and test the remaining.
    let mut res = Vec::new();

    for m in 0..(line.len()) {
        let sub = &line[m..];
        let m = re.find(sub);
        if m.is_some() {
            let s = m.unwrap().as_str();
            res.push(String::from(s));
        }
    }

    // contains now a list of all numbers found, including overlapping
    res
}

fn str_to_nr(line: &str) -> u64 {
    match line {
        "one" => 1,
        "1" => 1,
        "two" => 2,
        "eightwo" => 2,
        "2" => 2,
        "three" => 3,
        "eighthree" => 3,
        "3" => 3,
        "four" => 4,
        "4" => 4,
        "five" => 5,
        "5" => 5,
        "six" => 6,
        "6" => 6,
        "seven" => 7,
        "7" => 7,
        "eight" => 8,
        "oneight" => 8,
        "threeight" => 8,
        "fiveight" => 8,
        "nineight" => 8,
        "8" => 8,
        "nine" => 9,
        "9" => 9,
        _ => panic!("Unknown number"),
    }
}
