use clap::{self};
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day4/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

fn parse_list(s: &str) -> Vec<u64> {
    s.split(" ")
        .into_iter()
        .map(|x| x.parse())
        .flatten()
        .collect()
}

pub fn score_line(line: &str) -> u64 {
    let metadata_and_data: Vec<&str> = line.split(": ").collect();
    let data: Vec<&str> = metadata_and_data[1].split(" | ").collect();
    let winners: Vec<u64> = parse_list(data[0]);
    let assigned: Vec<u64> = parse_list(data[1]);
    let mut score: u64 = 0;
    // I know this looks dumb, it's because linear search is often faster whem there are very few elements
    for number in &assigned {
        for winner in &winners {
            if number == winner {
                score += 1;
            }
        }
    }
    return score;
}

pub fn part1(contents: &str) -> u64 {
    let mut sum: u64 = 0;
    for line in contents.lines() {
        let score = score_line(line);
        if score > 0 {
            sum += u64::pow(2, (score - 1).try_into().unwrap());
        }
    }
    return sum;
}

pub fn part2(contents: &str) -> u64 {
    let mut copies: Vec<u64> = vec![1; contents.lines().count()];
    for (i, line) in contents.lines().enumerate() {
        let matches = score_line(line);
        for j in 0..matches as usize {
            copies[i + j + 1] += copies[i];
        }
    }
    return copies.iter().sum();
}

pub fn entrypoint(args: &Args) {
    let contents = fs::read_to_string(&args.file).expect("I/O error");
    let out: u64;
    if args.part2 {
        out = part2(&contents);
    } else {
        out = part1(&contents);
    }
    println!("{}", out);
}
