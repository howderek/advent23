use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day9/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

pub fn entrypoint(args: &Args) {
    let _input = fs::read_to_string(&args.file).expect("I/O error");
    println!("this day has not yet been implemented")
}
