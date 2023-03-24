use std::str::FromStr;

use itertools::Itertools;
use std::fmt::Debug;

use crate::problem::problemdef::Problem;

pub struct DaySix {}

fn parse_input<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut lines = s.split("\n").filter(|l| !l.is_empty());
    lines
        .next()
        .unwrap()
        .split(",")
        .filter(|l| !l.is_empty())
        .map(|n_st| n_st.parse::<T>().unwrap())
        .collect_vec()
}

fn solve(input: &str, iterations: usize) -> String{
        let population: Vec<usize> = parse_input(input);

        let mut quant: Vec<usize> = vec![0; 9];

        for ind in population {
            quant[ind] += 1;
        }

        for _ in 0..iterations{
            let reproducing = quant[0];
            for i in 1..9{
                quant[i-1]=quant[i];
            }
            quant[8]=reproducing;
            quant[6]=quant[6].checked_add(reproducing).unwrap();
        }

        format!("{}", quant.iter().fold(0usize, |acc, i| acc.checked_add(*i).unwrap()))
}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        solve(input, 80)
    }

    fn part_two(&self, input: &str) -> String {
        solve(input, 256)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
}
