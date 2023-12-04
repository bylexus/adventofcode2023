use std::collections::HashMap;

use super::Day;
use adventofcode2023::read_lines;
use regex::Regex;

#[derive(Debug)]
struct Card {
    win: Vec<i64>,
    own: Vec<i64>,
}

#[derive(Debug)]
pub struct Day04 {
    input: Vec<String>,
    cards: Vec<Card>,
    winning_cards_memo: HashMap<u32, u32>,
}

impl Day04 {
    pub fn new() -> Day04 {
        Day04 {
            input: Vec::new(),
            cards: Vec::new(),
            winning_cards_memo: HashMap::new(),
        }
    }

    fn parse_input(&mut self) {
        let matcher = Regex::new(r".*:(.*)\|(.*)").unwrap();
        for line in &self.input {
            if line.trim().is_empty() {
                continue;
            }
            let caps = matcher.captures(line).unwrap();
            if caps.len() < 3 {
                continue;
            }
            let win = caps.get(1).unwrap().as_str();
            let own = caps.get(2).unwrap().as_str();
            let win_cards: Vec<i64> = win
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
            let own_cards: Vec<i64> = own
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();

            self.cards.push(Card {
                win: win_cards,
                own: own_cards,
            });
        }
    }

    fn count_cards(&self, card_nrs: &Vec<u32>) -> u64 {
        let mut counter: u64 = 0;
        // 1st, count all given cards:
        counter += card_nrs.len() as u64;

        // process each card:
        let mut cards_to_process: Vec<u32> = Vec::new();
        for card_nr in card_nrs.iter() {
            let winning_cards = match self.winning_cards_memo.get(card_nr) {
                Some(v) => *v,
                None => 0,
            };
            if winning_cards > 0 {
                // keep a memo of the sub-cards to process:
                cards_to_process.extend((card_nr + 1)..=(*card_nr + winning_cards));
            }
        }
        // process a list of sub-cards:
        if cards_to_process.len() > 0 {
            counter += self.count_cards(&cards_to_process);
        }

        counter
    }

    fn count_winning_cards(&self, card: &Card) -> u32 {
        let mut count = 0;
        for num in &card.own {
            if card.win.contains(num) {
                count += 1;
            }
        }
        count
    }
}

impl Day for Day04 {
    fn day_nr(&self) -> String {
        String::from("04")
    }
    fn title(&self) -> String {
        String::from("Day 4: Scratchcards")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day04.txt");
        // let input = read_lines("data/day04-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut sum = 0;
        for (nr, card) in self.cards.iter().enumerate() {
            let count = self.count_winning_cards(card);
            // store results for solution 2:
            self.winning_cards_memo.insert(nr as u32, count);
            let points = match count {
                0 => 0,
                _ => (2_i128).pow(count - 1),
            };
            sum += points;
        }
        String::from(format!("{0}", sum))
    }

    fn solve2(&mut self) -> String {
        let initial_cards: Vec<u32> = (0..((self.cards.len()) as u32)).collect();
        let count: u64 = self.count_cards(&initial_cards);
        String::from(format!("{0}", count))
    }
}
