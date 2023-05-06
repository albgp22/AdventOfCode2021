use crate::problem::problemdef::Problem;
use itertools::Itertools;

pub struct DayTwenty {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Mode {
    On,
    Off,
}

impl Mode {
    fn to_bin(&self) -> char {
        match self {
            Self::Off => '0',
            Self::On => '1',
        }
    }
}

impl DayTwenty {
    fn read_input(input: &str) -> (Vec<Mode>, Vec<Vec<Mode>>) {
        let mut lines = input.lines().filter(|l| !l.is_empty());
        let algorithm = lines.next().unwrap();
        (
            algorithm
                .chars()
                .map(|c| match c {
                    '.' => Mode::Off,
                    '#' => Mode::On,
                    _ => unreachable!(),
                })
                .collect_vec(),
            lines
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '.' => Mode::Off,
                            '#' => Mode::On,
                            _ => unreachable!(),
                        })
                        .collect_vec()
                })
                .collect_vec(),
        )
    }
    fn enlarge_image(v: &mut Vec<Vec<Mode>>, mode: Mode) {
        for vv in v.iter_mut() {
            vv.insert(0, mode);
            vv.push(mode)
        }
        let len = v[0].len();
        v.insert(0, vec![mode; len]);
        v.push(vec![mode; len]);
    }
    fn try_add(x: usize, dx: i32) -> Option<usize> {
        if x == 0 && dx < 0 {
            None
        } else {
            Some((x as i32 + dx) as usize)
        }
    }
    fn neighbors_index(img: &[Vec<Mode>], (i, j): (usize, usize), bg_mode: Mode) -> usize {
        let mut indices: Vec<Mode> = vec![];
        for di in [-1, 0, 1] {
            for dj in [-1, 0, 1] {
                let ii = Self::try_add(i, di);
                let jj = Self::try_add(j, dj);
                indices.push(if ii.is_none() || jj.is_none() {
                    bg_mode
                } else {
                    let ii = ii.unwrap();
                    let jj = jj.unwrap();
                    match img.get(ii) {
                        Some(row) => match row.get(jj) {
                            Some(col) => *col,
                            None => bg_mode,
                        },
                        None => bg_mode,
                    }
                })
            }
        }
        usize::from_str_radix(&indices.iter().map(|m| m.to_bin()).join(""), 2).unwrap()
    }
    fn step(img: &mut Vec<Vec<Mode>>, alg: &Vec<Mode>, stepno: usize) -> Vec<Vec<Mode>> {
        Self::enlarge_image(img, if stepno % 2 == 0 { Mode::Off } else { Mode::On });
        let mut new_img = vec![vec![Mode::Off; img[0].len()]; img.len()];
        for i in 0..img.len() {
            for j in 0..img[0].len() {
                new_img[i][j] = alg[Self::neighbors_index(
                    img,
                    (i, j),
                    if stepno % 2 == 0 { Mode::Off } else { Mode::On },
                )];
            }
        }
        new_img
    }
}

impl Problem for DayTwenty {
    fn part_one(&self, input: &str) -> String {
        let (algorithm, mut img) = Self::read_input(input);
        for i in 0..2 {
            img = Self::step(&mut img, &algorithm, i);
        }
        format!(
            "{}",
            img.iter().flatten().filter(|c| **c == Mode::On).count()
        )
    }

    fn part_two(&self, input: &str) -> String {
        let (algorithm, mut img) = Self::read_input(input);
        for i in 0..50 {
            img = Self::step(&mut img, &algorithm, i);
        }
        format!(
            "{}",
            img.iter().flatten().filter(|c| **c == Mode::On).count()
        )
    }
}

#[cfg(test)]
mod tests {
    
}
