use std::{collections::{HashMap, HashSet}, str::FromStr};

use itertools::Itertools;

use crate::problem::problemdef::Problem;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .tuples()
            .next()
            .ok_or(ParsePointError)?;

        let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;
        let z_fromstr = z.parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point { x: x_fromstr, y: y_fromstr , z: z_fromstr})
    }
}

type Offset = Point;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash)]
struct Orientation {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Scan {
    dots: Vec<Point>,
    distances: HashMap<(usize, usize), i32>,
}

impl Scan {
    fn get_all_distances(&self) -> Vec<i32> {
        self.distances.values().map(|i| *i).collect_vec()
    }
    fn overlaps_distances(&self, other: &Self) -> bool {
        let distances_a = self.get_all_distances();
        let distances_b = other.get_all_distances();
        let elements_a: HashSet<i32> = HashSet::from_iter(distances_a.iter().cloned());
        let count = |v: &Vec<i32>, e: i32| v.iter().filter(|&n| *n == e).count();

        let mut matches = 0;
        for e in elements_a {
            matches += count(&distances_a, e).min(count(&distances_b, e));
        }

        matches >= 66
    }
}

pub struct DayNineteen {}

impl DayNineteen {
    fn read_input(input: &str) -> Vec<Scan> {
        let mut v_curr = vec![];
        let mut r = vec![];
        let mut it = input.split('\n').filter(|l| !l.is_empty()).peekable();
        while let Some(line) = it.next()  {
            if line.contains("---"){
                if v_curr.is_empty(){ continue; }
                let distances = Self::compute_manhattan_distances(&v_curr);
                r.push(Scan { dots: v_curr, distances: distances });
                v_curr = vec![];
            }else if it.peek().is_none(){
                v_curr.push(Point::from_str(line).unwrap());
                let distances = Self::compute_manhattan_distances(&v_curr);
                r.push(Scan { dots: v_curr, distances: distances });
                v_curr = vec![];
            }else{
                v_curr.push(Point::from_str(line).unwrap());
            }
        }
        r
    }
    fn compute_manhattan_distances(ps: &Vec<Point>) -> HashMap<(usize, usize), i32> {
        // If 12 points match then at least 12*11/2=66 distances would be the same
        let mut r = HashMap::new();
        for (i, p1) in ps.iter().enumerate() {
            for (j, p2) in ps.iter().skip(i).enumerate() {
                r.insert(
                    (i, j),
                    (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs(),
                );
            }
        }
        r
    }
    fn overlapp(idx1: usize, idx2: usize, s0: &Scan, s1: &Scan) -> Option<(Orientation, Offset)> {
        if s0.overlaps_distances(s1){
            //println!("overlapp detected between {} and {}!",idx1,idx2);
        }
        Some((Orientation {}, Offset { x: 0, y: 0, z: 0 }))
    }
    fn reconstruct(
        scans: &Vec<Scan>,
        orientations: &HashMap<(usize, usize), (Orientation, Offset)>,
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
                /* Idea:
                you dont't really need to run this particular pair if there's already a path i--j
                 */
                let r = Self::overlapp(i,j,&scans[i], &scans[j]);
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
