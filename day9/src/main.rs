use std::{collections::VecDeque, fs};

fn main() {
    first_half("example1");
    first_half("example2");
    first_half("example3");
    first_half("input");
    second_half("example1");
    second_half("example2");
    second_half("example3");
    second_half("input");
}

fn parse(path: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn calculate_growth(row: &Vec<i32>) -> Vec<i32> {
    let mut growth = Vec::with_capacity(row.len() - 1);
    let mut previous = row[0];
    for i in 1..row.len() {
        let current = row[i];
        growth.push(current - previous);
        previous = current;
    }
    growth
}

fn first_half(path: &str) {
    let input = parse(path);
    let mut total_sum = 0;
    for row in input {
        let mut sequences = VecDeque::new();
        sequences.push_back(row);
        while !sequences.back().unwrap().iter().all(|i| *i == 0) {
            let new_sequence = calculate_growth(sequences.back().unwrap());
            sequences.push_back(new_sequence);
        }
        let mut next = 0;
        for sequence in sequences.into_iter().rev() {
            let last = sequence.last().unwrap();
            next = last + next;
        }
        total_sum += next;
    }
    println!("{path}: {total_sum}")
}

fn second_half(path: &str) {
    let input = parse(path);
    let mut total_sum = 0;
    for row in input {
        let mut sequences = VecDeque::new();
        sequences.push_back(row);
        while !sequences.back().unwrap().iter().all(|i| *i == 0) {
            let new_sequence = calculate_growth(sequences.back().unwrap());
            sequences.push_back(new_sequence);
        }
        let mut next = 0;
        for sequence in sequences.into_iter().rev() {
            let first = sequence.first().unwrap();
            next = first - next;
        }
        total_sum += next;
    }
    println!("{path}: {total_sum}")
}
