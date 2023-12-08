use advent23::vendor::lcm;
use clap;
use std::{collections::HashMap, fs};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day8/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Instruction {
    Left,
    Right,
}

pub fn parse_tree(s: &str) -> HashMap<&str, (&str, &str)> {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in s.lines() {
        let id = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(id, (left, right));
    }
    map
}

pub fn parse_traversal_stack(s: &str) -> Vec<Instruction> {
    s.chars()
        .rev()
        .flat_map(|c| match c {
            'L' => Some(Instruction::Left),
            'R' => Some(Instruction::Right),
            _ => None,
        })
        .collect()
}

pub fn follow_path(
    starts: Vec<&str>,
    tree: &HashMap<&str, (&str, &str)>,
    stack: &Vec<Instruction>,
) -> u64 {
    let mut step_count: u64 = 0;
    let mut nodes: Vec<&str> = starts.clone();
    loop {
        let mut s: Vec<Instruction> = stack.clone();
        while let Some(next) = s.pop() {
            if nodes.iter().all(|node| node.ends_with("Z")) {
                return step_count;
            }
            nodes.iter_mut().for_each(|node| {
                let (left, right) = tree.get(node).unwrap();
                match next {
                    Instruction::Left => {
                        *node = left;
                    }
                    Instruction::Right => {
                        *node = right;
                    }
                }
            });
            step_count += 1;
        }
    }
}

pub fn part1(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let parts: Vec<&str> = input.split("\n\n").collect();
    let orig_stack = parse_traversal_stack(parts[0]);
    let tree = parse_tree(parts[1]);
    follow_path(vec!["AAA"], &tree, &orig_stack)
}

pub fn part2(args: &Args) -> u64 {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let parts: Vec<&str> = input.split("\n\n").collect();
    let orig_stack = parse_traversal_stack(parts[0]);
    let tree = parse_tree(parts[1]);
    let starts: Vec<&str> = tree
        .iter()
        .filter(|(key, (_, _))| key.ends_with("A"))
        .map(|(id, _)| *id)
        .collect();
    let factors: Vec<u64> = starts
        .iter()
        .map(|k| follow_path(vec![k], &tree, &orig_stack))
        .collect();
    lcm(factors.as_slice())
}

pub fn entrypoint(args: &Args) {
    if args.part2 {
        println!("{}", part2(args));
    } else {
        println!("{}", part1(args));
    }
}
