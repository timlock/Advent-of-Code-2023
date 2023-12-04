use std::fs;

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    first_half("example");
    first_half("input");
    second_half("example2");
    second_half("input");
}

fn first_half(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let result = input
        .lines()
        .map(|line| {
            let mut first_digit = line
                .char_indices()
                .find(|(_, c)| c.to_digit(10).is_some())
                .unwrap()
                .1
                .to_string();
            let last_digit = line
                .char_indices()
                .rfind(|(_, c)| c.to_digit(10).is_some())
                .unwrap()
                .1;
            first_digit.push(last_digit);
            u32::from_str_radix(&first_digit, 10).unwrap()
        })
        .sum::<u32>();
    println!("{path}: {result}");
}

fn second_half(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let result = input
        .lines()
        .map(|line| {
            let mut first_digit = line.char_indices().find(|(_, c)| c.to_digit(10).is_some());
            let mut last_digit = line.char_indices().rfind(|(_, c)| c.to_digit(10).is_some());
            for i in 0..SPELLED_DIGITS.len() {
                let digit_spelled = SPELLED_DIGITS[i];
                let digit = char::from_digit((i + 1) as u32, 10).unwrap();
                if let Some(position) = line.find(digit_spelled) {
                    let new_digit = Some((position, digit));
                    match first_digit {
                        Some((p, _)) => {
                            if position < p {
                                first_digit = new_digit;
                            }
                        }
                        None => first_digit = new_digit,
                    }
                }
                if let Some(position) = line.rfind(digit_spelled) {
                    let new_digit = Some((position, digit));
                    match last_digit {
                        Some((p, _)) => {
                            if position > p {
                                last_digit = new_digit;
                            }
                        }
                        None => last_digit = new_digit,
                    }
                }
            }
            let mut value = first_digit.unwrap().1.to_string();
            value.push(last_digit.unwrap().1);
            u32::from_str_radix(&value, 10).unwrap()
        })
        .sum::<u32>();
    println!("{path}: {result}");
}
