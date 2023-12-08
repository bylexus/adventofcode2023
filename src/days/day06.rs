use super::Day;
use adventofcode2023::read_lines;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Day06 {
    input: Vec<String>,
    times: Vec<i64>,
    distances: Vec<i64>,
}

impl Day06 {
    pub fn new() -> Day06 {
        Day06 {
            input: Vec::new(),
            times: Vec::new(),
            distances: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        // Line 1: Times:
        let nr_regex = Regex::new(r"(\d+)").unwrap();
        let times = nr_regex.find_iter(self.input.get(0).unwrap());
        for t in times {
            self.times.push(t.as_str().parse::<i64>().unwrap());
        }

        // Line 2: Distances:
        let distances = nr_regex.find_iter(self.input.get(1).unwrap());
        for t in distances {
            self.distances.push(t.as_str().parse::<i64>().unwrap());
        }
    }
}

impl Day for Day06 {
    fn day_nr(&self) -> String {
        String::from("06")
    }
    fn title(&self) -> String {
        String::from("Wait For It")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day06.txt");
        // let input = read_lines("data/day06-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 1;

        for (i, time) in self.times.iter().enumerate() {
            let mut win_count: u64 = 0;
            let dist = *self.distances.get(i).unwrap();
            for hold in 0..=*time {
                let mydist = hold * (time - hold);
                if mydist > dist {
                    win_count += 1;
                }
            }
            println!("Win count for time {0}: {1}", time, win_count);
            solution *= win_count;
        }

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 1;
        let time = self
            .times
            .iter()
            .map(|nr| nr.to_string())
            .join("")
            .parse::<u64>()
            .unwrap();
        let dist = self
            .distances
            .iter()
            .map(|nr| nr.to_string())
            .join("")
            .parse::<u64>()
            .unwrap();

        let mut win_count: u64 = 0;
        for hold in 0..=time {
            let mydist = hold * (time - hold);
            if mydist > dist {
                win_count += 1;
            }
        }
        println!("Win count for time {0}: {1}", time, win_count);
        solution *= win_count;
        String::from(format!("{0}", solution))
    }
}
