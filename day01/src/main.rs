use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;

mod day01;
mod day02;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: String,

    #[arg(short, long)]
    input: String,
}
fn main() {
    let args = Args::parse();
    match args.day.as_str() {
        "1" => day01::day01(args.input),
        "2" => day02::day02(args.input),
        _ => panic!("invalid day")
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
