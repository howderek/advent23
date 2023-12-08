use advent23::vendor::lcm;
use clap;
use std::{collections::HashMap, fs, str::FromStr};

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

struct Program {
    graph: HashMap<String, (String, String)>,
    stack: Vec<Instruction>,
}

impl Program {
    pub fn run(&self, start: String) -> u64 {
        let mut step_count: u64 = 0;
        let mut node = &start;
        loop {
            let mut s: Vec<Instruction> = self.stack.clone();
            while let Some(next) = s.pop() {
                if node.ends_with("Z") {
                    return step_count;
                }
                let (left, right) = self.graph.get(node).unwrap();
                match next {
                    Instruction::Left => node = left,
                    Instruction::Right => node = right,
                }
                step_count += 1;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseProgramError;

impl FromStr for Program {
    type Err = ParseProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("\n\n").collect();
        // Parse the instructions
        let stack: Vec<Instruction> = parts[0]
            .chars()
            .rev()
            .flat_map(|c| match c {
                'L' => Some(Instruction::Left),
                'R' => Some(Instruction::Right),
                _ => None,
            })
            .collect();
        // Parse the tree;
        let mut graph: HashMap<String, (String, String)> = HashMap::new();
        for line in parts[1].lines() {
            let id = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            graph.insert(id, (left, right));
        }
        Ok(Program { graph, stack })
    }
}

pub fn part1(input: String) -> u64 {
    input.parse::<Program>().unwrap().run("AAA".to_string())
}

pub fn part2(input: String) -> u64 {
    let program: Program = input.parse().unwrap();
    lcm(program
        .graph
        .iter()
        .filter_map(|(key, (_, _))| if key.ends_with("A") { Some(key) } else { None })
        .map(|start| program.run(start.clone()))
        .collect::<Vec<_>>()
        .as_slice())
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    if args.part2 {
        println!("{}", part2(input));
    } else {
        println!("{}", part1(input));
    }
}
