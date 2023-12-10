use clap;
use std::{
    collections::{HashSet, VecDeque},
    fs, str,
};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day10/input.txt"))]
    file: String,
    /// Find "gear ratios"
    #[clap(long, short, action)]
    part2: bool,
}

fn string_as_2d_array(s: String) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tile {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Tile {
    pub fn new(world_width: usize, world_height: usize, x: usize, y: usize) -> Self {
        Tile {
            x,
            y,
            w: world_width,
            h: world_height,
        }
    }

    pub fn from_world(world: &Vec<Vec<char>>, x: usize, y: usize) -> Self {
        Tile {
            x,
            y,
            w: world[0].len(),
            h: world.len(),
        }
    }

    pub fn char_at(&self, world: &Vec<Vec<char>>) -> Option<char> {
        if self.y >= world.len() || self.x >= world[self.y].len() {
            None
        } else {
            Some(world[self.y][self.x])
        }
    }

    pub fn digit_at(&self, world: &Vec<Vec<char>>) -> Option<u32> {
        if self.y >= world.len() || self.x >= world[self.y].len() {
            None
        } else {
            world[self.y][self.x].to_digit(10)
        }
    }

    pub fn topleft(&self) -> Option<Self> {
        if self.x > 0 && self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y - 1));
        }
        return None;
    }

    pub fn top(&self) -> Option<Self> {
        if self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x, self.y - 1));
        }
        return None;
    }

    pub fn topright(&self) -> Option<Self> {
        if self.x + 1 < self.w && self.y > 0 {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y - 1));
        }
        return None;
    }

    pub fn left(&self) -> Option<Self> {
        if self.x > 0 {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y));
        }
        return None;
    }

    pub fn right(&self) -> Option<Self> {
        if self.x + 1 < self.w {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y));
        }
        return None;
    }

    pub fn bottomleft(&self) -> Option<Self> {
        if self.x > 0 && self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x - 1, self.y + 1));
        }
        return None;
    }

    pub fn bottom(&self) -> Option<Self> {
        if self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x, self.y + 1));
        }
        return None;
    }

    pub fn bottomright(&self) -> Option<Self> {
        if self.x + 1 < self.w && self.y + 1 < self.h {
            return Some(Self::new(self.w, self.h, self.x + 1, self.y + 1));
        }
        return None;
    }

    pub fn adjacencies(&self) -> Vec<Self> {
        [self.top(), self.left(), self.right(), self.bottom()]
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn is_connected(&self, world: &Vec<Vec<char>>, other: &Self) -> bool {
        let c = other.char_at(world);
        if other.x == self.x && other.y == self.y + 1 {
            return c == Some('|') || c == Some('L') || c == Some('J') || c == Some('S');
        } else if other.x == self.x && self.y > 0 && other.y == self.y - 1 {
            return c == Some('|') || c == Some('7') || c == Some('F') || c == Some('S');
        } else if self.x > 0 && other.x == self.x - 1 && other.y == self.y {
            return c == Some('-') || c == Some('F') || c == Some('L') || c == Some('S');
        } else if other.x == self.x + 1 && other.y == self.y {
            return c == Some('-') || c == Some('J') || c == Some('7') || c == Some('S');
        }
        return false;
    }

    pub fn neighbors(&self, world: &Vec<Vec<char>>) -> Vec<Self> {
        match self.char_at(world) {
            Some('|') => [self.top(), self.bottom()]
                .into_iter()
                .flatten()
                .filter(|x| self.is_connected(world, x))
                .collect(),
            Some('-') => [self.left(), self.right()]
                .into_iter()
                .flatten()
                .filter(|x| self.is_connected(world, x))
                .collect(),
            Some('L') => [self.top(), self.right()]
                .into_iter()
                .flatten()
                .filter(|x| self.is_connected(world, x))
                .collect(),
            Some('J') => [self.top(), self.left()]
                .into_iter()
                .flatten()
                .filter(|x| self.is_connected(world, x))
                .collect(),
            Some('7') => [self.bottom(), self.left()]
                .into_iter()
                .flatten()
                .filter(|x| self.is_connected(world, x))
                .collect(),
            Some('F') => [self.bottom(), self.right()]
                .into_iter()
                .flatten()
                .filter(|x| self.is_connected(world, x))
                .collect(),
            Some('S') => self
                .adjacencies()
                .into_iter()
                .filter(|x| self.is_connected(world, x))
                .collect(),
            _ => vec![],
        }
    }
}

