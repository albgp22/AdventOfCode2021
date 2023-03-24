mod problem;
mod days;

use problem::problemdef::Problem;
use days::*;
use clap::Parser;
use std::fs;
use std::error::Error;

const NOPROBLEM: usize = 1234;

#[derive(Parser,Debug)]
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
        1 => Some(Box::new(day1::DayOne{})),
        2 => Some(Box::new(day2::DayTwo{})),
        3 => Some(Box::new(day3::DayThree{})),
        4 => Some(Box::new(day4::DayFour{})),
        5 => Some(Box::new(day5::DayFive{})),
        6 => Some(Box::new(day6::DaySix{})),
        _ => None
    }
}

fn run_problem(p: Box<dyn Problem>, benchmark: bool, day: usize) -> Result<(), Box<dyn Error>>{

    let file_content = fs::read_to_string(format!("src/inputs/{}.txt",day))?;
    p.run(&file_content);

    if benchmark{
        p.benchmark(&file_content);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{

    let args = Args::parse();


    if args.day==NOPROBLEM && !args.run_all{
        return Err("Either --day <day> or --run-all must be provided.".into());
    }

    if args.run_all{
        (0..=25).for_each(|i|{
            let p = day_to_problem(i);
            if p.is_some(){
                println!("Day {}:", i);
                run_problem(p.unwrap(), args.benchmark, i).unwrap();
                println!("");
            }
        })
    }else{
        let p = day_to_problem(args.day).ok_or("Day is not implemented")?;
        return run_problem(p, args.benchmark, args.day)
    }


    Ok(())
}
