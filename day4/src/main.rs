use std::fs;

fn main() {
    first_half("example");
    first_half("input");
    second_half("example");
    second_half("input");
}

struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
    matching_numbers: Vec<u32>,
    score: usize,
}

fn parse(path: &str) -> Vec<Card> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut iter = line.split([':', '|']).skip(1);
            let winning_numbers = iter
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|int_str| int_str.parse().expect("Could not parse int"))
                .collect::<Vec<_>>();
            let scratched_numbers = iter
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|int_str| int_str.parse().expect("Could not parse int"))
                .collect::<Vec<_>>();
            let mut matching_numbers = Vec::new();
            let mut score = 0;
            for scratched in scratched_numbers.iter() {
                if winning_numbers.contains(scratched) {
                    matching_numbers.push(*scratched);
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                }
            }
            Card {
                id: id + 1,
                winning_numbers,
                scratched_numbers,
                matching_numbers,
                score,
            }
        })
        .collect()
}

fn first_half(path: &str) {
    let cards = parse(path);
    let score = cards.into_iter().map(|card| card.score).sum::<usize>();
    println!("{path}: {score}")
}

fn second_half(path: &str) {
    let mut cards = parse(path)
        .into_iter()
        .map(|card| (card, 1))
        .collect::<Vec<_>>();
    for i in 0..cards.len() {
        let matching_numbers = cards[i].0.matching_numbers.len();
        if matching_numbers == 0 {
            continue;
        }
        let copies = cards[i].1;
        let begin = std::cmp::min(i + 1, cards.len() - 1);
        let end = std::cmp::min(i + matching_numbers + 1, cards.len());
        for i in begin..end {
            cards[i].1 += copies;
        }
    }
    let total_cards = cards.iter().map(|(_, count)| *count).sum::<u32>();
    println!("{path}: {total_cards}");
}
