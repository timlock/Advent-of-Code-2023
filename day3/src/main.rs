use std::{cmp::min, fs};

fn main() {
    first_half("example");
    first_half("input");
    second_half("example");
    second_half("input");
}

struct PartNumber {
    begin: usize,
    end: usize,
    value: u64,
}
impl PartNumber {
    fn from_group(group: Vec<(usize, char)>) -> Result<Self, ()> {
        let begin = match group.first() {
            Some((pos, _)) => *pos,
            None => return Err(()),
        };
        let end = match group.last() {
            Some((pos, _)) => *pos,
            None => return Err(()),
        };
        let value = group.into_iter().fold(String::new(), |mut acc, (_, c)| {
            acc.push(c);
            acc
        });
        let value = match u64::from_str_radix(&value, 10) {
            Ok(value) => value,
            Err(_) => return Err(()),
        };
        Ok(Self { begin, end, value })
    }

    fn is_in_range(&self, x_pos: usize) -> bool {
        let begin = self.begin.checked_sub(1).unwrap_or_default();
        x_pos >= begin && x_pos <= self.end + 1
    }
}

fn parse_symbols(path: &str) -> Vec<Vec<usize>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_digit(10) && *c != '.')
                .map(|(x, _)| x)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_gear_symbols(path: &str) -> Vec<Vec<usize>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(|(x, _)| x)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_numbers(path: &str) -> Vec<Vec<PartNumber>> {
    let input = fs::read_to_string(path).unwrap();
    let filtered_numbers = input
        .lines()
        .map(|line| line.chars().enumerate().filter(|(_, c)| c.is_digit(10)));
    let mut part_numbers = Vec::new();
    for line in filtered_numbers {
        let mut numbers_in_line = Vec::new();
        let mut current_number: Vec<(usize, char)> = Vec::new();
        for (x, c) in line {
            match current_number.last() {
                Some(group) => {
                    if group.0 + 1 == x {
                        current_number.push((x, c));
                    } else {
                        numbers_in_line.push(
                            PartNumber::from_group(current_number)
                                .expect("Could not parse part numer"),
                        );
                        current_number = vec![(x, c)];
                    }
                }
                None => current_number.push((x, c)),
            }
        }
        if !current_number.is_empty() {
            numbers_in_line
                .push(PartNumber::from_group(current_number).expect("Could not parse part numer"));
        }
        part_numbers.push(numbers_in_line);
    }
    part_numbers
}

fn first_half(path: &str) {
    let symbols = parse_symbols(path);
    let part_numbers = parse_numbers(path);
    let mut sum = 0;
    for y in 0..part_numbers.len() {
        for part_number in &part_numbers[y] {
            let mut value = 0;
            let y_begin = y.checked_sub(1).unwrap_or_default();
            let y_end = min(part_numbers.len() - 1, y + 1);
            for y in y_begin..=y_end {
                let symbol_range = &symbols[y];
                for symbol in symbol_range {
                    if part_number.is_in_range(*symbol) && value == 0 {
                        value = part_number.value
                    }
                }
            }
            sum += value;
        }
    }
    println!("{path}: {sum}");
}

fn second_half(path: &str) {
    let gears = parse_gear_symbols(path);
    let part_numbers = parse_numbers(path);
    let mut sum = 0;
    for y in 0..gears.len() {
        for gear in &gears[y] {
            let y_begin = y.checked_sub(1).unwrap_or_default();
            let y_end = min(gears.len() - 1, y + 1);
            let mut adjacent = Vec::new();
            for y in y_begin..=y_end {
                for part_number in &part_numbers[y] {
                    if part_number.is_in_range(*gear) {
                        adjacent.push(part_number);
                    }
                }
            }
            if adjacent.len() == 2 {
                sum += adjacent[0].value * adjacent[1].value;
            }
        }
    }
    println!("{path}: {sum}");
}