fn follow_pipe(world: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(Tile, u64)> {
    let mut stack: VecDeque<(Tile, u64)> = [(Tile::from_world(world, x, y), 0)].into();
    let mut visited: HashSet<Tile> = HashSet::new();
    let mut result: Vec<(Tile, u64)> = vec![];
    while let Some((tile, distance)) = stack.pop_front() {
        if visited.contains(&tile) {
            continue;
        }
        visited.insert(tile);
        result.push((tile.clone(), distance));
        for neighbor in tile.neighbors(world) {
            stack.push_back((neighbor, distance + 1));
        }
    }
    result
}

pub fn part1(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let world = string_as_2d_array(input);
    for (y, line) in world.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                let distances = follow_pipe(&world, x, y);
                let mut max_distance = 0;
                for (_tile, distance) in distances {
                    if distance > max_distance {
                        max_distance = distance;
                    }
                }
                return max_distance;
            }
        }
    }
    return 0;
}

fn outside(world: &mut Vec<Vec<char>>, walls: &HashSet<Tile>) -> u64 {
    let mut stack: VecDeque<(Tile, u64)> = [(Tile::from_world(world, 0, 0), 0)].into();
    let mut visited: HashSet<Tile> = HashSet::new();
    let mut count = 0;
    while let Some((tile, distance)) = stack.pop_front() {
        if visited.contains(&tile) {
            continue;
        }
        visited.insert(tile);
        world[tile.y][tile.x] = ' ';
        count += 1;
        for t in tile.adjacencies().iter() {
            if !walls.contains(t) {
                stack.push_back((t.clone(), distance + 1));
            }
        }
    }
    count
}

fn count_inside(world: &mut Vec<Vec<char>>, walls: &HashSet<Tile>) -> u64 {
    let mut count = 0;
    for (y, line) in world.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let tile = Tile::from_world(world, x, y);
            if walls.contains(&tile) {
                print!("\x1b[1;32m{}\x1b[0m", c);
                continue;
            }
            let mut t = tile;

            t = tile;
            let mut left = 0;
            while let Some(next) = t.left() {
                if walls.contains(&next) {
                    let nextc = next.char_at(world).unwrap();
                    if nextc != '-' && nextc != 'F' && nextc != '7' {
                        left += 1;
                    }
                }
                t = next;
            }
            if left == 0 {
                print!(" ");
                continue;
            }

            t = tile;
            let mut right = 0;
            while let Some(next) = t.right() {
                if walls.contains(&next) {
                    let nextc = next.char_at(world).unwrap();
                    if nextc != '-' && nextc != 'L' && nextc != 'J' {
                        right += 1;
                    }
                }
                t = next;
            }
            if right == 0 {
                print!(" ");
                continue;
            }

            t = tile;
            let mut bottom = 0;
            while let Some(next) = t.bottom() {
                if walls.contains(&next) {
                    let nextc = next.char_at(world).unwrap();
                    if nextc != '|' && nextc != 'J' && nextc != '7' {
                        bottom += 1;
                    }
                }
                t = next;
            }
            if bottom == 0 {
                print!(" ");
                continue;
            }

            t = tile;
            let mut top = 0;
            while let Some(next) = t.top() {
                if walls.contains(&next) {
                    let nextc = next.char_at(world).unwrap();
                    if nextc != '|' && nextc != 'L' && nextc != 'F' {
                        top += 1;
                    }
                }
                t = next;
            }
            if top == 0 {
                print!(" ");
                continue;
            }

            if (left % 2) + (right % 2) + (top % 2) + (bottom % 2) < 3 {
                print!(" ");
                continue;
            }

            print!("\x1b[1;31m#\x1b[0m");
            count += 1
        }
        print!("\n");
    }
    count
}

pub fn part2(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let mut world = string_as_2d_array(input);
    let mut expanded_world: Vec<Vec<char>> = vec![];
    let mut pipe_nodes: HashSet<Tile> = HashSet::new();
    let w = world[0].len();
    let h = world.len();
    expanded_world.push(vec![' '; w + 2]);
    for (y, line) in world.iter().enumerate() {
        let mut next_row = vec![' '; w + 2];
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                let distances = follow_pipe(&world, x, y);
                distances.iter().for_each(|x| {
                    let mut new_wall = x.0.clone();
                    new_wall.h += 2;
                    new_wall.w += 2;
                    new_wall.x += 1;
                    new_wall.y += 1;
                    pipe_nodes.insert(new_wall);
                });
            }
            next_row[x + 1] = *c;
        }
        expanded_world.push(next_row);
    }
    expanded_world.push(vec![' '; w + 2]);
    let wall_tiles = pipe_nodes.len();
    let inside_tiles = count_inside(&mut expanded_world, &pipe_nodes);
    return inside_tiles;
}

pub fn entrypoint(args: &Args) {
    if args.part2 {
        let res = part2(args);
        println!("{}", res);
    } else {
        let res = part1(args);
        println!("{}", res);
    }
}
