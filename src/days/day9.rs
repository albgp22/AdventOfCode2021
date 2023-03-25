extern crate queues;
use std::collections::HashSet;

use crate::problem::problemdef::Problem;
use queues::*;
pub struct DayNine {}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        if u < (i.abs() as usize) {
            None
        } else {
            Some(u.checked_sub(i.abs() as usize).unwrap())
        }
    } else {
        Some(u + i as usize)
    }
}

impl DayNine {
    fn parse_input(input: &str) -> Vec<Vec<u32>> {
        let mut ret = vec![];
        input.split("\n").filter(|l| !l.is_empty()).for_each(|l| {
            ret.push(
                l.chars()
                    .map(|c| c.to_string().parse::<u32>().unwrap())
                    .collect(),
            )
        });
        ret
    }

    fn neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        for di in -1..=1i32 {
            for dj in -1..=1i32 {
                if di.abs() + dj.abs() > 1 || di.abs() + dj.abs() == 0 {
                    continue;
                }
                let newi = add(i, di);
                let newj = add(j, dj);
                if newi.is_none() || newj.is_none() {
                    continue;
                }
                ret.push((newi.unwrap(), newj.unwrap()));
            }
        }
        ret
    }
}

impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let mut total_risk_level: u32 = 0;
        let map = DayNine::parse_input(input);
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                let mut found_lower = false;
                'vn: for di in -1..=1i32 {
                    for dj in -1..=1i32 {
                        let newi = add(i, di);
                        let newj = add(j, dj);
                        if newi.is_none() || newj.is_none() {
                            continue;
                        }
                        let (newi, newj) = (newi.unwrap(), newj.unwrap());
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        if newi >= map.len() {
                            continue;
                        }
                        if newj >= map[newi].len() {
                            continue;
                        }
                        if map[newi][newj] < map[i][j] {
                            found_lower = true;
                            break 'vn;
                        }
                    }
                }
                if !found_lower {
                    total_risk_level += map[i][j] + 1;
                }
            }
        }
        format!("{}", total_risk_level)
    }

    fn part_two(&self, input: &str) -> String {
        let map = DayNine::parse_input(input);
        let mut sinks = vec![];
        let mut basin_sizes = vec![];
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                let mut found_lower = false;
                'vn: for di in -1..=1i32 {
                    for dj in -1..=1i32 {
                        let newi = add(i, di);
                        let newj = add(j, dj);
                        if newi.is_none() || newj.is_none() {
                            continue;
                        }
                        let (newi, newj) = (newi.unwrap(), newj.unwrap());
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        if di.abs()+dj.abs()>1{continue;}
                        if newi >= map.len() {
                            continue;
                        }
                        if newj >= map[newi].len() {
                            continue;
                        }
                        if map[newi][newj] < map[i][j] {
                            found_lower = true;
                            break 'vn;
                        }
                    }
                }
                if !found_lower {
                    sinks.push((i, j));
                }
            }
        }

        for sink in sinks {
            let mut q: Queue<((usize, usize), usize)> =
                queue![(sink, map[sink.0][sink.1] as usize)];
            let mut basin: Vec<(usize, usize)> = vec![];
            while q.size() > 0 {
                let ((i, j), h) = q.remove().unwrap();
                if map[i][j] == 9 {
                    continue;
                }
                basin.push((i, j));
                for (ii, jj) in DayNine::neighbors(i, j) {
                    if map.get(ii).is_none() || map.get(ii).unwrap().get(jj).is_none() {
                        continue;
                    }
                    if !basin.contains(&(ii, jj)) {
                        if map[ii][jj] as usize > h {
                            q.add(((ii, jj), map[ii][jj] as usize)).unwrap();
                        }
                    }
                }
            }
            let basin = basin.iter().collect::<HashSet<_>>();
            basin_sizes.push(basin.len());
            /*println!(
                "Sink {:?} has a basin of size {}, which is {:?}",
                sink,
                basin.len(),
                basin
            );*/
        }
        basin_sizes.sort();
        format!(
            "{}",
            basin_sizes
                .iter()
                .rev()
                .take(3)
                .fold(1usize, |acc, i| acc.checked_mul(*i).unwrap())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
