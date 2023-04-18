use std::collections::{HashMap, HashSet};
use std::ops::Sub;

use crate::problem::problemdef::Problem;

pub struct DayEight {}

fn parse_input(s: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut ret = vec![];
    let lines = s.split("\n").filter(|l| !l.is_empty());
    lines.for_each(|l| {
        let mut lineparts = l.split("|");
        let part1v = lineparts
            .next()
            .unwrap()
            .split(" ")
            .filter(|n| !n.is_empty())
            .collect();
        let part2v = lineparts
            .next()
            .unwrap()
            .split(" ")
            .filter(|n| !n.is_empty())
            .collect();
        ret.push((part1v, part2v));
    });

    ret
}
#[derive(Debug, Default)]
struct Mapping {
    m: Vec<(HashSet<char>, i32)>,
}

impl Mapping {
    fn new(digits: &Vec<&str>) -> Self {
        let mut v: Vec<(HashSet<char>, i32)> = vec![];

        // 1,7,4,8
        for d in digits {
            match d.len() {
                2 => v.push((d.chars().collect(), 1)),
                3 => v.push((d.chars().collect(), 7)),
                4 => v.push((d.chars().collect(), 4)),
                7 => v.push((d.chars().collect(), 8)),
                _ => {}
            }
        }

        let remaining: Vec<&&str> = digits
            .iter()
            .filter(|l| !vec![2, 3, 4, 7].contains(&l.len()))
            .collect();

        for cero in remaining.iter().filter(|l| l.len() == 6) {
            for two in remaining.iter().filter(|l| l.len() == 5) {
                for three in remaining.iter().filter(|l| l.len() == 5) {
                    if two == three {
                        continue;
                    }
                    for five in remaining.iter().filter(|l| l.len() == 5) {
                        if two == five || three == five {
                            continue;
                        }
                        for six in remaining.iter().filter(|l| l.len() == 6) {
                            if six == cero {
                                continue;
                            }
                            for nine in remaining.iter().filter(|l| l.len() == 6) {
                                if nine == cero || nine == six {
                                    continue;
                                }
                                let mut v_proposal = v.to_vec();
                                v_proposal.push((cero.chars().collect(), 0));
                                v_proposal.push((two.chars().collect(), 2));
                                v_proposal.push((three.chars().collect(), 3));
                                v_proposal.push((five.chars().collect(), 5));
                                v_proposal.push((six.chars().collect(), 6));
                                v_proposal.push((nine.chars().collect(), 9));
                                if Self::is_ok(&v_proposal) {
                                    return Self { m: v_proposal };
                                }
                            }
                        }
                    }
                }
            }
        }

        Self { m: v }
    }

    fn is_ok(v: &Vec<(HashSet<char>, i32)>) -> bool {
        let mut contains: HashMap<i32, Vec<i32>> = HashMap::new();
        contains.insert(0, vec![1, 7]);
        contains.insert(1, vec![]);
        contains.insert(2, vec![]);
        contains.insert(3, vec![1, 7]);
        contains.insert(4, vec![1]);
        contains.insert(5, vec![]);
        contains.insert(6, vec![5]);
        contains.insert(7, vec![1]);
        contains.insert(8, vec![0, 1, 2, 3, 4, 5, 6, 7, 9]);
        contains.insert(9, vec![5]);

        let get_num_chars = |i| v.iter().find(|(_, j)| j == i).unwrap().0.clone();

        contains.iter().all(|(i, deps)| {
            deps.iter()
                .all(|j| get_num_chars(j).is_subset(&get_num_chars(i)))
        }) && get_num_chars(&1).is_superset(&get_num_chars(&8).sub(&get_num_chars(&6)))
            && get_num_chars(&1).is_superset(&get_num_chars(&8).sub(&get_num_chars(&6)))
            && !get_num_chars(&7).is_superset(&get_num_chars(&8).sub(&get_num_chars(&0)))
            && get_num_chars(&6).is_superset(&get_num_chars(&8).sub(&get_num_chars(&0)))
            && get_num_chars(&5).is_superset(&get_num_chars(&8).sub(&get_num_chars(&0)))
            && get_num_chars(&2).is_superset(&get_num_chars(&8).sub(&get_num_chars(&0)))
    }

    fn get(&self, tofind: &HashSet<char>) -> i128 {
        //println!("Trying to get {:?} from {:?}", tofind, self.m);
        self.m
            .iter()
            .find(|(chrs, _i)| chrs == tofind)
            .unwrap()
            .1
            .into()
    }
}

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let digits = parse_input(input);
        let particular_digits = vec![2, 4, 3, 7];

        format!(
            "{}",
            digits
                .iter()
                .map(|(_v1, v2)| v2)
                .flatten()
                .map(|s| s.len())
                .filter(|n| particular_digits.contains(n))
                .count()
        )
    }

    fn part_two(&self, input: &str) -> String {
        let digits = parse_input(input);
        format!(
            "{}",
            digits
                .iter()
                .map(|(allnums, digits)| {
                    let mapping = Mapping::new(allnums);
                    digits
                        .iter()
                        .map(|d| {
                            let digit_chars = d.chars().collect::<HashSet<_>>();
                            mapping.get(&digit_chars)
                        })
                        .fold(0, |acc, new| acc * 10 + new)
                })
                .sum::<i128>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
