mod problem;
mod days;

use problem::problemdef::Problem;
use days::day1::DayOne;
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
        1 => Some(Box::new(DayOne{})),
        _ => None
    }
}

fn main() -> Result<(), Box<dyn Error>>{

    let args = Args::parse();

    let problem = day_to_problem(args.day).ok_or("Day is not implemented")?;
    let file_content = fs::read_to_string(format!("inputs/{}.txt",args.day))?;

    problem.run(&file_content);

    Ok(())
}
