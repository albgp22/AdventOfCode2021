use crate::problem::problemdef::Problem;
use itertools::Itertools;

pub struct DayTwentyTwo {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum ActionType {
    On,
    Off,
}

impl ActionType {
    fn opposite(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
    fn multiplicative_factor(&self) -> i128 {
        match self {
            Self::On => 1,
            Self::Off => -1,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Interval {
    start: i32,
    end: i32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Cube {
    at: ActionType,
    coords: Vec<Interval>,
}

impl Cube {
    fn intersect(&self, other: &Cube) -> Option<Cube> {
        let candidate = Cube {
            at: other.at.opposite(),
            coords: vec![
                Interval {
                    start: self.coords[0].start.max(other.coords[0].start),
                    end: self.coords[0].end.min(other.coords[0].end),
                },
                Interval {
                    start: self.coords[1].start.max(other.coords[1].start),
                    end: self.coords[1].end.min(other.coords[1].end),
                },
                Interval {
                    start: self.coords[2].start.max(other.coords[2].start),
                    end: self.coords[2].end.min(other.coords[2].end),
                },
            ],
        };
        if candidate.coords[0].start <= candidate.coords[0].end
            && candidate.coords[1].start <= candidate.coords[1].end
            && candidate.coords[2].start <= candidate.coords[2].end
        {
            Some(candidate)
        } else {
            None
        }
    }

    fn is_part_1(&self) -> bool {
        self.coords
            .iter()
            .flat_map(|cc| vec![cc.start, cc.end])
            .all(|c| c.abs() <= 50)
    }
}

impl DayTwentyTwo {
    fn parse_input(input: &str) -> Vec<Cube> {
        let mut ret = vec![];
        for line in input.split('\n').filter(|l| !l.is_empty()) {
            let mut lineparts = line.split(' ');
            let at = match lineparts.next().unwrap() {
                "on" => ActionType::On,
                "off" => ActionType::Off,
                _ => unreachable!(),
            };
            let mut coords = vec![];
            for coord in lineparts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.trim().replace(['x', 'y', 'z', '='], ""))
            {
                let (x, y) = coord
                    .split("..")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                coords.push(Interval { start: x, end: y });
            }
            ret.push(Cube {
                at,
                coords,
            });
        }
        ret
    }

    fn count(cs: Vec<Cube>) -> i128 {
        cs.iter()
            .map(|c| {
                let mut ret = 1i128;
                for coord in &c.coords {
                    ret = ret
                        .checked_mul((coord.end - coord.start + 1) as i128)
                        .unwrap();
                }
                ret * c.at.multiplicative_factor()
            })
            .fold(0, |acc, x| acc.checked_add(x).unwrap())
    }

    fn solve(c: Vec<Cube>) -> String {
        let mut previous = vec![];
        for cube in c {
            let mut add = if cube.at == ActionType::On {
                vec![cube.clone()]
            } else {
                vec![]
            };
            for p in &previous {
                if let Some(intersection) = cube.intersect(p) {
                    add.push(intersection);
                }
            }
            previous.append(&mut add);
        }
        let res = Self::count(previous);
        format!("{res}")
    }
}

impl Problem for DayTwentyTwo {
    fn part_one(&self, input: &str) -> String {
        let cubes = Self::parse_input(input)
            .iter()
            .filter(|&c| c.is_part_1())
            .cloned()
            .collect();
        Self::solve(cubes)
    }

    fn part_two(&self, input: &str) -> String {
        Self::solve(Self::parse_input(input))
    }
}

#[cfg(test)]
mod tests {
    
}
