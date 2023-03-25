use itertools::Itertools;

use crate::problem::problemdef::Problem;
pub struct DayTen {}

impl DayTen {
    fn get_closing(c: char) -> char {
        match c {
            ')' => '(',
            ']' => '[',
            '>' => '<',
            '}' => '{',
            _ => unreachable!(),
        }
    }

    fn get_score(c: char) -> i32 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    }

    fn get_score_2(c: char) -> u128 {
        match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        }
    }

    fn corrupted<T: AsRef<str>>(line: T) -> Option<char> {
        let mut q = vec![];
        let line = line.as_ref();
        for c in line.chars() {
            if "([<{".contains(&c.to_string()) {
                q.push(c)
            } else {
                let popped = q.pop().unwrap();
                let expected_opening = Self::get_closing(c);
                if popped != expected_opening {
                    return Some(c);
                }
            }
        }
        None
    }

    fn get_completion<T: AsRef<str>>(line: T) -> Vec<char> {
        let mut q = vec![];
        let line = line.as_ref();
        for c in line.chars() {
            if "([<{".contains(&c.to_string()) {
                q.push(c)
            } else {
                q.pop().unwrap();
            }
        }
        q
    }
}

impl Problem for DayTen {
    fn part_one(&self, input: &str) -> String {
        let mut total_score = 0i32;
        input.split("\n").filter(|l| !l.is_empty()).for_each(|l| {
            if let Some(corr) = Self::corrupted(l) {
                total_score = total_score.checked_add(Self::get_score(corr)).unwrap();
            }
        });
        format!("{}", total_score)
    }

    fn part_two(&self, input: &str) -> String {
        let mut score_vec = input
            .split("\n")
            .filter(|l| !l.is_empty())
            .filter(|l| Self::corrupted(l).is_none())
            .map(|l| {
                Self::get_completion(l).iter().rev().fold(0u128, |acc, c| {
                    acc.checked_mul(5).unwrap() + Self::get_score_2(*c)
                })
            }).collect_vec();
        score_vec.sort();
        let score = score_vec.get(score_vec.len()/2).unwrap();
        format!("{}", score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
