use crate::problem::problemdef::Problem;
use std::str::FromStr;

pub struct DayFour {}

#[derive(Debug)]
struct BingoBoard {
    numbers: Vec<Vec<usize>>,
}

impl BingoBoard {
    fn substitute_value(&mut self, bef: usize, aft: usize) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[0].len() {
                if self.numbers[i][j] == bef {
                    self.numbers[i][j] = aft;
                }
            }
        }
    }

    fn won(&self, target: usize) -> bool {
        for i in 0..self.numbers.len() {
            if self.numbers[i].iter().all(|n| *n == target) {
                return true;
            }
        }
        for j in 0..self.numbers[0].len() {
            if (0..self.numbers.len())
                .map(|i| self.numbers[i][j])
                .all(|n| n == target)
            {
                return true;
            }
        }

        false
    }

    fn sum_rest(&self, except: usize) -> usize {
        let mut ret = 0;
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[0].len() {
                if self.numbers[i][j] != except {
                    ret += self.numbers[i][j];
                }
            }
        }
        ret
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBingoBoardError;

impl FromStr for BingoBoard {
    type Err = ParseBingoBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = vec![vec![0; 5]; 5];

        s.split('\n')
            .filter(|l| !l.is_empty())
            .enumerate()
            .for_each(|(i, l)| {
                l.split(' ')
                    .filter(|st| !st.is_empty())
                    .enumerate()
                    .for_each(|(j, c)| {
                        v[i][j] = c.to_string().parse().unwrap();
                    })
            });

        Ok(BingoBoard { numbers: v })
    }
}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let mut input_parts = input.split("\n\n").filter(|l| !l.is_empty());

        let numbers = input_parts
            .next()
            .unwrap()
            .split(',')
            .map(|i_st| i_st.parse::<usize>().unwrap());

        let mut boards: Vec<BingoBoard> = input_parts
            .map(|board_st| BingoBoard::from_str(board_st).unwrap())
            .collect();

        const NEMPTY: usize = 12345678;

        for n in numbers {
            for board in &mut boards {
                board.substitute_value(n, NEMPTY);
                if board.won(NEMPTY) {
                    return format!("{}", board.sum_rest(NEMPTY) * n);
                }
            }
        }

        "Fail".to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut input_parts = input.split("\n\n").filter(|l| !l.is_empty());

        let numbers = input_parts
            .next()
            .unwrap()
            .split(',')
            .map(|i_st| i_st.parse::<usize>().unwrap());

        let mut boards: Vec<BingoBoard> = input_parts
            .map(|board_st| BingoBoard::from_str(board_st).unwrap())
            .collect();

        const NEMPTY: usize = 12345678;

        for n in numbers {
            let boards_remain = boards.iter().filter(|b| !b.won(NEMPTY)).count();

            let board_left_idx = if boards_remain == 1 {
                Some(
                    boards
                        .iter()
                        .enumerate()
                        .filter(|(_i, b)| !b.won(NEMPTY))
                        .map(|(i, _b)| i)
                        .next()
                        .unwrap(),
                )
            } else {
                None
            };

            for board in &mut boards {
                board.substitute_value(n, NEMPTY);
            }
            let boards_after = boards.iter().filter(|b| !b.won(NEMPTY)).count();

            if boards_after == 0 {
                return format!("{}", boards[board_left_idx.unwrap()].sum_rest(NEMPTY) * n);
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    
}
