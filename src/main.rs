use clap::Parser;

mod challenges;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Challenge Day
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Day1(challenges::day01::Args),
    Day2(challenges::day02::Args),
    Day3(challenges::day03::Args),
    Day4(challenges::day04::Args),
    Day5(challenges::day05::Args),
    Day6(challenges::day06::Args),
    Day7(challenges::day07::Args),
    Day8(challenges::day08::Args),
    Day9(challenges::day09::Args),
    Day10(challenges::day10::Args),
    Day11(challenges::day11::Args),
    Day12(challenges::day12::Args),
    Day13(challenges::day13::Args),
    Day14(challenges::day14::Args),
    Day15(challenges::day15::Args),
    Day16(challenges::day16::Args),
    Day17(challenges::day17::Args),
    Day18(challenges::day18::Args),
    Day19(challenges::day19::Args),
    Day20(challenges::day20::Args),
    Day21(challenges::day21::Args),
    Day22(challenges::day22::Args),
    Day23(challenges::day23::Args),
    Day24(challenges::day24::Args),
    Day25(challenges::day25::Args),
    Day26(challenges::day26::Args),
    Day27(challenges::day27::Args),
    Day28(challenges::day28::Args),
    Day29(challenges::day29::Args),
    Day30(challenges::day30::Args),
    Day31(challenges::day31::Args),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Day1(args) => challenges::day01::entrypoint(args),
        Command::Day2(args) => challenges::day02::entrypoint(args),
        Command::Day3(args) => challenges::day03::entrypoint(args),
        Command::Day4(args) => challenges::day04::entrypoint(args),
        Command::Day5(args) => challenges::day05::entrypoint(args),
        Command::Day6(args) => challenges::day06::entrypoint(args),
        Command::Day7(args) => challenges::day07::entrypoint(args),
        Command::Day8(args) => challenges::day08::entrypoint(args),
        Command::Day9(args) => challenges::day09::entrypoint(args),
        Command::Day10(args) => challenges::day10::entrypoint(args),
        Command::Day11(args) => challenges::day11::entrypoint(args),
        Command::Day12(args) => challenges::day12::entrypoint(args),
        Command::Day13(args) => challenges::day13::entrypoint(args),
        Command::Day14(args) => challenges::day14::entrypoint(args),
        Command::Day15(args) => challenges::day15::entrypoint(args),
        Command::Day16(args) => challenges::day16::entrypoint(args),
        Command::Day17(args) => challenges::day17::entrypoint(args),
        Command::Day18(args) => challenges::day18::entrypoint(args),
        Command::Day19(args) => challenges::day19::entrypoint(args),
        Command::Day20(args) => challenges::day20::entrypoint(args),
        Command::Day21(args) => challenges::day21::entrypoint(args),
        Command::Day22(args) => challenges::day22::entrypoint(args),
        Command::Day23(args) => challenges::day23::entrypoint(args),
        Command::Day24(args) => challenges::day24::entrypoint(args),
        Command::Day25(args) => challenges::day25::entrypoint(args),
        Command::Day26(args) => challenges::day26::entrypoint(args),
        Command::Day27(args) => challenges::day27::entrypoint(args),
        Command::Day28(args) => challenges::day28::entrypoint(args),
        Command::Day29(args) => challenges::day29::entrypoint(args),
        Command::Day30(args) => challenges::day30::entrypoint(args),
        Command::Day31(args) => challenges::day31::entrypoint(args),
    }
}
