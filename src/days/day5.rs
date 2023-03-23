use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::problem::problemdef::Problem;

pub struct DayFive {}

#[derive(Debug)]
struct Line {
    origin: Vec<i32>,
    destination: Vec<i32>,
}

impl Line {
    fn get_points(&self, consider_diagonals: bool) -> Vec<(i32, i32)> {
        let (a, b, c, d) = (
            self.origin[0],
            self.origin[1],
            self.destination[0],
            self.destination[1],
        );
        if a == c {
            if b > d { d..b + 1 } else { b..d + 1 }
                .map(|y| (a, y))
                .collect()
        } else if d == b {
            if c > a { a..c + 1 } else { c..a + 1 }
                .map(|x| (x, b))
                .collect()
        } else {
            if !consider_diagonals {
                vec![]
            } else {
                assert!((a - c).abs() == (b - d).abs());
                (0..(a - c).abs()+1)
                    .map(|delta| (a + delta * (c - a).signum(), b + (d - b).signum() * delta))
                    .collect()
            }
        }
    }
}

#[derive(Debug)]
struct ParseLineError;

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        Ok(Line {
            origin: parts
                .next()
                .unwrap()
                .split(",")
                .map(|n_st| n_st.parse().unwrap())
                .collect(),
            destination: parts
                .next()
                .unwrap()
                .split(",")
                .map(|n_st| n_st.parse().unwrap())
                .collect(),
        })
    }
}

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let lines = input
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| Line::from_str(l).unwrap())
            .collect_vec();

        let mut points = HashMap::new();

        for line in lines {
            for point in line.get_points(false) {
                if points.contains_key(&point) {
                    *points.get_mut(&point).unwrap() += 1;
                } else {
                    points.insert(point, 1);
                }
            }
        }

        format!("{}", points.iter().filter(|((_x, _y), q)| **q > 1).count())
    }

    fn part_two(&self, input: &str) -> String {
        let lines = input
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| Line::from_str(l).unwrap())
            .collect_vec();

        let mut points = HashMap::new();

        for line in lines {
            for point in line.get_points(true) {
                if points.contains_key(&point) {
                    *points.get_mut(&point).unwrap() += 1;
                } else {
                    points.insert(point, 1);
                }
            }
        }

        format!("{}", points.iter().filter(|((_x, _y), q)| **q > 1).count())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
}
