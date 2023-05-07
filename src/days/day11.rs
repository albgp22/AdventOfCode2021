use itertools::Itertools;

use crate::problem::problemdef::Problem;

pub struct DayEleven {}

impl DayEleven {
    fn parse(input: &str) -> Vec<Vec<usize>> {
        input
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect_vec()
            })
            .collect()
    }

    fn add(u: usize, i: i32) -> Option<usize> {
        if i.is_negative() {
            if u < (i.unsigned_abs() as usize) {
                None
            } else {
                Some(u.checked_sub(i.unsigned_abs() as usize).unwrap())
            }
        } else {
            Some(u + i as usize)
        }
    }

    fn neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for di in -1..=1 {
            for dj in -1..=1 {
                if let Some(newi) = Self::add(i, di) {
                    if let Some(newj) = Self::add(j, dj) {
                            v.push((newi, newj));
                    }
                }
            }
        }
        v
    }

    fn step(a: &mut Vec<Vec<usize>>) -> usize {
        for i in 0..a.len() {
            for j in 0..a[0].len() {
                a[i][j] += 1;
            }
        }
        let mut flashed: Vec<(usize, usize)> = vec![];
        let mut changed = true;

        while changed {
            changed = false;
            for i in 0..a.len() {
                for j in 0..a[0].len() {
                    if a[i][j] > 9 && !flashed.contains(&(i, j)) {
                        changed = true;
                        flashed.push((i, j));
                        for (ii, jj) in Self::neighbors(i, j) {
                            if ii > a.len() - 1 || jj > a.get(0).unwrap().len() - 1 {
                                continue;
                            }
                            a[ii][jj] += 1;
                        }
                    }
                }
            }
        }

        for (i, j) in &flashed {
            a[*i][*j] = 0;
        }

        flashed.len()
    }
}

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let mut mat = DayEleven::parse(input);
        let res = (0..100).map(|_| Self::step(&mut mat)).sum::<usize>();
        format!("{}", res)
    }

    fn part_two(&self, input: &str) -> String {
        let mut mat = DayEleven::parse(input);
        let mut i = 0;
        let size = mat.len() * mat[0].len();
        while Self::step(&mut mat) != size {
            i += 1;
        }
        format!("{}", i + 1)
    }
}
