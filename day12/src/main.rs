use std::{collections::HashSet, fs, hash::Hash};

fn main() {
    println!("Hello, world!");
}

fn first_part(path: &str) {
    let rows = parse(path);
    let mut sum = 0;
    for (springs, group_sizes) in rows {
        let arrangements = find_arrangements(&springs, &group_sizes);
        sum += arrangements.into_iter().count();
    }

    println!("{path}: {sum}")
}

fn find_arrangements(springs: &Vec<Spring>, group_sizes: &Vec<usize>) -> HashSet<Vec<Spring>> {
    let arrengement = HashSet::new();
    let mut positions = Vec::new();
    for size in group_sizes {
        let start = match positions.last() {
            Some((prev_start, prev_size)) => find_valid_start(springs, *size, prev_start + prev_size),
            None => find_valid_start(springs, *size, 0),
        };
        match start {
            Some(start) => positions.push((start, start + size)),
            None => panic!("Could not find valid starting position, should not be reached"),
        }
    }

    loop {
        
    }
    // let mut in_group = false;
    // let mut groups: Vec<Vec<Spring>> = Vec::new();
    // for spring in springs {
    //     match spring {
    //         Spring::Operational => in_group = false,
    //         _ => {
    //             if in_group {
    //                 groups.last_mut().unwrap().push(spring.clone());
    //             } else {
    //                 groups.push(vec![spring.clone()]);
    //             }
    //             in_group = true;
    //         }
    //     };
    // }
    // let mut arrangements = HashSet::new();
    // for group_length in group_sizes {}
    todo!()
}

fn find_valid_start(springs: &Vec<Spring>, size: usize, start: usize) -> Option<usize> {
    if start + size > springs.len() {
        return None;
    }
    for i in start..springs.len() {
        let valid_start = match springs.get(i - 1) {
            Some(previous_) => !previous_.is_operational(),
            None => true,
        };
        if valid_start {
            let mut filled = 0;
            for j in i..(i + size) {
                if !springs[j].is_operational() {
                    filled += 1;
                }
            }
            if filled == size {
                return Some(i);
            }
        }
    }
    None
}

fn is_valid_arrangement(springs: &Vec<Spring>, group_sizes: &Vec<usize>) -> bool {
    let mut groups: Vec<Vec<Spring>> = Vec::new();
    let mut in_group = false;
    for spring in springs {
        match spring {
            Spring::Operational => in_group = false,
            Spring::Damaged => {
                if in_group {
                    groups.last_mut().unwrap().push(spring.clone());
                } else {
                    groups.push(vec![spring.clone()]);
                }
                in_group = true;
            }
            Spring::Unkown => return false,
        };
    }
    if groups.len() != group_sizes.len() {
        return false;
    }
    for i in 0..groups.len() {
        if groups[i].len() != group_sizes[i] {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unkown,
}
impl Spring {
    fn is_operational(&self) -> bool {
        match self {
            Spring::Operational => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Spring {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unkown),
            _ => Err(format!("Invalid symbol: {value}")),
        }
    }
}

fn parse(path: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let springs = iter
                .next()
                .unwrap()
                .chars()
                .map(|c| Spring::try_from(c).unwrap())
                .collect();
            let groups = iter
                .next()
                .unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as usize)
                .collect();
            (springs, groups)
        })
        .collect()
}
