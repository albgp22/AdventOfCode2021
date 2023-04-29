use crate::problem::problemdef::Problem;
use itertools::Itertools;

pub struct DayTwentyOne {}

impl DayTwentyOne {
    fn wins(p1: i128, s1: i128, p2: i128, s2: i128, rf: &Vec<(i128, i128)>) -> (i128, i128) {
        if s2 <= 0 {
            return (0, 1);
        }
        let (mut w1, mut w2) = (0i128, 0i128);
        for (r, f) in rf {
            let (c2, c1) = Self::wins(p2, s2, (p1 + r) % 10, s1 - 1 - (p1 + r) % 10, rf);
            (w1, w2) = (w1.checked_add(f.checked_mul(c1).unwrap()).unwrap(), w2.checked_add(f.checked_mul(c2).unwrap()).unwrap());
        }
        (w1, w2)
    }
}

impl Problem for DayTwentyOne {
    fn part_one(&self, input: &str) -> String {
        let (mut pos1, mut pos2) = input
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.split(":").nth(1).unwrap().trim().parse::<i128>().unwrap() - 1)
            .collect_tuple()
            .unwrap();
        let (mut score1, mut score2) = (0i128, 0i128);
        let dice = (1..=100i128).cycle();
        for (i, t) in dice.tuples::<(_, _, _)>().enumerate() {
            let roll: i128 = vec![t.0, t.1, t.2].iter().sum();
            if i % 2 == 0 {
                pos1 = (pos1 + roll) % 10;
                score1 += pos1 + 1;
                if score1 >= 1000 {
                    return format!("{}", (score2) * (i as i128 * 3 + 3));
                }
            } else {
                pos2 = (pos2 + roll) % 10;
                score2 += pos2 + 1;
                if score1 >= 1000 {
                    return format!("{}", (score1) * (i as i128 * 3 + 3));
                }
            }
        }
        format!("")
    }

    fn part_two(&self, input: &str) -> String {
        let roll_frequency = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let ( pos1,  pos2) = input
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.split(":").nth(1).unwrap().trim().parse::<i128>().unwrap()-1)
            .collect_tuple()
            .unwrap();
        let (w1, w2) = Self::wins(pos1, 21, pos2, 21, &roll_frequency);
        format!("{}", w1.max(w2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
