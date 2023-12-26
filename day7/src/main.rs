use std::{collections::HashMap, fs};

fn main() {
    first_half("example");
    first_half("input");
    second_half("example");
    second_half("input");
}
#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let cards = &value.cards;
        let mut groups = HashMap::new();
        let mut hand_type = HandType::HighCard;
        for card in cards {
            match groups.get_mut(card) {
                Some(counter) => *counter += 1,
                None => {
                    groups.insert(card, 1);
                }
            };
        }
        for (_, value) in groups {
            match value {
                5 => return HandType::Five,
                4 => return HandType::Four,
                3 => match hand_type {
                    HandType::Pair => return HandType::FullHouse,
                    _ => {
                        if hand_type < HandType::Three {
                            hand_type = HandType::Three;
                        }
                    }
                },
                2 => match hand_type {
                    HandType::Three => return HandType::FullHouse,
                    HandType::Pair => return HandType::TwoPair,
                    _ => {
                        if hand_type == HandType::HighCard {
                            hand_type = HandType::Pair;
                        }
                    }
                },
                _ => {}
            }
        }
        hand_type
    }
}

impl From<&HandWithJoker> for HandType {
    fn from(value: &HandWithJoker) -> Self {
        let cards = &value.cards;
        let mut groups = HashMap::new();
        let mut hand_type = HandType::HighCard;
        let mut joker_counter = 0;
        for card in cards {
            if *card == 1 {
                joker_counter += 1;
            } else {
                match groups.get_mut(card) {
                    Some(counter) => *counter += 1,
                    None => {
                        groups.insert(card, 1);
                    }
                };
            }
        }
        let mut highest = (0, 0);
        for (card, amount) in groups.iter() {
            if *amount > highest.1 {
                highest = (**card, *amount);
            }
        }
        groups.insert(&highest.0, highest.1 + joker_counter);
        for (_, value) in groups {
            match value {
                5 => return HandType::Five,
                4 => return HandType::Four,
                3 => match hand_type {
                    HandType::Pair => return HandType::FullHouse,
                    _ => {
                        if hand_type < HandType::Three {
                            hand_type = HandType::Three;
                        }
                    }
                },
                2 => match hand_type {
                    HandType::Three => return HandType::FullHouse,
                    HandType::Pair => return HandType::TwoPair,
                    _ => {
                        if hand_type == HandType::HighCard {
                            hand_type = HandType::Pair;
                        }
                    }
                },
                _ => {}
            }
        }
        hand_type
    }
}

#[derive(Eq, Ord)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    hand_type: HandType,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut iter = value.split_whitespace();
        let cards = match iter.next() {
            Some(cards) => cards
                .chars()
                .map(|card| match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => match card.to_digit(10) {
                        Some(digit) => digit,
                        None => panic!("Could not parse {card}"),
                    },
                })
                .collect::<Vec<u32>>(),
            None => panic!("Could not parse {value}"),
        };
        let bid = match iter.next() {
            Some(bid) => match bid.parse() {
                Ok(bid) => bid,
                Err(_) => panic!("Could not parse {bid}"),
            },
            None => panic!("Could not parse {value}"),
        };
        let mut hand = Self {
            cards,
            bid,
            hand_type: HandType::HighCard,
        };
        hand.hand_type = HandType::from(&hand);
        hand
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(ordering) => match ordering {
                std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Equal => {
                    for i in 0..self.cards.len() {
                        match self.cards[i].cmp(&other.cards[i]) {
                            std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                            std::cmp::Ordering::Equal => {}
                            std::cmp::Ordering::Greater => {
                                return Some(std::cmp::Ordering::Greater)
                            }
                        };
                    }
                    Some(std::cmp::Ordering::Equal)
                }
                std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            },
            None => None,
        }
    }
}

fn first_half(path: &str) {
    let mut hands = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| Hand::from(line))
        .collect::<Vec<_>>();
    hands.sort();
    let result = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as u32) * hand.bid)
        .sum::<u32>();
    println!("{path}: {result}");
}
#[derive(Eq, Ord)]
struct HandWithJoker {
    cards: Vec<u32>,
    bid: u32,
    hand_type: HandType,
}


impl From<&str> for HandWithJoker {
    fn from(value: &str) -> Self {
        let mut iter = value.split_whitespace();
        let cards = match iter.next() {
            Some(cards) => cards
                .chars()
                .map(|card| match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    _ => match card.to_digit(10) {
                        Some(digit) => digit,
                        None => panic!("Could not parse {card}"),
                    },
                })
                .collect::<Vec<u32>>(),
            None => panic!("Could not parse {value}"),
        };
        let bid = match iter.next() {
            Some(bid) => match bid.parse() {
                Ok(bid) => bid,
                Err(_) => panic!("Could not parse {bid}"),
            },
            None => panic!("Could not parse {value}"),
        };
        let mut hand = Self {
            cards,
            bid,
            hand_type: HandType::HighCard,
        };
        hand.hand_type = HandType::from(&hand);
        hand
    }
}
impl PartialEq for HandWithJoker {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(ordering) => match ordering {
                std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Equal => {
                    for i in 0..self.cards.len() {
                        match self.cards[i].cmp(&other.cards[i]) {
                            std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                            std::cmp::Ordering::Equal => {}
                            std::cmp::Ordering::Greater => {
                                return Some(std::cmp::Ordering::Greater)
                            }
                        };
                    }
                    Some(std::cmp::Ordering::Equal)
                }
                std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            },
            None => None,
        }
    }
}

fn second_half(path: &str) {
    let mut hands = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| HandWithJoker::from(line))
        .map(|mut hand| {
            hand.hand_type = HandType::from(&hand);
            hand
        })
        .collect::<Vec<_>>();
    hands.sort();
    let result = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as u32) * hand.bid)
        .sum::<u32>();
    println!("{path}: {result}");
}
