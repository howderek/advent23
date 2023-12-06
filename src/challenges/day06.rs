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

    pub fn ways_to_win_optimized(&self) -> u64 {
        let r = self.record as f64;
        let l = self.time_limit as f64;
        let record_velocity = f64::floor((l - f64::sqrt((l * l) - (4.0 * r))) / 2.0);
        let ways_to_play = self.time_limit - 1;
        let ways_to_lose = (record_velocity) * 2.0;
        let ways_to_win = ways_to_play - ways_to_lose as u64;
        return ways_to_win;
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
    races.iter().map(|r| r.ways_to_win_optimized()).product()
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let out: u64 = part1(&input);
    println!("{}", out);
}
