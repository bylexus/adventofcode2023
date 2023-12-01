use adventofcode2023::read_lines;

use super::Day;

const MAX_NR: u32 = 1000;

#[derive(Debug)]
pub struct Day01 {
    input: Vec<String>,
}

impl Day01 {
    pub fn new() -> Day01 {
        Day01 { input: Vec::new() }
    }
}

impl Day for Day01 {
    fn day_nr(&self) -> String {
        String::from("01")
    }
    fn title(&self) -> String {
        String::from("Hello, World!")
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
            let re = regex::Regex::new(
                r"(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)",
            )
            .unwrap();
            let lower = line.to_lowercase();
            let all: Vec<&str> = re.find_iter(lower.as_str()).map(|m| m.as_str()).collect();
            if all.len() > 0 {
                let firstStr = all[0];
                let lastStr = all[all.len() - 1];
                let first = strToNr(firstStr);
                let last = strToNr(lastStr);
                // println!("{0} {1}", first, last);
                sum += (10 * first) + last;
            }
        }
        String::from(format!("{0}", sum))
    }
}

fn strToNr(line: &str) -> u64 {
    match line {
        "one" => 1,
        "1" => 1,
        "two" => 2,
        "2" => 2,
        "three" => 3,
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
        "8" => 8,
        "nine" => 9,
        "9" => 9,
        _ => panic!("Unknown number"),
    }
}
