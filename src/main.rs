use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

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
        "3" => day03::solve(args.input),
        "4" => day04::solve(args.input),
        "5" => day05::solve(args.input),
        "6" => day06::solve(args.input),
        "7" => day07::solve(args.input),
        "8" => day08::solve(args.input),
        "9" => day09::solve(args.input),
        "10" => day10::solve(args.input),
        "11" => day11::solve(args.input),
        "12" => day12::solve(args.input),
        "13" => {let _ = day13::solve(args.input);},
        "14" => day14::solve(args.input),
        "15" => day15::solve(args.input),
        "16" => day16::solve(args.input),
        "17" => day17::solve(args.input),
        "18" => day18::solve(args.input),
        "19" => day19::solve(args.input),
        "20" => day20::solve(args.input),
        "21" => day21::solve(args.input),
        "22" => day22::solve(args.input),
        "23" => day23::solve(args.input),
        "24" => day24::solve(args.input),
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
