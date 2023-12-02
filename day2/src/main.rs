use std::fs;

fn main() {
    first_half("example");
    first_half("input");
    second_half("example");
    second_half("input");
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut args = value.split_ascii_whitespace().collect::<Vec<_>>();
        if args.len() != 2 {
            return Err(format!("Color has invalid amount of arguments: {value}"));
        }
        let amount = args
            .remove(0)
            .parse()
            .map_err(|_| "Could not parse amount: {value}")?;
        let color = args.remove(0);
        match color {
            "red" => Ok(Color::Red(amount)),
            "green" => Ok(Color::Green(amount)),
            "blue" => Ok(Color::Blue(amount)),
            _ => Err(format!("No valid color: {value}")),
        }
    }
}
struct Game {
    hands: Vec<Vec<Color>>,
}

impl Game {
    fn new() -> Self {
        Self { hands: Vec::new() }
    }

    fn add(&mut self, hand: Vec<Color>) {
        self.hands.push(hand);
    }

    fn could_contain_color(&self, color: Color) -> bool {
        for hand in self.hands.iter() {
            for c in hand {
                match color {
                    Color::Red(amount) => match c {
                        Color::Red(a) => {
                            if *a > amount {
                                return false;
                            }
                        }
                        _ => (),
                    },
                    Color::Green(amount) => match c {
                        Color::Green(a) => {
                            if *a > amount {
                                return false;
                            }
                        }
                        _ => (),
                    },
                    Color::Blue(amount) => match c {
                        Color::Blue(a) => {
                            if *a > amount {
                                return false;
                            }
                        }
                        _ => (),
                    },
                }
            }
        }
        true
    }

    fn minimal_amount(&self) -> (u32, u32, u32) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for hand in &self.hands {
            for color in hand {
                match color {
                    Color::Red(amount) => {
                        if *amount > red {
                            red = *amount;
                        }
                    }
                    Color::Green(amount) => {
                        if *amount > green {
                            green = *amount;
                        }
                    }
                    Color::Blue(amount) => {
                        if *amount > blue {
                            blue = *amount;
                        }
                    }
                }
            }
        }

        (red, green, blue)
    }
}

impl TryFrom<&str> for Game {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let colon_pos = value.find(':').unwrap() + 1;
        let value = &value[colon_pos..];
        let mut game = Game::new();
        for hand_str in value.split(';') {
            let mut hand = Vec::new();
            for color_str in hand_str.split(',') {
                match Color::try_from(color_str) {
                    Ok(color) => hand.push(color),
                    Err(err) => return Err(err),
                };
            }
            game.add(hand);
        }
        Ok(game)
    }
}

fn parse(path: &str) -> Vec<Game> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| match Game::try_from(line) {
            Ok(game) => game,
            Err(err) => panic!("{err}"),
        })
        .collect()
}

fn first_half(path: &str) {
    let result = parse(path)
        .into_iter()
        .enumerate()
        .filter_map(|(index, game)| {
            if game.could_contain_color(Color::Red(12))
                && game.could_contain_color(Color::Green(13))
                && game.could_contain_color(Color::Blue(14))
            {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("{path}: {result}")
}

fn second_half(path: &str) {
    let result = parse(path).into_iter().map(|game| {
        let minimal_amount = game.minimal_amount();
        minimal_amount.0 * minimal_amount.1 * minimal_amount.2
    }).sum::<u32>();
    println!("{path}: {result}");
}
