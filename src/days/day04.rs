use std::collections::HashMap;

use super::Day;
use alex_lib::read_lines;
use regex::Regex;

#[derive(Debug)]
struct Card {
    win: Vec<i64>,
    own: Vec<i64>,
}

type CardNumber = u32;

#[derive(Debug)]
pub struct Day04 {
    input: Vec<String>,
    cards: Vec<Card>,
    // stores the number of winning cards for each card
    winning_cards_memo: HashMap<CardNumber, u32>,
    // stores the total count for each card, including gained sub-cards recursively
    card_counter_memo: HashMap<CardNumber, u64>,
}

impl Day04 {
    pub fn new() -> Day04 {
        Day04 {
            input: Vec::new(),
            cards: Vec::new(),
            winning_cards_memo: HashMap::new(),
            card_counter_memo: HashMap::new(),
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

    /// Clever memoization-based algorithm to count all winning cards
    /// including the newly gained sub-cards. Each card's total count
    /// is either calculated recursively, or, if already memoized,
    /// returned from the cache.
    /// This makes the calculation super-fast.
    fn count_card(&mut self, card_nr: CardNumber) -> u64 {
        let mut counter: u64 = 0;

        if let Some(counter_memo) = self.card_counter_memo.get(&card_nr) {
            return *counter_memo;
        }

        // 1st, count the one card:
        counter += 1;

        // process each sub-card based on the number of winning entries:
        let winning_cards = match self.winning_cards_memo.get(&card_nr) {
            Some(v) => *v,
            None => 0,
        };
        if winning_cards > 0 {
            for sub_card in (card_nr + 1)..=(card_nr + winning_cards) {
                counter += self.count_card(sub_card);
            }
        }
        self.card_counter_memo.insert(card_nr, counter);
        return counter;
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
        String::from("Scratchcards")
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
            self.winning_cards_memo.insert(nr as CardNumber, count);
            let points = match count {
                0 => 0,
                _ => (2_i128).pow(count - 1),
            };
            sum += points;
        }
        String::from(format!("{0}", sum))
    }

    fn solve2(&mut self) -> String {
        let initial_cards: Vec<CardNumber> = (0..((self.cards.len()) as CardNumber)).collect();
        let count: u64 = initial_cards.iter().map(|nr| self.count_card(*nr)).sum();
        String::from(format!("{0}", count))
    }
}
