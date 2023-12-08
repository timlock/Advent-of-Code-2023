use std::{collections::HashMap, fs};

fn main() {
    first_half("example1");
    first_half("example2");
    first_half("input");
    second_half("example3");
    second_half("input");
}
enum Instruction {
    Left,
    Right,
}

fn parse_instructions(path: &str) -> Vec<Instruction> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == 'L' {
                        Instruction::Left
                    } else {
                        Instruction::Right
                    }
                })
                .collect::<Vec<_>>()
        })
        .next()
        .unwrap()
}

fn parse_nodes(path: &str) -> HashMap<String, (String, String)> {
    let nodes = fs::read_to_string(path)
        .unwrap()
        .lines()
        .skip(2)
        .map(|line| {
            line.split([' ', '(', ',', ')', '='])
                .filter(|node| !node.is_empty())
                .map(|node| node.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut node_map = HashMap::new();
    for mut node in nodes {
        let id = node.remove(0);
        let left = node.remove(0);
        let right = node.remove(0);
        node_map.insert(id, (left, right));
    }
    node_map
}

fn first_half(path: &str) {
    let instructions = parse_instructions(path);
    let node_map = parse_nodes(path);
    let mut current_node = "AAA";
    let mut instruction_index = 0;
    let mut counter = 0;
    while current_node != "ZZZ" {
        let (left, right) = node_map
            .get(current_node)
            .expect(format!("Map does not contain {current_node}").as_str());
        current_node = match instructions[instruction_index] {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        counter += 1;
        instruction_index = (instruction_index + 1) % instructions.len();
    }
    println!("{path}: {counter}");
}

fn second_half(path: &str) {
    let instructions = parse_instructions(path);
    let node_map = parse_nodes(path);
    let current_nodes = node_map
        .iter()
        .filter_map(|(key, _)| if key.ends_with('A') { Some(key) } else { None })
        .collect::<Vec<_>>();
    let mut minimum_required_steps: Vec<u64> = Vec::new();
    for node in current_nodes {
        let mut instruction_index = 0;
        let mut counter = 0;
        let mut current_node = node;
        while !current_node.ends_with('Z') {
            let (left, right) = node_map
                .get(current_node)
                .expect(format!("Map does not contain {node}").as_str());
            current_node = match instructions[instruction_index] {
                Instruction::Left => left,
                Instruction::Right => right,
            };
            counter += 1;
            instruction_index = (instruction_index + 1) % instructions.len();
        }
        minimum_required_steps.push(counter);
    }
    let mut lcm = minimum_required_steps[0];
    for steps in minimum_required_steps {
        lcm = least_common_multiple(steps, lcm);
    }
    println!("{path}: {:?}", lcm);
}

fn greatest_common_divisor(mut first: u64, mut second: u64) -> u64 {
    while second != 0 {
        let tmp = first;
        first = second;
        second = tmp % second;
    }
    first
}

fn least_common_multiple(first: u64, second: u64) -> u64 {
    first * second / greatest_common_divisor(first, second)
}
