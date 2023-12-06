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

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    println!(
        "{}",
        zip(
            parse_number_list::<f64>(input.lines().nth(0).unwrap().split(":").last().unwrap()),
            parse_number_list::<f64>(input.lines().nth(1).unwrap().split(":").last().unwrap()),
        )
        .map(|(l, r)| l - f64::floor((l - f64::sqrt((l * l) - (4.0 * r))) / 2.0) * 2.0 - 1.0)
        .product::<f64>()
    );
}
