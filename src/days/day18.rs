use itertools::{iproduct, Itertools};

use crate::problem::problemdef::Problem;
use std::{ops::Add, str::FromStr};

#[derive(Debug, Clone)]
struct VecTree {
    vals: Vec<u32>,
    depths: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseVecTreeError;

impl FromStr for VecTree {
    type Err = ParseVecTreeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = VecTree {
            vals: Vec::new(),
            depths: Vec::new(),
        };

        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                ',' => (),
                ']' => {
                    depth -= 1;
                }
                d => {
                    t.vals.push(d.to_digit(10).unwrap());
                    t.depths.push(depth - 1);
                }
            }
        }
        Ok(t)
    }
}

impl Add for VecTree {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            vals: self.vals.iter().chain(other.vals.iter()).cloned().collect(),
            depths: self
                .depths
                .iter()
                .chain(other.depths.iter())
                .cloned()
                .map(|x| x + 1)
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ExplodeError;

#[derive(Debug, PartialEq, Eq)]
struct SplitError;

impl VecTree {
    fn try_explode(&mut self) -> Result<Self, ExplodeError> {
        for i in 0..self.depths.len() {
            let depth = self.depths[i];
            if depth != 4 {
                continue;
            }

            if i != 0 {
                self.vals[i - 1] += self.vals[i];
            }

            if i + 2 < self.vals.len() {
                self.vals[i + 2] += self.vals[i + 1];
            }

            self.vals[i] = 0;
            self.depths[i] = 3;
            self.vals.remove(i + 1);
            self.depths.remove(i + 1);

            return Ok(self.clone());
        }

        Err(ExplodeError)
    }

    fn try_split(&mut self) -> Result<Self, SplitError> {
        for i in 0..self.vals.len() {
            let v = self.vals[i];
            if v < 10 {
                continue;
            }

            let (a, b) = if v % 2 == 0 {
                (v / 2, v / 2)
            } else {
                (v / 2, v / 2 + 1)
            };

            self.vals[i] = a;
            self.depths[i] += 1;
            self.vals.insert(i + 1, b);
            self.depths.insert(i + 1, self.depths[i]);

            return Ok(self.clone());
        }

        Err(SplitError)
    }

    fn reduce(&mut self) -> Self {
        loop {
            if self.try_explode().is_err() && self.try_split().is_err() {
                break;
            }
        }
        self.clone()
    }

    fn score(&self) -> u32 {
        let mut vals = self.vals.clone();
        let mut depths = self.depths.clone();

        while vals.len() > 1 {
            for i in 0..depths.len() - 1 {
                if depths[i] == depths[i + 1] {
                    vals[i] = 3 * vals[i] + 2 * vals[i + 1];
                    vals.remove(i + 1);
                    depths.remove(i + 1);

                    if depths[i] > 0 {
                        depths[i] -= 1;
                    }

                    break;
                }
            }
        }

        vals[0]
    }
}

pub struct DayEighteen {}

impl Problem for DayEighteen {
    fn part_one(&self, input: &str) -> String {
        let mut lines = input.lines().filter(|l| !l.is_empty());
        let first = VecTree::from_str(lines.next().unwrap()).unwrap();
        lines
            .map(|l| l.parse::<VecTree>().unwrap())
            .fold(first, |n, acc| (n + acc).reduce())
            .score()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();
        iproduct!(lines.iter().enumerate(), lines.iter().enumerate())
            .filter(|((i, _), (j, _))| i < j)
            .map(|((_, x), (_, y))| {
                (x.parse::<VecTree>().unwrap() + y.parse::<VecTree>().unwrap())
                    .reduce()
                    .score()
            })
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {}
