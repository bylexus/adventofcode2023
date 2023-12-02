use adventofcode2023::read_lines;
use regex::Regex;

use super::Day;

#[derive(Debug)]
pub struct Day02 {
    input: Vec<String>,
}

impl Day02 {
    pub fn new() -> Day02 {
        Day02 { input: Vec::new() }
    }
}

impl Day for Day02 {
    fn day_nr(&self) -> String {
        String::from("02")
    }
    fn title(&self) -> String {
        String::from("Day 2: Cube Conundrum")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day02.txt");
        // let input = read_lines("data/day02-test.txt");
        self.input = input;
    }

    fn solve1(&self) -> String {
        let game_id_match = Regex::new(r"Game (\d+):").unwrap();
        let group_match = Regex::new(r"(\d+) (\w+)").unwrap();
        let mut sum = 0;
        for line in self.input.iter() {
            // extract game id:
            let game_id_opt = game_id_match.captures(line);
            if !game_id_opt.is_some() {
                continue;
            }
            let game_id_str = game_id_opt.unwrap().get(1).unwrap().as_str();
            let game_id = game_id_str.parse::<u32>().unwrap();

            // rest of string:
            let rest = &line[(line.find(':').unwrap() + 1)..];

            // split groups:
            let groups: Vec<String> = rest.split(';').map(|m| m.to_string()).collect();

            // parse each group:
            let mut valid = true;
            for group in groups {
                let parts: Vec<(u32, &str)> = group_match
                    .captures_iter(group.as_str())
                    .map(|m| {
                        let (full, [a, b]) = m.extract();
                        (a.parse::<u32>().unwrap(), b)
                    })
                    .collect();

                // Check if game is valid
                for part in parts {
                    if part.1 == "red" && part.0 > 12 {
                        valid = false;
                        break;
                    }
                    if part.1 == "green" && part.0 > 13 {
                        valid = false;
                        break;
                    }
                    if part.1 == "blue" && part.0 > 14 {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                sum += game_id;
            }
        }
        String::from(format!("{0}", sum))
    }
    fn solve2(&self) -> String {
        let game_id_match = Regex::new(r"Game (\d+):").unwrap();
        let group_match = Regex::new(r"(\d+) (\w+)").unwrap();
        let mut sum = 0;
        for line in self.input.iter() {
            // extract game id:
            let game_id_opt = game_id_match.captures(line);
            if !game_id_opt.is_some() {
                continue;
            }
            let game_id_str = game_id_opt.unwrap().get(1).unwrap().as_str();
            let game_id = game_id_str.parse::<u32>().unwrap();

            // rest of string:
            let rest = &line[(line.find(':').unwrap() + 1)..];

            // split groups:
            let groups: Vec<String> = rest.split(';').map(|m| m.to_string()).collect();

            // parse each group:
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            for group in groups {
                let parts: Vec<(u32, &str)> = group_match
                    .captures_iter(group.as_str())
                    .map(|m| {
                        let (full, [a, b]) = m.extract();
                        (a.parse::<u32>().unwrap(), b)
                    })
                    .collect();

                // Check if game is valid
                for part in parts {
                    if part.1 == "red" && part.0 > max_red {
                        max_red = part.0;
                    }
                    if part.1 == "green" && part.0 > max_green {
                        max_green = part.0;
                    }
                    if part.1 == "blue" && part.0 > max_blue {
                        max_blue = part.0;
                    }
                }
            }
            let power = max_red * max_green * max_blue;
            sum += power;
        }
        String::from(format!("{0}", sum))
    }
}
