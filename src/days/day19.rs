use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use log::{debug, error, log_enabled, info, Level};

use itertools::Itertools;

use crate::problem::problemdef::Problem;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s.split(',').tuples().next().ok_or(ParsePointError)?;

        let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;
        let z_fromstr = z.parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
            z: z_fromstr,
        })
    }
}

type Offset = Point;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash, Default)]
enum Facing {
    #[default]
    Forward,
    Backward,
}
use Facing::*;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash, Default)]
struct Orientation {
    rotx: i8,
    roty: i8,
    rotz: i8,
    facx: Facing,
    facy: Facing,
    facz: Facing,
}

#[allow(dead_code)]
fn get_all_orientations() -> Vec<Orientation> {
    let mut v = vec![];
    for rotx in 0..4 {
        for facx in vec![Backward, Forward] {
            v.push(Orientation {
                rotx: rotx,
                roty: 0,
                rotz: 0,
                facx: facx,
                facy: Forward,
                facz: Forward,
            })
        }
    }
    for roty in 0..4 {
        for facy in vec![Backward, Forward] {
            v.push(Orientation {
                rotx: 0,
                roty: roty,
                rotz: 0,
                facx: Forward,
                facy: facy,
                facz: Forward,
            })
        }
    }
    for rotz in 0..4 {
        for facz in vec![Backward, Forward] {
            v.push(Orientation {
                rotx: 0,
                roty: 0,
                rotz: rotz,
                facx: Forward,
                facy: Forward,
                facz: facz,
            })
        }
    }
    assert_eq!(v.len(), 24);
    v
}

static ALL_ORIENTATIONS: &'static [Orientation] = &[
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 0,
        facx: Backward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 1,
        roty: 0,
        rotz: 0,
        facx: Backward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 1,
        roty: 0,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 2,
        roty: 0,
        rotz: 0,
        facx: Backward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 2,
        roty: 0,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 3,
        roty: 0,
        rotz: 0,
        facx: Backward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 3,
        roty: 0,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 0,
        facx: Forward,
        facy: Backward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 1,
        rotz: 0,
        facx: Forward,
        facy: Backward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 1,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 2,
        rotz: 0,
        facx: Forward,
        facy: Backward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 2,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 3,
        rotz: 0,
        facx: Forward,
        facy: Backward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 3,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Backward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 0,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 1,
        facx: Forward,
        facy: Forward,
        facz: Backward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 1,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 2,
        facx: Forward,
        facy: Forward,
        facz: Backward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 2,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 3,
        facx: Forward,
        facy: Forward,
        facz: Backward,
    },
    Orientation {
        rotx: 0,
        roty: 0,
        rotz: 3,
        facx: Forward,
        facy: Forward,
        facz: Forward,
    },
];

#[derive(Debug, PartialEq, Eq, Clone)]
struct Scan {
    dots: Vec<Point>,
    distances: HashMap<(usize, usize), i32>,
}

impl Scan {
    fn get_all_distances(&self) -> Vec<i32> {
        self.distances.values().map(|i| *i).collect_vec()
    }
    fn overlaps_distances(&self, other: &Self) -> bool {
        let distances_a = self.get_all_distances();
        let distances_b = other.get_all_distances();
        let elements_a: HashSet<i32> = HashSet::from_iter(distances_a.iter().cloned());
        let count = |v: &Vec<i32>, e: i32| v.iter().filter(|&n| *n == e).count();

        let mut matches = 0;
        for e in elements_a {
            matches += count(&distances_a, e).min(count(&distances_b, e));
        }

        matches >= 66
    }
    fn rotate(&self, o: &Orientation) -> Scan {
        let my_cos = |k| match k {
            0 => 1,
            1 => 0,
            2 => -1,
            3 => 0,
            _ => unreachable!(),
        };
        let my_sin = |k| match k {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => unreachable!(),
        };

        let rotate_x = |p: &Point, k| Point {
            x: p.x,
            y: p.y * my_cos(k) - p.z * my_sin(k),
            z: p.y * my_sin(k) + p.z * my_cos(k),
        };
        let rotate_y = |p: Point, k| Point {
            x: p.x * my_cos(k) + p.z * my_sin(k),
            y: p.y,
            z: p.z * my_cos(k) - p.x * my_sin(k),
        };
        let rotate_z = |p: Point, k| Point {
            x: p.x * my_cos(k) - p.y * my_sin(k),
            y: p.x * my_sin(k) + p.y * my_cos(k),
            z: p.z,
        };

        let mut new_points = self
            .dots
            .iter()
            .map(|p| rotate_x(p, o.rotx))
            .map(|p| rotate_y(p, o.roty))
            .map(|p| rotate_z(p, o.rotz))
            .collect_vec();

        for p in new_points.iter_mut() {
            p.x = p.x * if o.facx == Forward { 1 } else { -1 };
            p.y = p.y * if o.facy == Forward { 1 } else { -1 };
            p.z = p.z * if o.facz == Forward { 1 } else { -1 };
        }

        Scan {
            dots: new_points,
            distances: self.distances.clone(),
        }
    }
}

