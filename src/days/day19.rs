use crate::problem::problemdef::Problem;
use itertools::{iproduct, Itertools};
use num::{traits::Zero, Integer, Signed};
use std::ops::{Add, Neg, Sub};
use std::str::FromStr;

const MATCH_INDICATOR: usize = 66usize;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point<T>
where
    T: Clone + Signed + Integer + Zero + Neg + Copy + Eq + PartialEq,
{
    x: T,
    y: T,
    z: T,
}

fn abs<T>(x: T) -> T
where
    T: Signed + Integer + Zero + Neg + Copy,
{
    if x < T::zero() {
        -x
    } else {
        x
    }
}

impl<T> Point<T>
where
    T: Signed + Integer + Zero + Neg + Copy + Add,
{
    fn distance(&self, other: Self) -> T {
        abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl<T> FromStr for Point<T>
where
    T: Signed + Integer + Zero + Neg + Copy + FromStr,
{
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s.split(',').collect_tuple().ok_or(ParsePointError)?;

        let x_fromstr = x.parse::<T>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<T>().map_err(|_| ParsePointError)?;
        let z_fromstr = z.parse::<T>().map_err(|_| ParsePointError)?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
            z: z_fromstr,
        })
    }
}

pub struct DayNineteen {}

impl DayNineteen {
    fn calculate_distances<T>(
        pts1: impl IntoIterator<Item = Point<T>, IntoIter = ::std::vec::IntoIter<Point<T>>>,
        pts2: impl IntoIterator<Item = Point<T>, IntoIter = ::std::vec::IntoIter<Point<T>>>,
    ) -> Vec<T>
    where
        T: Signed + Integer + Zero + Neg + Copy + Add,
    {
        iproduct!(pts1, pts2)
            .map(|(pt1, pt2)| pt1.distance(pt2))
            .collect_vec()
    }

    fn matches<T>(
        distances1: impl IntoIterator<Item = T, IntoIter = ::std::vec::IntoIter<T>>,
        distances2: impl IntoIterator<Item = T, IntoIter = ::std::vec::IntoIter<T>>,
    ) -> bool
    where
        T: Eq + PartialEq + Clone,
    {
        iproduct!(distances1, distances2)
            .filter(|(d1, d2)| d1 == d2)
            .count()
            > MATCH_INDICATOR
    }
}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {
        format!("")
    }
    fn part_two(&self, input: &str) -> String {
        format!("")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use super::*;

    #[test]
    fn test_distance() {
        let a = Point { x: 0, y: 0, z: 1 };
        let b = Point { x: 1, y: 1, z: 3 };
        assert_eq!(a.distance(b), 4);
    }

    #[test]
    fn test_matches() {
        assert!(true)
    }

    #[test]
    fn test_from_str_point_i32() {
        assert_eq!(
            Point::<i32>::from_str("-3,2,4").unwrap(),
            Point { x: -3, y: 2, z: 4 }
        );
        assert_eq!(
            Point::<i128>::from_str("3,2,4").unwrap(),
            Point {
                x: 3_i128,
                y: 2_i128,
                z: 4_i128
            }
        );
    }
}
