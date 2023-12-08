use std::{cmp::Ordering, collections::HashMap, rc::Rc};

use super::Day;
use adventofcode2023::read_lines;
use regex::Regex;

type TypeRank = u64;

#[derive(Debug)]
struct Card {
    card_type: TypeRank,
    type_str: String,
    hand: String,
    bid: u64,
    card_points: Rc<HashMap<char, u64>>,
}

impl Card {
    fn compare_hands(&self, a: &String, b: &String) -> Ordering {
        for i in 0..a.len() {
            let c_a = a.chars().nth(i).unwrap();
            let c_b = b.chars().nth(i).unwrap();
            let p_a = self.card_points.get(&c_a).unwrap();
            let p_b = self.card_points.get(&c_b).unwrap();
            if p_a < p_b {
                return Ordering::Less;
            }
            if p_a > p_b {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.card_type < other.card_type {
            return Some(std::cmp::Ordering::Less);
        }
        if self.card_type > other.card_type {
            return Some(std::cmp::Ordering::Greater);
        }
        return Some(self.compare_hands(&self.hand, &other.hand));
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(o) => o,
            _ => panic!("Failed to compare cards"),
        }
    }
}

#[derive(Debug)]
pub struct Day07 {
    input: Vec<String>,
    card_points: Rc<HashMap<char, u64>>,
    card_points2: Rc<HashMap<char, u64>>,
    cards: Vec<Card>,
}

impl Day07 {
    pub fn new() -> Day07 {
        Day07 {
            input: Vec::new(),
            card_points: Rc::new(HashMap::from([
                ('2', 2),
                ('3', 3),
                ('4', 4),
                ('5', 5),
                ('6', 6),
                ('7', 7),
                ('8', 8),
                ('9', 9),
                ('T', 10),
                ('J', 11),
                ('Q', 12),
                ('K', 13),
                ('A', 14),
            ])),
            card_points2: Rc::new(HashMap::from([
                ('J', 1),
                ('2', 2),
                ('3', 3),
                ('4', 4),
                ('5', 5),
                ('6', 6),
                ('7', 7),
                ('8', 8),
                ('9', 9),
                ('T', 10),
                ('Q', 12),
                ('K', 13),
                ('A', 14),
            ])),
            cards: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        let matcher = Regex::new(r"(\w+)\s+(\d+)").unwrap();
        for line in self.input.iter() {
            if let Some(group) = matcher.captures(line) {
                let hand = group.get(1).unwrap().as_str();
                let bid = group.get(2).unwrap().as_str().parse::<u64>().unwrap();
                self.cards.push(Card {
                    hand: String::from(hand),
                    bid,
                    card_type: 0,
                    type_str: "".to_string(),
                    card_points: self.card_points.clone(),
                });
            }
        }
        // println!("{:?}", self.cards);
    }
}

impl Day for Day07 {
    fn day_nr(&self) -> String {
        String::from("07")
    }
    fn title(&self) -> String {
        String::from("Day 7: Camel Cards")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day07.txt");
        // let input = read_lines("data/day07-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;

        for card in self.cards.iter_mut() {
            card.card_points = self.card_points.clone();
            let (card_type, type_str) = calc_type(card);
            card.card_type = card_type;
            card.type_str = type_str;
        }

        self.cards.sort();

        for (i, card) in self.cards.iter().enumerate() {
            // println!(
            //     "Cards: {0}, Type: {1} ({2})",
            //     card.hand, card.card_type, card.type_str
            // );
            solution += (i as u64 + 1) * card.bid;
        }

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;

        for card in self.cards.iter_mut() {
            card.card_points = self.card_points2.clone();
            let (card_type, type_str) = calc_type2(card);
            card.card_type = card_type;
            card.type_str = type_str;
        }

        self.cards.sort();

        for (i, card) in self.cards.iter().enumerate() {
            // println!(
            //     "Cards: {0}, Type: {1} ({2})",
            //     card.hand, card.card_type, card.type_str
            // );
            solution += (i as u64 + 1) * card.bid;
        }

        String::from(format!("{0}", solution))
    }
}

fn calc_type(card: &Card) -> (TypeRank, String) {
    let mut card_count: HashMap<char, u64> = HashMap::new();
    // count the amount of each card in the hand:
    for c in card.hand.chars() {
        if let Some(count) = card_count.get(&c) {
            card_count.insert(c, count + 1);
        } else {
            card_count.insert(c, 1);
        }
    }

    // a list of how many times a certain amount of cards appear:
    // e.g. for a Two Pair, we have 2 labels that appear 2 times, and one label that appear one time.
    let mut group_count: HashMap<u64, u64> = HashMap::new();
    for amount in card_count.values() {
        if let Some(count) = group_count.get(amount) {
            group_count.insert(*amount, count + 1);
        } else {
            group_count.insert(*amount, 1);
        }
    }

    // figure out the type of the card:

    // five of a kind:
    if let Some(count) = group_count.get(&5) {
        if *count == 1 {
            return (7, "Five of a kind".to_string());
        }
    }
    // four of a kind:
    if let Some(count) = group_count.get(&4) {
        if *count == 1 {
            return (6, "Four of a kind".to_string());
        }
    }
    // Full house: 3 same, and 2 same:
    if group_count.contains_key(&3) && group_count.contains_key(&2) {
        return (5, "Full House".to_string());
    }

    // Three of a kind: 3 same, and 2 different:
    if group_count.contains_key(&3) && group_count.contains_key(&1) {
        return (4, "Three of a kind".to_string());
    }

    // Two pair
    if let Some(count) = group_count.get(&2) {
        if *count == 2 {
            return (3, "Two pair".to_string());
        }
    }

    // One Pair
    if let Some(count_2) = group_count.get(&2) {
        if let Some(count_1) = group_count.get(&1) {
            if *count_2 == 1 && *count_1 == 3 {
                return (2, "One pair".to_string());
            }
        }
    }

    // High card
    if let Some(count) = group_count.get(&1) {
        if *count == 5 {
            return (1, "High card".to_string());
        }
    }

    panic!("Unknown card type");
}

fn calc_type2(card: &Card) -> (TypeRank, String) {
    let mut card_count: HashMap<char, u64> = HashMap::new();
    // count the amount of each card in the hand:
    // count the J's to the most present card:
    let mut count_j = 0;
    for c in card.hand.chars() {
        if c == 'J' {
            count_j += 1;
        } else {
            if let Some(count) = card_count.get(&c) {
                card_count.insert(c, count + 1);
            } else {
                card_count.insert(c, 1);
            }
        }
    }
    if count_j == 5 {
        card_count.insert('A', 5);
    } else {
        let (c, val) = card_count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
        // println!("Card: {0}, max char: {1}, max: {2}", card.hand, c, val);
        card_count.insert(*c, val + count_j);
    }

    // a list of how many times a certain amount of cards appear:
    // e.g. for a Two Pair, we have 2 labels that appear 2 times, and one label that appear one time.
    let mut group_count: HashMap<u64, u64> = HashMap::new();
    for (c, amount) in card_count.iter() {
        if *c == 'J' {
            count_j += amount;
        } else {
            if let Some(count) = group_count.get(amount) {
                group_count.insert(*amount, count + 1);
            } else {
                group_count.insert(*amount, 1);
            }
        }
    }
    // figure out the type of the card:
    // println!("Card {0}, group counts: {1:?}", card.hand, group_count);

    // five of a kind:
    if let Some(count) = group_count.get(&5) {
        if *count == 1 {
            return (7, "Five of a kind".to_string());
        }
    }
    // four of a kind:
    if let Some(count) = group_count.get(&4) {
        if *count == 1 {
            return (6, "Four of a kind".to_string());
        }
    }
    // Full house: 3 same, and 2 same:
    if group_count.contains_key(&3) && group_count.contains_key(&2) {
        return (5, "Full House".to_string());
    }

    // Three of a kind: 3 same, and 2 different:
    if group_count.contains_key(&3) && group_count.contains_key(&1) {
        return (4, "Three of a kind".to_string());
    }

    // Two pair
    if let Some(count) = group_count.get(&2) {
        if *count == 2 {
            return (3, "Two pair".to_string());
        }
    }

    // One Pair
    if let Some(count_2) = group_count.get(&2) {
        if let Some(count_1) = group_count.get(&1) {
            if *count_2 == 1 && *count_1 == 3 {
                return (2, "One pair".to_string());
            }
        }
    }

    // High card
    if let Some(count) = group_count.get(&1) {
        if *count == 5 {
            return (1, "High card".to_string());
        }
    }

    panic!("Unknown card type");
}