pub struct DayNineteen {}

impl DayNineteen {
    fn read_input(input: &str) -> Vec<Scan> {
        let mut v_curr = vec![];
        let mut r = vec![];
        let mut it = input.split('\n').filter(|l| !l.is_empty()).peekable();
        while let Some(line) = it.next() {
            if line.contains("---") {
                if v_curr.is_empty() {
                    continue;
                }
                let distances = Self::compute_manhattan_distances(&v_curr);
                r.push(Scan {
                    dots: v_curr,
                    distances: distances,
                });
                v_curr = vec![];
            } else if it.peek().is_none() {
                v_curr.push(Point::from_str(line).unwrap());
                let distances = Self::compute_manhattan_distances(&v_curr);
                r.push(Scan {
                    dots: v_curr,
                    distances: distances,
                });
                v_curr = vec![];
            } else {
                v_curr.push(Point::from_str(line).unwrap());
            }
        }
        r
    }
    fn compute_manhattan_distances(ps: &Vec<Point>) -> HashMap<(usize, usize), i32> {
        // If 12 points match then at least 12*11/2=66 distances would be the same
        let mut r = HashMap::new();
        for (i, p1) in ps.iter().enumerate() {
            for (j, p2) in ps.iter().skip(i).enumerate() {
                r.insert(
                    (i, j),
                    (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs(),
                );
            }
        }
        r
    }
    fn overlapp(_idx1: usize, _idx2: usize, s0: &Scan, s1: &Scan) -> Option<(Orientation, Offset)> {
        if s0.overlaps_distances(s1) {
            info!("Overlapp detected between sensors {} and {}", _idx1, _idx2);
            for orientation in ALL_ORIENTATIONS {
                let s1_oriented = s1.rotate(&orientation);
                let mut distances = HashMap::new();
                let compute_distance = |p1: &Point, p2: &Point| {
                    (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs()
                };
                for p1 in &s0.dots {
                    for p2 in &s1_oriented.dots {
                        let distance = compute_distance(&p1, &p2);
                        *distances.entry(distance).or_insert(0) += 1;
                    }
                }
                if distances.values().any(|&x| x>=12){
                    info!("Orientation found between sensors {} and {}: {:?}", _idx1, _idx2, orientation);
                    return Some((orientation.clone(),Offset::default()));
                }
                //println!("{:?}", distances);
            }
            //println!("overlapp detected between {} and {}!",idx1,idx2);
        }
        Some((Orientation::default(), Offset { x: 0, y: 0, z: 0 }))
    }
    fn reconstruct(
        scans: &Vec<Scan>,
        orientations: &HashMap<(usize, usize), (Orientation, Offset)>,
    ) -> HashSet<Point> {
        HashSet::new()
    }
}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {
        let scans = Self::read_input(input);
        let n_sensors = scans.len();
        let mut overlaps = HashMap::new();
        for i in 0..n_sensors {
            for j in i + 1..n_sensors {
                /* Idea:
                you dont't really need to run this particular pair if there's already a path i--j
                 */
                let r = Self::overlapp(i, j, &scans[i], &scans[j]);
                if r.is_some() {
                    overlaps.insert((i, j), r.unwrap());
                }
            }
        }
        let reconstructed_map = Self::reconstruct(&scans, &overlaps);

        format!("{}", reconstructed_map.len())
    }

    fn part_two(&self, input: &str) -> String {
        format!("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
