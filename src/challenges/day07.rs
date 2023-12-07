use clap;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day7/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::Jack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }
}

pub fn score_hand(hand: &Vec<Card>) -> u64 {
    let mut cardmap: HashMap<Card, u8> = HashMap::new();
    let mut score: u64 = 0;
    let mut factor: u64 = 1;
    for card in hand.iter().rev() {
        let count = cardmap.entry(*card).or_insert(0);
        *count += 1;
        score += *card as u64 * factor;
        factor *= 15;
    }
    const MAX_SINGLE: u64 = 15 * 15 * 15 * 15 * 15 * 15;
    for (card, count) in cardmap {
        match count {
            2 => score += MAX_SINGLE,
            3 => score += 3 * MAX_SINGLE,
            4 => score += 5 * MAX_SINGLE,
            5 => score += 6 * MAX_SINGLE,
            _ => score += card as u64,
        }
    }
    return score;
}

pub fn score_jokers(hand: &Vec<Card>) -> u64 {
    let mut cardmap: HashMap<Card, u8> = HashMap::new();
    let mut score: u64 = 0;
    let mut factor: u64 = 1;
    let mut joker_count: u8 = 0;
    let mut max_count_and_value: u64 = 0;
    let mut joker_card = Card::Jack;
    for card in hand.iter().rev() {
        let mut card_value = *card as u64;
        if *card == Card::Jack {
            joker_count += 1;
            card_value = 1;
        } else {
            let count = cardmap.entry(*card).or_insert(0);
            *count += 1;
        }
        score += card_value * factor;
        factor *= 15;
    }
    const MAX_SINGLE: u64 = 15 * 15 * 15 * 15 * 15 * 15;
    for (card, count) in cardmap.iter() {
        let count_and_value = (*count as u64 * 100) + *card as u64;
        if count_and_value > max_count_and_value {
            max_count_and_value = count_and_value;
            joker_card = *card;
        }
    }
    let count = cardmap.entry(joker_card).or_insert(0);
    *count += joker_count;
    for (card, count) in cardmap {
        match count {
            2 => score += MAX_SINGLE,
            3 => score += 3 * MAX_SINGLE,
            4 => score += 5 * MAX_SINGLE,
            5 => score += 6 * MAX_SINGLE,
            _ => score += card as u64,
        }
    }
    return score;
}

#[derive(Eq, Debug)]
pub struct Hand {
    cards: Vec<Card>,
    bet: u64,
    score: u64,
}

impl Hand {
    pub fn jacks(cards: Vec<Card>, bet: u64) -> Self {
        let score = score_hand(&cards);
        Self { cards, bet, score }
    }

    pub fn jokers(cards: Vec<Card>, bet: u64) -> Self {
        let score = score_jokers(&cards);
        Self { cards, bet, score }
    }

    pub fn from_string(input: String, jokers: bool) -> Option<Self> {
        let mut parts: Vec<&str> = input.split_whitespace().collect();
        let mut cards: Vec<Card> = vec![];
        for c in parts[0].chars() {
            let card = Card::from_char(c)?;
            cards.push(card);
        }
        if jokers {
            Some(Self::jokers(cards, parts[1].parse().unwrap()))
        } else {
            Some(Self::jacks(cards, parts[1].parse().unwrap()))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

pub fn part1(args: &Args) -> u64 {
    let contents = fs::read_to_string(&args.file).expect("I/O error");
    let mut max_score: u64 = 0;
    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|line| Hand::from_string(line.to_string(), false))
        .flatten()
        .collect();

    hands.sort_unstable();
    let result = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bet)
        .sum();

    for (i, h) in hands.iter().enumerate() {
        let rank = i as u64 + 1;
        println!("{} - {:?}", rank, h);
    }
    return result;
}

pub fn part2(args: &Args) -> u64 {
    let contents = fs::read_to_string(&args.file).expect("I/O error");
    let mut max_score: u64 = 0;
    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|line| Hand::from_string(line.to_string(), true))
        .flatten()
        .collect();

    hands.sort_unstable();
    let result = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bet)
        .sum();

    for (i, h) in hands.iter().enumerate() {
        let rank = i as u64 + 1;
        println!("{} - {:?}", rank, h);
    }
    return result;
}
pub fn entrypoint(args: &Args) {
    if !args.part2 {
        println!("{}", part1(args));
    } else {
        println!("{}", part2(args));
    }
}
