use advent23::parse_number_list;
use clap;
use std::iter::zip;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day6/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

struct Race {
    time_limit: u64,
    record: u64,
}

impl Race {
    pub fn new(time_limit: u64, record: u64) -> Self {
        Self { time_limit, record }
    }

    pub fn ways_to_win(&self) -> u64 {
        let mut ways_to_win = 0;
        for t in 0..self.time_limit {
            let acceleration = 1;
            let velocity = acceleration * t;
            let distance = velocity * (self.time_limit - t);
            if distance > self.record {
                // println!("{} * {} = {} > {}", t, velocity, distance, self.record);
                ways_to_win += 1;
            }
        }
        ways_to_win
    }
}

pub fn part1(contents: &str) -> u64 {
    let mut line_iter = contents.lines();
    let times = parse_number_list(line_iter.next().unwrap().split(":").last().unwrap());
    let records = parse_number_list(line_iter.next().unwrap().split(":").last().unwrap());
    let mut races: Vec<Race> = vec![];
    for (time, record) in zip(times, records) {
        races.push(Race::new(time, record));
    }
    races.iter().map(|r| r.ways_to_win()).product()
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let out: u64 = part1(&input);
    println!("{}", out);
}
