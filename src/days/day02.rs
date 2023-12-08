use adventofcode2023::read_lines;
use regex::Regex;

use super::Day;

#[derive(Debug)]
pub struct Day02 {
    input: Vec<String>,
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    groups: Vec<(u32, String)>,
}

impl Day02 {
    pub fn new() -> Day02 {
        Day02 {
            input: Vec::new(),
            games: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        let game_id_match = Regex::new(r"Game (\d+):").unwrap();
        let group_match = Regex::new(r"(\d+) (\w+)").unwrap();
        for line in self.input.iter() {
            let mut game = Game {
                id: 0,
                groups: Vec::new(),
            };

            // extract game id:
            let game_id_opt = game_id_match.captures(line);
            if !game_id_opt.is_some() {
                continue;
            }
            let game_id_str = game_id_opt.unwrap().get(1).unwrap().as_str();
            let game_id = game_id_str.parse::<u32>().unwrap();
            game.id = game_id;

            // rest of string:
            let rest = &line[(line.find(':').unwrap() + 1)..];

            // split groups:
            let groups: Vec<String> = rest.split(';').map(|m| m.to_string()).collect();

            // parse each group:
            for group in groups {
                group_match
                    .captures_iter(group.as_str())
                    .map(|m| {
                        let (_, [a, b]) = m.extract();
                        (a.parse::<u32>().unwrap(), b)
                    })
                    // emit a tuple (nr, color) into the game.groups vec:
                    .for_each(|(nr, color)| {
                        game.groups.push((nr, color.to_string()));
                    });
            }
            self.games.push(game);
        }
    }
}

impl Day for Day02 {
    fn day_nr(&self) -> String {
        String::from("02")
    }
    fn title(&self) -> String {
        String::from("Cube Conundrum")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day02.txt");
        // let input = read_lines("data/day02-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut sum = 0;
        for game in self.games.iter() {
            // Check if game is valid
            let mut valid = true;
            for part in game.groups.iter() {
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
            if valid {
                sum += game.id;
            }
        }
        String::from(format!("{0}", sum))
    }
    fn solve2(&mut self) -> String {
        let mut sum = 0;
        for game in self.games.iter() {
            // parse each group:
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            for group in game.groups.iter() {
                // Check if game is valid
                if group.1 == "red" && group.0 > max_red {
                    max_red = group.0;
                }
                if group.1 == "green" && group.0 > max_green {
                    max_green = group.0;
                }
                if group.1 == "blue" && group.0 > max_blue {
                    max_blue = group.0;
                }
            }
            let power = max_red * max_green * max_blue;
            sum += power;
        }
        String::from(format!("{0}", sum))
    }
}
