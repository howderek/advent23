use advent23::parse_number_list;
use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day9/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

pub fn derivative(v: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 0..v.len() - 1 {
        result.push(v[i + 1] - v[i]);
    }
    result
}

pub fn autoderive(v: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut result = vec![v.clone()];
    let mut next = v.clone();
    while !next.iter().all(|x| *x == 0) {
        next = derivative(&next);
        result.push(next.clone());
    }
    result
}

pub fn extrapolate(v: &mut Vec<Vec<i64>>) -> &mut Vec<Vec<i64>> {
    {
        let len = v.len();
        let base: &mut Vec<i64> = &mut v[len - 1];
        base.push(0);
    }
    for i in 1..v.len() {
        let level_idx = v.len() - i - 1;
        {
            let last_lower_level = v[level_idx + 1][v[level_idx + 1].len() - 1];
            let level: &mut Vec<i64> = &mut v[level_idx];
            level.push(level[level.len() - 1] + last_lower_level);
        }
    }
    v
}

pub fn extrapolate_left(v: &mut Vec<Vec<i64>>) -> &mut Vec<Vec<i64>> {
    {
        let len = v.len();
        let base: &mut Vec<i64> = &mut v[len - 1];
        base.push(0);
    }
    for i in 1..v.len() {
        let level_idx = v.len() - i - 1;
        {
            let first_lower_level = v[level_idx + 1][0];
            let level: &mut Vec<i64> = &mut v[level_idx];
            level.insert(0, level[0] - first_lower_level);
        }
    }
    v
}

pub fn part1(input: String) -> i64 {
    let mut result = 0;
    let sequences: Vec<Vec<i64>> = input.lines().map(|l| parse_number_list(l)).collect();
    for sequence in sequences.iter() {
        let mut layers = autoderive(&sequence);
        extrapolate(&mut layers);
        result += layers[0][layers[0].len() - 1];
    }
    result
}

pub fn part2(input: String) -> i64 {
    let mut result = 0;
    let sequences: Vec<Vec<i64>> = input.lines().map(|l| parse_number_list(l)).collect();
    for sequence in sequences.iter() {
        let mut layers = autoderive(&sequence);
        extrapolate_left(&mut layers);
        result += layers[0][0];
    }
    result
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    if args.part2 {
        println!("{}", part2(input));
    } else {
        println!("{}", part1(input));
    }
}
