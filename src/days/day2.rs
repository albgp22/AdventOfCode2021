use std::str::FromStr;

use crate::problem::problemdef::Problem;

pub struct DayTwo {}

enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[derive(Debug)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.clone().split(" ");
        let ins_base = parts.next().ok_or(ParseInstructionError)?;
        let qua_st = parts.next().ok_or(ParseInstructionError)?;
        let qua: i64 = qua_st.parse().unwrap();
        let ins = match ins_base {
            "forward" => Instruction::Forward(qua),
            "down" => Instruction::Down(qua),
            "up" => Instruction::Up(qua),
            _ => return Err(ParseInstructionError),
        };

        Ok(ins)
    }
}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let mut x = 0;
        let mut y = 0;

        input
            .split("\n")
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|line| line.parse::<Instruction>().unwrap())
            .for_each(|i| match i {
                Instruction::Forward(q) => x += q,
                Instruction::Down(q) => y += q,
                Instruction::Up(q) => y -= q,
            });

        format!("{}", x * y)
    }

    fn part_two(&self, input: &str) -> String {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;

        input
            .split("\n")
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|line| line.parse::<Instruction>().unwrap())
            .for_each(|i| match i {
                Instruction::Forward(q) => {
                    x += q;
                    y += aim * q;
                }
                Instruction::Down(q) => aim += q,
                Instruction::Up(q) => aim -= q,
            });

        format!("{}", x * y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
