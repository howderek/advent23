use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day3/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

fn string_as_2d_array(s: String) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Debug, PartialEq)]
struct Tile {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Tile {
    fn new(world_width: usize, world_height: usize, x: usize, y: usize) -> Self {
        Tile {
            x,
            y,
            w: world_width,
            h: world_height,
        }
    }

    fn from_world(world: &Vec<Vec<char>>, x: usize, y: usize) -> Self {
        Tile {
            x,
            y,
            w: world[0].len(),
            h: world.len(),
        }
    }

    fn char_at(&self, world: &Vec<Vec<char>>) -> Option<char> {
        if self.y >= world.len() || self.x >= world[self.y].len() {
            None
        } else {
            Some(world[self.y][self.x])
        }
    }

    fn digit_at(&self, world: &Vec<Vec<char>>) -> Option<u32> {
        if self.y >= world.len() || self.x >= world[self.y].len() {
            None
        } else {
            world[self.y][self.x].to_digit(10)
        }
    }

    fn topleft(&self) -> Option<Self> {
        if self.x > 0 && self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y - 1));
        }
        return None;
    }

    fn top(&self) -> Option<Self> {
        if self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x, self.y - 1));
        }
        return None;
    }

    fn topright(&self) -> Option<Self> {
        if self.x + 1 < self.w && self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y - 1));
        }
        return None;
    }

    fn left(&self) -> Option<Self> {
        if self.x > 0 {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y));
        }
        return None;
    }

    fn right(&self) -> Option<Self> {
        if self.x + 1 < self.w {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y));
        }
        return None;
    }

    fn bottomleft(&self) -> Option<Self> {
        if self.x > 0 && self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y + 1));
        }
        return None;
    }

    fn bottom(&self) -> Option<Self> {
        if self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x, self.y + 1));
        }
        return None;
    }

    fn bottomright(&self) -> Option<Self> {
        if self.x + 1 < self.w && self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y + 1));
        }
        return None;
    }

    fn adjacencies(&self) -> Vec<Self> {
        [
            self.topleft(),
            self.top(),
            self.topright(),
            self.left(),
            self.right(),
            self.bottomleft(),
            self.bottom(),
            self.bottomright(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn is_special(c: char) -> bool {
    for special in ['!', '@', '#', '$', '%', '^', '&', '*', '+', '-', '/', '='] {
        if c == special {
            return true;
        }
    }
    return false;
}

fn is_special_adjacent(world: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    for adj in Tile::from_world(world, x, y).adjacencies() {
        match adj.char_at(world) {
            Some(c) => {
                if is_special(c) {
                    return true;
                }
            }
            None => (),
        }
    }
    return false;
}

fn part1(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let world = string_as_2d_array(input);
    let mut current_number = 0;
    let mut sum = 0;
    let mut is_valid = false;
    for (y, line) in world.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                current_number = (current_number * 10) + c.to_digit(10).unwrap();
                is_valid = is_valid || is_special_adjacent(&world, x, y);
            } else {
                if is_valid {
                    sum += current_number;
                    is_valid = false;
                }
                current_number = 0;
            }
        }
    }
    println!("{}", sum);
}

fn gather_number(world: &Vec<Vec<char>>, x: usize, y: usize) -> Option<(u32, Tile)> {
    let mut tile = Tile::from_world(world, x, y);
    if tile.digit_at(world).is_none() {
        return None;
    }
    while let Some(next_tile) = tile.left() {
        if next_tile.digit_at(world).is_some() {
            tile = next_tile;
        } else {
            break;
        }
    }
    let mut number = tile.digit_at(world).unwrap();
    while let Some(next_tile) = tile.right() {
        if let Some(n) = next_tile.digit_at(world) {
            number = (number * 10) + n;
            tile = next_tile;
        } else {
            break;
        }
    }
    return Some((number, tile));
}

fn gear_ratio(world: &Vec<Vec<char>>, x: usize, y: usize) -> Option<u32> {
    let tile = Tile::from_world(world, x, y);
    let mut numbers: Vec<u32> = vec![];
    let mut previous: Tile = Tile::new(0, 0, 0, 0);
    for adj in tile.adjacencies() {
        if let Some((number, tile)) = gather_number(world, adj.x, adj.y) {
            if tile != previous {
                numbers.push(number);
                previous = tile;
            }
        }
    }
    if numbers.len() == 2 {
        return Some(numbers[0] * numbers[1]);
    } else {
        return None;
    }
}

fn part2(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let world = string_as_2d_array(input);
    let mut sum = 0;
    for (y, line) in world.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '*' {
                match gear_ratio(&world, x, y) {
                    Some(n) => sum += n,
                    None => (),
                }
            }
        }
    }
    println!("{}", sum);
}

pub fn entrypoint(args: &Args) {
    if args.part2 {
        part2(args)
    } else {
        part1(args)
    }
}
