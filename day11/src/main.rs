use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    first_half("example");
    first_half("input");
    second_half("example", 10);
    second_half("example", 100);
    second_half("input", 1000000);
}
#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}
impl Coordinate {
    fn distance(&self, rhs: &Self) -> i64 {
        let x = self.x.abs_diff(rhs.x) as i64;
        let y = self.y.abs_diff(rhs.y) as i64;
        x + y
    }
}
fn first_half(path: &str) {
    let input = parse(path);
    let (empty_x, empty_y) = find_empty_lines(&input);
    let mut current = Coordinate { x: 0, y: 0 };
    let mut universes = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == '#' {
                universes.push(current);
            } else if empty_x.contains(&x) {
                current.x += 1;
            }
            current.x += 1;
        }
        current.y += 1;
        if empty_y.contains(&y) {
            current.y += 1;
        }
        current.x = 0;
    }
    let result = shortest_path(&universes);
    println!("{path}: {result}");
}
fn second_half(path: &str, expansion: i64) {
    let input = parse(path);
    let (empty_x, empty_y) = find_empty_lines(&input);
    let mut current = Coordinate { x: 0, y: 0 };
    let mut universes = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == '#' {
                universes.push(current);
            } else if empty_x.contains(&x) {
                current.x += expansion -1;
            }
            current.x += 1;
        }
        current.y += 1;
        if empty_y.contains(&y) {
            current.y += expansion - 1;
        }
        current.x = 0;
    }
    let result = shortest_path(&universes);
    println!("{path}: {result}");
}

fn parse(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
fn find_empty_lines(input: &Vec<Vec<char>>) -> (HashSet<usize>, HashSet<usize>) {
    let mut filled_lines_y = HashSet::new();
    let mut filled_lines_x = HashSet::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let value = input[y][x];
            if value == '#' {
                filled_lines_x.insert(x);
                filled_lines_y.insert(y);
            }
        }
    }
    let mut empty_lines_x = HashSet::new();
    let mut empty_linex_y = HashSet::new();
    for y in 0..input.len() {
        if !filled_lines_y.contains(&y) {
            empty_linex_y.insert(y);
        }
        for x in 0..input[0].len() {
            if !filled_lines_x.contains(&x) {
                empty_lines_x.insert(x);
            }
        }
    }
    (empty_lines_x, empty_linex_y)
}
fn shortest_path(input: &Vec<Coordinate>) -> i64 {
    let mut results = HashMap::new();
    let mut i = 1;
    for current in input {
        let mut j = 1;
        for other in input {
            if !(current.x == other.x && current.y == other.y)
                && !(results.contains_key(&(i, j)) || results.contains_key(&(j, i)))
            {
                let distance = current.distance(other);
                results.insert((i, j), distance);
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    let mut keys = results.keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });
    // println!("{keys:?}");
    results.values().sum()
}
