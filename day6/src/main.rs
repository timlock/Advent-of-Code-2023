use std::fs;

fn main() {
    first_half("example");
    first_half("input");
    second_half("example");
    second_half("input");
}

fn parse(path: &str) -> Vec<(u32, u32)> {
    let mut input = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|int_str| int_str.parse().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();
    let time = input.remove(0);
    let distance = input.remove(0);
    time.into_iter()
        .enumerate()
        .map(|(index, time)| (time, distance[index]))
        .collect()
}

fn first_half(path: &str) {
    let mut product = 0;
    for (time, distance) in parse(path) {
        let mut min = 1;
        while min * (time - min) <= distance {
            min += 1;
        }
        let mut max = time - 1;
        while max * (time - max) <= distance {
            max -= 1;
        }
        let valid = max - min + 1;
        if product == 0 {
            product = valid;
        } else {
            product *= valid;
        }
    }
    println!("{path}: {product}");
}

fn second_half(path: &str) {
    let (time, distance) = parse(path).into_iter().fold(
        (String::new(), String::new()),
        |mut acc, (time, distance)| {
            acc.0 += time.to_string().as_str();
            acc.1 += distance.to_string().as_str();
            acc
        },
    );
    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse().unwrap();
    let mut min = 1;
    while min * (time - min) <= distance {
        min += 1;
    }
    let mut max = time - 1;
    while max * (time - max) <= distance {
        max -= 1;
    }
    let valid = max - min + 1;
    println!("{path}: {valid}");
}
