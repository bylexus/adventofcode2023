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
}

impl Day04 {
    pub fn new() -> Day04 {
        Day04 {
            input: Vec::new(),
            cards: Vec::new(),
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
            let winCards: Vec<i64> = win
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();
            let ownCards: Vec<i64> = own
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();

            self.cards.push(Card {
                win: winCards,
                own: ownCards,
            });
        }
    }

    fn count_cards(&self, card_nrs: &Vec<u32>) -> u128 {
        let mut counter: u128 = 0;
        // 1st, count all given cards:
        counter += card_nrs.len() as u128;

        // process each card:
        for card_nr in card_nrs.iter() {
            let card = &self.cards[*card_nr as usize];
            let winning_cards = self.count_winning_cards(card);
            if winning_cards > 0 {
                // process a list of sub-cards:
                let sub_cards:Vec<u32> = ((card_nr+1)..=(*card_nr  + winning_cards)).collect();
                counter += self.count_cards(&sub_cards);
            }
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

    fn solve1(&self) -> String {
        let mut sum: i128 = 0;
        for card in self.cards.iter() {
            let mut count = self.count_winning_cards(card);
            let points = match count {
                0 => 0,
                _ => (2_i128).pow(count - 1),
            };
            sum += points;
        }
        String::from(format!("{0}", sum))
    }

    fn solve2(&self) -> String {
        let initial_cards: Vec<u32> = (0..((self.cards.len()) as u32)).collect();
        let count: u128 = self.count_cards(&initial_cards);
        String::from(format!("{0}", count))
    }
}
