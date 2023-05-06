use crate::problem::problemdef::Problem;
use itertools::Itertools;

pub struct DayOne {}

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|i| i.parse::<i32>().unwrap())
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|i| i.parse::<i32>().unwrap())
            .tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    
}
