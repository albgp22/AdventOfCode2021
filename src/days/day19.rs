use std::collections::{HashMap, HashSet};

use crate::problem::problemdef::Problem;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

type Offset = Point;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash)]
struct Orientation {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Scan {
    dots: Vec<Point>,
}

pub struct DayNineteen {}

impl DayNineteen {
    fn read_input(input: &str) -> Vec<Scan> {
        vec![]
    }
    fn overlapp(s0: &Scan, s1: &Scan) -> Option<(Orientation, Offset)> {
        Some((Orientation {}, Offset { x: 0, y: 0, z: 0 }))
    }
    fn reconstruct(
        s: &Vec<Scan>,
        o: &HashMap<(usize, usize), (Orientation, Offset)>,
    ) -> HashSet<Point> {
        HashSet::new()
    }
}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {
        let scans = Self::read_input(input);
        let n_sensors = scans.len();
        let mut overlaps = HashMap::new();
        for i in 0..n_sensors {
            for j in i + 1..n_sensors {
                let r = Self::overlapp(&scans[i], &scans[j]);
                if r.is_some() {
                    overlaps.insert((i, j), r.unwrap());
                }
            }
        }
        let reconstructed_map = Self::reconstruct(&scans, &overlaps);

        format!("{}", reconstructed_map.len())
    }

    fn part_two(&self, input: &str) -> String {
        format!("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
