use advent23::parse_number_list;
use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day5/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,

    #[clap(long, short, action)]
    describe: bool,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Correspondence {
    source_start: u64,
    destination_start: u64,
    range_length: u64,
}

impl Correspondence {
    pub fn new(source_start: u64, destination_start: u64, range_length: u64) -> Self {
        Self {
            source_start,
            destination_start,
            range_length,
        }
    }

    pub fn from_line(line: &str) -> Self {
        let nums = parse_number_list(line);
        Self::new(nums[1], nums[0], nums[2])
    }

    pub fn lookup(&self, idx: u64) -> Option<u64> {
        if idx >= self.source_start && idx <= (self.source_start + self.range_length) {
            Some(self.destination_start + (idx - self.source_start))
        } else {
            None
        }
    }

    pub fn overlaps(&self, ranges: &Vec<(u64, u64)>) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
        let mut leftovers = vec![];
        let mut overlaps = vec![];

        // println!(
        //     "{:?} into ({}-{} -> {}-{}) (delta {})",
        //     ranges,
        //     self.source_start,
        //     self.source_start + self.range_length,
        //     self.destination_start,
        //     self.destination_start + self.range_length,
        //     self.destination_start as i64 - self.source_start as i64,
        // );

        for range_tuple in ranges.iter() {
            let (start, end) = *range_tuple;
            if start >= self.source_start && start <= (self.source_start + self.range_length) {
                if end >= self.source_start && end <= self.source_start + self.range_length {
                    overlaps.push((
                        self.destination_start + (start - self.source_start),
                        self.destination_start + (end - self.source_start),
                    ));
                } else {
                    overlaps.push((
                        self.destination_start + (start - self.source_start),
                        self.destination_start + self.range_length,
                    ));
                    leftovers.push((self.source_start + self.range_length + 1, end));
                }
            } else if end >= self.source_start && end <= (self.source_start + self.range_length) {
                leftovers.push((start, self.source_start - 1));
                overlaps.push((
                    self.destination_start,
                    self.destination_start + end - self.source_start,
                ));
            } else if start < self.source_start && end > (self.source_start + self.range_length) {
                leftovers.push((start, self.source_start - 1));
                overlaps.push((
                    self.destination_start,
                    self.destination_start + self.range_length,
                ));
                leftovers.push((self.source_start + self.range_length + 1, end));
            } else {
                leftovers.push((start, end));
            }
        }

        // println!(
        //     "    leftovers: {:?}\n     overlaps: {:?}\n",
        //     leftovers, overlaps
        // );
        return (leftovers, overlaps);
    }
}

#[derive(Debug)]
pub struct ResourceMap {
    from: String,
    to: String,
    correspondences: Vec<Correspondence>,
}

impl ResourceMap {
    pub fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            correspondences: vec![],
        }
    }

    pub fn from_line(line: &str) -> Self {
        let name = line.split(" ").next().unwrap();
        let resources: Vec<&str> = name.split("-to-").collect();
        return Self::new(resources[0].to_string(), resources[1].to_string());
    }

    pub fn add_correspondence_from_line(&mut self, line: &str) {
        self.correspondences.push(Correspondence::from_line(line))
    }

    pub fn lookup(&self, idx: u64) -> u64 {
        let found = self
            .correspondences
            .iter()
            .map(|c| c.lookup(idx))
            .flatten()
            .next();
        match found {
            Some(v) => v,
            None => idx,
        }
    }

    pub fn overlaps(&self, start: u64, end: u64) -> Vec<(u64, u64)> {
        let mut solved = vec![];
        let mut unresolved = vec![(start, end)];
        // println!("\n\n");
        // self.describe();
        // println!();
        for correspondence in self.correspondences.iter() {
            let (leftovers, mut overlaps) = correspondence.overlaps(&unresolved);
            unresolved = leftovers;
            solved.append(&mut overlaps);
        }
        solved.append(&mut unresolved);
        // println!("solved: {:?}", solved);
        return solved;
    }

    pub fn describe(&self) {
        println!("{} to {}:", self.from, self.to);
        for correspondence in &self.correspondences {
            println!(
                "    {}-{} -> {}-{}",
                correspondence.source_start,
                correspondence.source_start + correspondence.range_length,
                correspondence.destination_start,
                correspondence.destination_start + correspondence.range_length
            );
        }
    }
}

#[derive(Debug)]
pub struct SeedData {
    seeds: Vec<u64>,
    resource_maps: Vec<ResourceMap>,
}

impl SeedData {
    pub fn from_string(input: &str) -> Option<Self> {
        let mut seeds: Vec<u64> = vec![];
        let mut resource_maps: Vec<ResourceMap> = vec![];
        let mut current_resource: ResourceMap =
            ResourceMap::new(String::from("seed"), String::from("seed"));
        for (i, line) in input.lines().enumerate() {
            if i == 0 {
                seeds = parse_number_list(line.trim_start_matches("seeds: "));
            } else {
                match line.chars().next() {
                    Some(c) => match c {
                        'a'..='z' => {
                            resource_maps.push(current_resource);
                            current_resource = ResourceMap::from_line(line)
                        }
                        '0'..='9' => current_resource.add_correspondence_from_line(line),
                        _ => (),
                    },
                    None => (),
                }
            }
        }
        resource_maps.push(current_resource);
        Some(Self {
            seeds,
            resource_maps,
        })
    }

    pub fn smallest_location_for_seed(&self, idx: u64) -> u64 {
        let mut current_idx = idx;
        for resource_map in &self.resource_maps {
            let next_idx = resource_map.lookup(current_idx);
            current_idx = next_idx;
        }
        current_idx
    }

    pub fn smallest_location_for_range(&self, start: u64, end: u64) -> u64 {
        let mut current_stack: Vec<(u64, u64)> = vec![(start, end)];
        for resource_map in &self.resource_maps {
            let mut next_stack: Vec<(u64, u64)> = vec![];
            while let Some((start, end)) = current_stack.pop() {
                next_stack.append(&mut resource_map.overlaps(start, end));
            }
            // println!("next_stack: {:?}", next_stack);
            current_stack = next_stack;
        }
        // println!("");
        return current_stack.iter().min().unwrap().0;
    }

    pub fn smallest_location_for_all_seeds(&self) -> u64 {
        self.seeds
            .iter()
            .map(|x| self.smallest_location_for_seed(*x))
            .min()
            .unwrap()
    }

    pub fn smallest_location_for_all_ranges(&self) -> u64 {
        let mut i = 0;
        let mut min: u64 = u64::MAX;
        while i < self.seeds.len() {
            let start = self.seeds[i];
            let end = start + self.seeds[i + 1];
            let lookup = self.smallest_location_for_range(start, end);
            if lookup < min {
                min = lookup;
            }
            i += 2;
        }
        min
    }
}

pub fn entrypoint(args: &Args) {
    let input = fs::read_to_string(&args.file).expect("I/O error");
    let seed_data = SeedData::from_string(&input).unwrap();
    if args.describe {
        for resource_map in &seed_data.resource_maps {
            resource_map.describe();
            println!()
        }
    } else {
        if !args.part2 {
            println!("{}", seed_data.smallest_location_for_all_seeds());
        } else {
            println!("{}", seed_data.smallest_location_for_all_ranges());
        }
    }
}
