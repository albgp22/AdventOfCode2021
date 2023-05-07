use crate::problem::problemdef::Problem;
use itertools::Itertools;

pub struct DayThree {}

fn get_most_common_bits<T, U>(input: T, len: usize) -> Vec<i32>
where
    T: Iterator<Item = U>,
    U: AsRef<str>,
{
    let mut occ = vec![0_i32; len];

    input.for_each(|l| {
        l.as_ref()
            .chars()
            .enumerate()
            .for_each(|(i, c)| match c.to_string().parse().unwrap() {
                0 => occ[i] -= 1,
                1 => occ[i] += 1,
                _ => panic!("Unreachable"),
            });
    });

    occ.iter().map(|i| i.signum()).collect_vec()
}

fn get_rating(input: impl AsRef<str>, binlen: usize, o2: bool) -> i32 {
    let mut numbers = input
        .as_ref()
        .split('\n')
        .filter(|&s| !s.is_empty())
        .collect_vec();
    let mut i = 0;
    let mut res = 0;

    while i < binlen && numbers.len() > 1 {
        let mcb = get_most_common_bits(numbers.iter(), binlen);
        numbers.retain(|l| {
            l.chars()
                .nth(i)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
                == if o2 {
                    if mcb[i] != -1 {
                        1
                    } else {
                        0
                    }
                } else if mcb[i] != -1 {
                    0
                } else {
                    1
                }
        });
        i += 1;
    }

    assert!(numbers.len() == 1);

    // Convert to decimal
    numbers[0].chars().rev().enumerate().for_each(|(i, d)| {
        if d.to_string().parse::<i32>().unwrap() != 0 {
            res += 1 << i
        }
    });

    res
}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let mut gamma = 0;
        let mut eps = 0;
        let mcb = get_most_common_bits(
            input
                .split('\n')
                .filter(|s| !s.is_empty()),
            input.split('\n').next().unwrap().len(),
        );

        mcb.iter().rev().enumerate().for_each(|(i, q)| {
            if q == &-1 {
                eps += 1 << i;
            } else {
                gamma += 1 << i;
            }
        });

        format!("{}", eps * gamma)
    }

    fn part_two(&self, input: &str) -> String {
        let binlen = input.split('\n').next().unwrap().len();

        let oxigen = get_rating(input, binlen, true);
        let co2 = get_rating(input, binlen, false);

        format!("{}", oxigen * co2)
    }
}

#[cfg(test)]
mod tests {
    
}
