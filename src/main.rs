mod problem;
mod days;

use problem::problemdef::Problem;
use days::*;
use clap::Parser;
use std::fs;
use std::error::Error;

#[derive(Parser,Debug)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long, default_value_t = false)]
    benchmark: bool,
}

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(day1::DayOne{})),
        2 => Some(Box::new(day2::DayTwo{})),
        3 => Some(Box::new(day3::DayThree{})),
        4 => Some(Box::new(day4::DayFour{})),
        5 => Some(Box::new(day5::DayFive{})),
        _ => None
    }
}

fn main() -> Result<(), Box<dyn Error>>{

    let args = Args::parse();

    let problem = day_to_problem(args.day).ok_or("Day is not implemented")?;
    let file_content = fs::read_to_string(format!("src/inputs/{}.txt",args.day))?;

    problem.run(&file_content);

    if args.benchmark{
        problem.benchmark(&file_content);
    }

    Ok(())
}
