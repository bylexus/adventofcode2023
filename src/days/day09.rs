use super::Day;
use adventofcode2023::read_lines;

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
            for part in line.split_ascii_whitespace() {
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
        for data_line in &self.data {
            solution += calc_next_nr(data_line);
        }
        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: i64 = 0;
        // same as 1, but just with reversed input:
        for data_line in &self.data {
            let rev: Vec<i64> = data_line.iter().rev().cloned().collect();
            solution += calc_next_nr(&rev);
        }
        String::from(format!("{0}", solution))
    }
}

fn calc_next_nr(line: &Vec<i64>) -> i64 {
    let (diff_line, only_zeros) = calc_diff_line(line);
    let last = *line.last().unwrap();
    if only_zeros {
        return last;
    } else {
        return last + calc_next_nr(&diff_line);
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
