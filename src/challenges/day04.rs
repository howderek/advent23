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

pub fn score_line(line: &str) -> u32 {
    let metadata_and_data: Vec<&str> = line.split(": ").collect();
    let data: Vec<&str> = metadata_and_data[1].split(" | ").collect();
    let winners: Vec<u64> = parse_list(data[0]);
    let assigned: Vec<u64> = parse_list(data[1]);
    let mut score: u32 = 0;
    for number in &assigned {
        for winner in &winners {
            if number == winner {
                score += 1;
            }
        }
    }
    return score;
}

pub fn part1(args: &Args) -> u32 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut score = score_line(line);
        if score > 0 {
            score = u32::pow(2, score - 1);
        }
        sum += score;
    }
    return sum;
}

pub fn part2(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut copies: Vec<u64> = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let matches = score_line(line);
        for j in 0..matches as usize {
            copies[i + j + 1] += copies[i];
        }
    }
    return copies.iter().sum();
}

pub fn entrypoint(args: &Args) {
    if args.part2 {
        let out = part2(args);
        println!("{}", out);
    } else {
        let out = part1(args);
        println!("{}", out);
    }
}
