use std::{collections::HashMap, fmt::format};

use crate::problem::problemdef::Problem;

pub struct DayTwentyFive {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
}

impl DayTwentyFive {
    fn read_input(input: &str) -> (HashMap<(usize, usize), Direction>, (usize, usize)) {
        let mut ret = HashMap::new();
        let mut dims = (
            input.lines().filter(|l| !l.is_empty()).count(),
            input
                .lines()
                .filter(|l| !l.is_empty())
                .next()
                .unwrap()
                .len(),
        );
        input
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .for_each(|(i, line)| {
                line.chars().enumerate().for_each(|(j, c)| {
                    if let Some(direction) = match c {
                        '.' => None,
                        '>' => Some(Direction::Right),
                        'v' => Some(Direction::Down),
                        _ => panic!("Unexpected character {}", c),
                    } {
                        ret.insert((i, j), direction);
                    }
                });
            });
        (ret, dims)
    }

    fn step(
        hm: &HashMap<(usize, usize), Direction>,
        height: usize,
        width: usize,
    ) -> (HashMap<(usize, usize), Direction>, bool) {
        let mut new_hm = HashMap::new();
        let mut moved = false;
        for ((i, j), direction) in hm.iter() {
            match direction {
                Direction::Right => {
                    let new_pos = (*i, (j + 1) % width);
                    if !hm.contains_key(&new_pos) {
                        new_hm.insert(new_pos, Direction::Right);
                        moved = true;
                    } else {
                        new_hm.insert((*i, *j), Direction::Right);
                    }
                }
                Direction::Down => {
                    new_hm.insert((*i, *j), Direction::Down);
                }
            }
        }
        let mut new_new_hm = HashMap::new();
        for ((i, j), direction) in new_hm.iter() {
            match direction {
                Direction::Right => {
                    new_new_hm.insert((*i, *j), Direction::Right);
                }
                Direction::Down => {
                    let new_pos = ((*i + 1) % height, *j);
                    if !new_hm.contains_key(&new_pos) {
                        new_new_hm.insert(new_pos, Direction::Down);
                        moved = true;
                    } else {
                        new_new_hm.insert((*i, *j), Direction::Down);
                    }
                }
            }
        }
        (new_new_hm, moved)
    }
}

impl Problem for DayTwentyFive {
    fn part_one(&self, input: &str) -> String {
        let (mut hm, (height, width)) = DayTwentyFive::read_input(input);
        let mut counter = 0;
        loop {
            let (hm_new, moved) = DayTwentyFive::step(&hm, height, width);
            hm = hm_new;
            counter += 1;
            if !moved {
                break;
            }
        }
        format!("{}", counter)
    }

    fn part_two(&self, input: &str) -> String {
        format!("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
