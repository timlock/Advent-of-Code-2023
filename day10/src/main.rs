use std::{collections::HashMap, fs};

fn main() {
    first_half("example1");
    first_half("example2");
    first_half("input");
    second_half("example3");
    second_half("example4");
    second_half("example5");
    second_half("example6");
    second_half("input")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Address {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pipe {
    direction: (Direction, Direction),
}

impl Pipe {
    fn new(direction: (Direction, Direction)) -> Self {
        Pipe { direction }
    }

    fn is_connected(&self, direction: Direction) -> Option<Direction> {
        if self.direction.0.opposite() == direction {
            Some(self.direction.1)
        } else if self.direction.1.opposite() == direction {
            Some(self.direction.0)
        } else {
            None
        }
    }
    fn contains(&self, direction: Direction) -> bool {
        self.direction.0 == direction || self.direction.1 == direction
    }
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let pipe = match value {
            '|' => Pipe::new((Direction::North, Direction::South)),
            '-' => Pipe::new((Direction::West, Direction::East)),
            'L' => Pipe::new((Direction::North, Direction::East)),
            'J' => Pipe::new((Direction::North, Direction::West)),
            '7' => Pipe::new((Direction::South, Direction::West)),
            'F' => Pipe::new((Direction::South, Direction::East)),
            _ => return Err(()),
        };
        Ok(pipe)
    }
}

fn parse(path: &str) -> (Address, Vec<Vec<Option<Pipe>>>) {
    let mut start = None;
    let pipes = fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some(Address { x, y });
                    }
                    c.try_into().ok()
                })
                .collect()
        })
        .collect();
    match start {
        Some(start) => (start, pipes),
        None => panic!("No start"),
    }
}

fn next_pipe(
    address: Address,
    direction: Direction,
    pipes: &Vec<Vec<Option<Pipe>>>,
) -> Option<Address> {
    let x = match direction {
        Direction::East => {
            if address.x == pipes[0].len() - 1 {
                return None;
            } else {
                address.x + 1
            }
        }
        Direction::West => {
            if address.x == 0 {
                return None;
            } else {
                address.x - 1
            }
        }
        _ => address.x,
    };
    let y = match direction {
        Direction::North => {
            if address.y == 0 {
                return None;
            } else {
                address.y - 1
            }
        }

        Direction::South => {
            if address.y == pipes.len() - 1 {
                return None;
            } else {
                address.y + 1
            }
        }
        _ => address.y,
    };
    Some(Address { x, y })
}

fn find_loop(start: Address, pipes: &Vec<Vec<Option<Pipe>>>) -> Option<HashMap<Address, Pipe>> {
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    for direction in directions {
        let mut in_loop = HashMap::new();
        let mut current = match next_pipe(start, direction, pipes) {
            Some(address) => address,
            None => continue,
        };
        let mut current_direction = direction;
        while current.x != start.x || current.y != start.y {
            match pipes[current.y][current.x] {
                Some(pipe) => {
                    in_loop.insert(current, pipe);
                    current_direction = match pipe.is_connected(current_direction) {
                        Some(direction) => direction,
                        None => break,
                    };
                    match next_pipe(current, current_direction, pipes) {
                        Some(next) => {
                            current = next;
                        }
                        None => break,
                    };
                }
                None => break,
            };
        }
        if current == start {
            in_loop.insert(
                current,
                Pipe::new((direction, current_direction.opposite())),
            );
            return Some(in_loop);
        }
    }
    None
}
fn first_half(path: &str) {
    let (start, pipes) = parse(path);
    let pipe_loop = find_loop(start, &pipes).expect("Could not find loop");
    let mut furthest_away = pipe_loop.len() / 2;
    if pipe_loop.len() % 2 != 0 {
        furthest_away += 1;
    }
    println!("{path}: {furthest_away}");
}

fn second_half(path: &str) {
    let (start, mut pipes) = parse(path);
    let pipe_loop = find_loop(start, &pipes).expect("Could not find loop");
    pipes[start.y][start.x] = Some(pipe_loop[&start]);
    let mut counter = 0;
    let mut is_contained = false;
    for y in 0..pipes.len() {
        let mut last_direction = None;
        for x in 0..pipes[0].len() {
            let address = Address { x, y };
            match pipe_loop.get(&address) {
                Some(pipe) => {
                    if pipe.contains(Direction::North) && pipe.contains(Direction::South) {
                        is_contained = !is_contained;
                        last_direction = None;
                    } else if pipe.contains(Direction::North) || pipe.contains(Direction::South) {
                        match last_direction {
                            Some(dir) => {
                                if !pipe.contains(dir) {
                                    is_contained = !is_contained;
                                }
                                last_direction = None;
                            }
                            None => {
                                if pipe.contains(Direction::North) {
                                    last_direction = Some(Direction::North);
                                } else {
                                    last_direction = Some(Direction::South);
                                }
                            }
                        }
                    }
                }
                None => {
                    if is_contained {
                        counter += 1;
                    }
                    last_direction = None;
                }
            };
        }
    }
    println!("{path}: {}", counter);
}
