template = """use clap;
use std::{fs, str};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(default_value_t = String::from("./inputs/day{day}/input.txt"))]
    file: String,

    #[clap(long, short, action)]
    part2: bool,
}

pub fn entrypoint(args: &Args) {
    let _input = fs::read_to_string(&args.file).expect("I/O error");
    println!("this day has not yet been implemented")
}
"""

for day in range(1, 32):
    filename = f"day{day}.rs"
    with open(filename, 'w') as file:
        file.write(template.replace('{day}', str(day)))
    print(f"Created {filename}")

print("All files created.")
