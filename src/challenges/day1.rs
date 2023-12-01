use clap;
use regex::Regex;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day1/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

fn parse_digit(input: &str) -> Option<u64> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => input.parse::<u64>().ok(),
    }
}

fn extract_digits_written(input: String) -> Option<u64> {
    let first_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let last_re = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|\d).*$").unwrap();

    let mut count: u64 = 0;

    for line in input.lines() {
        let first_cap = first_re.captures(line)?;
        let last_cap = last_re.captures(line)?;
        let first_digit = parse_digit(first_cap.get(1)?.as_str())?;
        let last_digit = parse_digit(last_cap.get(1)?.as_str())?;
        count += (first_digit * 10) + last_digit
    }

    Some(count)
}

fn extract_digits(input: String) -> Option<u64> {
    let re = Regex::new(r"\d").unwrap();
    let mut count: u64 = 0;

    for line in input.lines() {
        let c: Vec<_> = re.captures_iter(line).collect();
        let first_digit = c.get(0)?.get(0)?.as_str().parse::<u64>().ok()?;
        let last_digit = c.get(c.len() - 1)?.get(0)?.as_str().parse::<u64>().ok()?;
        count += (first_digit * 10) + last_digit
    }

    Some(count)
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut extract_fn = extract_digits as fn(String) -> Option<u64>;
    if args.part2 {
        extract_fn = extract_digits_written as fn(String) -> Option<u64>;
    }
    match extract_fn(input) {
        Some(count) => println!("{}", count),
        None => println!("No digits found"),
    }
}
