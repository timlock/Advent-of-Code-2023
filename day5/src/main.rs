use std::{cmp, fs, ops::Range};

fn main() {
    first_half("example");
    first_half("input");
    second_half("example");
    second_half("input");
}

fn parse_seeds(path: &str) -> Vec<i64> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|int_str| int_str.parse().unwrap())
        .collect()
}
#[derive(Debug)]
struct Converter {
    destination: i64,
    source: i64,
    range: i64,
}

impl Converter {
    fn convert(&self, value: i64) -> Option<i64> {
        let end = self.source + self.range;
        if value >= self.source && value <= end {
            let new_value = value - self.source + self.destination;
            Some(new_value)
        } else {
            None
        }
    }

    fn convert_range(&self, value: &Range<i64>) -> Option<(Range<i64>, Vec<Range<i64>>)> {
        let start = cmp::max(value.start, self.source);
        let end = cmp::min(value.end, self.source + self.range);
        if start <= end {
            let mut not_converted = Vec::new();
            if value.start < start {
                not_converted.push(value.start..start);
            }
            if value.end > end {
                not_converted.push(end..value.end);
            }
            let difference = start - self.source;
            let destination_start = self.destination + difference;
            let destination_end = destination_start + end - start;
            let converted = destination_start..destination_end;
            Some((converted, not_converted))
        } else {
            None
        }
    }
}

fn parse_converters(path: &str) -> Vec<Vec<Converter>> {
    fs::read_to_string(path)
        .unwrap()
        .split("map:")
        .skip(1)
        .map(|chunk| {
            chunk
                .lines()
                .filter_map(|line| {
                    let values = line
                        .split_ascii_whitespace()
                        .filter_map(|int_str| int_str.parse::<i64>().ok())
                        .collect::<Vec<_>>();
                    if values.len() == 3 {
                        Some(Converter {
                            destination: values[0],
                            source: values[1],
                            range: values[2],
                        })
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

fn first_half(path: &str) {
    let seeds = parse_seeds(path);
    let converters = parse_converters(path);
    let mut location = i64::MAX;
    for seed in seeds {
        let mut value = seed;
        for converter_layer in converters.iter() {
            for converter in converter_layer {
                if let Some(new_value) = converter.convert(value) {
                    value = new_value;
                    break;
                }
            }
        }
        if value < location {
            location = value
        }
    }
    println!("{path}: {location}")
}

fn second_half(path: &str) {
    let mut seeds = parse_seeds(path)
        .chunks(2)
        .map(|slice| slice[0]..(slice[0] + slice[1]))
        .collect::<Vec<_>>();
    let converters = parse_converters(path);
    for converter_layer in converters {
        let mut converted_ranges = Vec::new();
        for converter in converter_layer {
            seeds = seeds
                .into_iter()
                .flat_map(|seed_range| match converter.convert_range(&seed_range) {
                    Some((converted, not_converted)) => {
                        converted_ranges.push(converted);
                        not_converted
                    }
                    None => vec![seed_range],
                })
                .collect();
        }
        seeds.extend(converted_ranges);
    }
    seeds.sort_by(|a, b| a.start.cmp(&b.start));
    let location = seeds[0].start;
    println!("{path}: {location}")
}
