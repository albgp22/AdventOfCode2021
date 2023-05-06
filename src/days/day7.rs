use itertools::Itertools;
use std::fmt::Debug;
use std::str::FromStr;

use crate::problem::problemdef::Problem;

pub struct DaySeven {}

fn parse_input<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut lines = s.split('\n').filter(|l| !l.is_empty());
    lines
        .next()
        .unwrap()
        .split(',')
        .filter(|l| !l.is_empty())
        .map(|n_st| n_st.parse::<T>().unwrap())
        .collect_vec()
}

fn median(numbers: &mut Vec<i64>) -> Vec<i64> {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        vec![numbers[mid - 1], numbers[mid]]
    } else {
        vec![numbers[mid]]
    }
}

fn triangular_number(n: i64) -> i128 {
    let n = n as i128;
    n.checked_mul(n + 1).unwrap() / 2
}

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let mut positions: Vec<i64> = parse_input(input);
        let median = median(&mut positions);

        format!(
            "{}",
            median
                .iter()
                .map(|med| (med, positions.iter().map(|n| (n - med).abs()).sum::<i64>()))
                .min_by(|(_, x), (_, y)| x.cmp(y))
                .unwrap()
                .1
        )
    }

    fn part_two(&self, input: &str) -> String {
        let mut positions: Vec<i64> = parse_input(input);
        let median = median(&mut positions);

        let lb = *median.iter().min().unwrap();

        format!(
            "{:?}",
            (lb - 1000..lb + 1000)
                .map(|med| (
                    med,
                    positions
                        .iter()
                        .map(|n| triangular_number((n - med).abs()))
                        .fold(0i128, |acc, m| acc.checked_add(m).unwrap())
                ))
                .min_by(|(_, x), (_, y)| x.cmp(y))
                .unwrap()
                .1
        )
    }
}

#[cfg(test)]
mod tests {
    
}
