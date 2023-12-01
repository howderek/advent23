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
    Day1(challenges::day1::Args),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Day1(args) => challenges::day1::entrypoint(args),
    }
}
