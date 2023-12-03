use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day2/input.txt"))]
    file: String,

    /// What is the sum of the minimum number of each color multiplied together
    #[clap(long, short, action)]
    part2: bool,
}

pub struct Game {
    id: u64,
    red: Vec<u64>,
    blue: Vec<u64>,
    green: Vec<u64>,
}

impl Game {
    pub fn new(id: u64, red: Vec<u64>, blue: Vec<u64>, green: Vec<u64>) -> Self {
        Self {
            id,
            red,
            blue,
            green,
        }
    }

    pub fn from_string(input: String) -> Option<Self> {
        // Parses "Game 1: 1 blue, 1 red; 10 red; 8 red, 1 blue, 1 green; 1 green, 5 blue"
        let parts: Vec<&str> = input.split(": ").collect();
        let id: u64 = parts[0].trim().trim_start_matches("Game ").parse().ok()?;
        let rolls: Vec<&str> = parts[1].split(";").collect();
        let mut reds: Vec<u64> = vec![];
        let mut blues: Vec<u64> = vec![];
        let mut greens: Vec<u64> = vec![];
        for roll in rolls {
            let count_and_colors: Vec<&str> = roll.split(", ").collect();
            for count_and_color in count_and_colors {
                let count_and_color_parts: Vec<&str> = count_and_color.trim().split(" ").collect();
                let count: u64 = count_and_color_parts[0].trim().parse().ok()?;
                let color: &str = count_and_color_parts[1];
                match color {
                    "red" => reds.push(count),
                    "blue" => blues.push(count),
                    "green" => greens.push(count),
                    _ => (),
                }
            }
        }
        Some(Self::new(id, reds, blues, greens))
    }

    pub fn max_red(&self) -> u64 {
        self.red.iter().cloned().max().unwrap_or(0)
    }

    pub fn max_blue(&self) -> u64 {
        self.blue.iter().cloned().max().unwrap_or(0)
    }

    pub fn max_green(&self) -> u64 {
        self.green.iter().cloned().max().unwrap_or(0)
    }

    pub fn multiply_colors(&self) -> u64 {
        self.max_red() * self.max_green() * self.max_blue()
    }

    pub fn is_possible(&self, red: u64, green: u64, blue: u64) -> bool {
        red >= self.max_red() && green >= self.max_green() && blue >= self.max_blue()
    }
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut games: Vec<Game> = vec![];
    let mut total: u64 = 0;
    for line in input.lines() {
        match Game::from_string(line.to_string()) {
            Some(game) => games.push(game),
            None => println!("Could not parse game: {}", line),
        }
    }
    if args.part2 {
        for game in games {
            total += game.multiply_colors();
        }
        println!("{}", total)
    } else {
        print!("possible: ");
        for game in games {
            if game.is_possible(12, 13, 14) {
                print!("{} ", game.id);
                total += game.id;
            }
        }
        println!("\n   total: {}", total)
    }
}
