mod days;
mod problem;

use clap::Parser;
use days::*;
use problem::problemdef::{BenchmarkResult, Problem};
use std::error::Error;
use std::fs;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

const NOPROBLEM: usize = 1234;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = NOPROBLEM)]
    day: usize,

    #[arg(short, long, default_value_t = false)]
    benchmark: bool,

    #[arg(short, long, default_value_t = false)]
    run_all: bool,
}

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(day1::DayOne {})),
        2 => Some(Box::new(day2::DayTwo {})),
        3 => Some(Box::new(day3::DayThree {})),
        4 => Some(Box::new(day4::DayFour {})),
        5 => Some(Box::new(day5::DayFive {})),
        6 => Some(Box::new(day6::DaySix {})),
        7 => Some(Box::new(day7::DaySeven {})),
        8 => Some(Box::new(day8::DayEight {})),
        9 => Some(Box::new(day9::DayNine {})),
        10 => Some(Box::new(day10::DayTen {})),
        11 => Some(Box::new(day11::DayEleven {})),
        12 => Some(Box::new(day12::DayTwelve {})),
        13 => Some(Box::new(day13::DayThirteen {})),
        _ => None,
    }
}

fn run_problem(
    p: Box<dyn Problem>,
    benchmark: bool,
    day: usize,
) -> Result<Option<BenchmarkResult>, Box<dyn Error>> {
    let file_content = fs::read_to_string(format!("src/inputs/{}.txt", day))?;
    p.run(&file_content);

    if benchmark {
        return Ok(Some(p.benchmark(&file_content)));
    }

    Ok(None)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.day == NOPROBLEM && !args.run_all {
        return Err("Either --day <day> or --run-all must be provided.".into());
    }

    if args.run_all {
        let mut bench_results = vec![];

        for i in 0..=25 {
            let p = day_to_problem(i);
            if p.is_some() {
                println!("Day {}:", i);
                match run_problem(p.unwrap(), args.benchmark, i) {
                    Ok(Some(br)) => {
                        bench_results.push(br);
                        bench_results.last_mut().unwrap().set_problem_num(i)
                    }
                    Ok(None) => {}
                    Err(e) => return Err(e),
                }
                println!("");
            }
        }
        if args.benchmark {
            let serialized = serde_json::to_string(&bench_results).unwrap();
            println!("{}", serialized);
        }
    } else {
        let p = day_to_problem(args.day).ok_or("Day is not implemented")?;
        match run_problem(p, args.benchmark, args.day) {
            Ok(Some(br)) => {
                let serialized = serde_json::to_string_pretty(&br).unwrap();
                println!("{}", serialized);
            }
            Ok(None) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
