use crate::problem::problemdef::Problem;
use array_tool::vec::Intersect;
use itertools::{iproduct, Itertools};
use num::{traits::Zero, Integer, Signed};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Add, Neg, Sub};
use std::str::FromStr;

const MATCH_INDICATOR: usize = 66usize;
const MIN_MATCHES: usize = 12usize;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point<T>
where
    T: Clone + Signed + Integer + Zero + Neg + Copy + Eq + PartialEq + Hash,
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

impl<T> Add for Point<T>
where
    T: Clone + Signed + Integer + Zero + Neg + Copy + Eq + PartialEq + Sub + Hash,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Clone + Signed + Integer + Zero + Neg + Copy + Eq + PartialEq + Sub + Hash,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Point<T>
where
    T: Signed + Integer + Zero + Neg + Copy + Add + Clone + Hash,
{
    fn distance(&self, other: &Self) -> T {
        abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)
    }
    fn rotate(&self, rot_idx: usize) -> Self {
        let (x, y, z) = (self.x, self.y, self.z);
        let (newx, newy, newz) = match rot_idx {
            0 => [x, y, z],
            1 => [x, z, -y],
            2 => [x, -y, -z],
            3 => [x, -z, y],
            4 => [y, x, -z],
            5 => [y, z, x],
            6 => [y, -x, z],
            7 => [y, -z, -x],
            8 => [z, x, y],
            9 => [z, y, -x],
            10 => [z, -x, -y],
            11 => [z, -y, x],
            12 => [-x, y, -z],
            13 => [-x, z, y],
            14 => [-x, -y, z],
            15 => [-x, -z, -y],
            16 => [-y, x, z],
            17 => [-y, z, -x],
            18 => [-y, -x, -z],
            19 => [-y, -z, x],
            20 => [-z, x, -y],
            21 => [-z, y, x],
            22 => [-z, -x, y],
            23 => [-z, -y, -x],
            _ => unreachable!(),
        }
        .iter()
        .cloned()
        .collect_tuple()
        .unwrap();

        Point {
            x: newx,
            y: newy,
            z: newz,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl<T> FromStr for Point<T>
where
    T: Signed + Integer + Zero + Neg + Copy + FromStr + Hash,
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
        pts: impl IntoIterator<Item = Point<T>, IntoIter = ::std::vec::IntoIter<Point<T>>>,
    ) -> Vec<T>
    where
        T: Signed + Integer + Zero + Neg + Copy + Add + Hash,
    {
        let pts = pts.into_iter().collect::<Vec<_>>();
        iproduct!(&pts, &pts)
            .filter(|(p1, p2)| p1 != p2)
            .map(|(p1, p2)| p1.distance(p2))
            .collect()
    }

    fn matches<T>(
        distances1: impl IntoIterator<Item = T, IntoIter = ::std::vec::IntoIter<T>>,
        distances2: impl IntoIterator<Item = T, IntoIter = ::std::vec::IntoIter<T>>,
    ) -> bool
    where
        T: Eq + PartialEq + Clone,
    {
        distances1
            .into_iter()
            .collect_vec()
            .intersect(distances2.into_iter().collect_vec())
            .len()
            >= MATCH_INDICATOR
    }

    fn find_rotation<T>(sensor1: &[Point<T>], sensor2: &[Point<T>]) -> Option<(Point<T>, usize)>
    where
        T: Signed + Integer + Zero + Neg + Copy + Add + Clone + Eq + PartialEq + Hash,
    {
        for rot_idx in 0..24 {
            let rotated_sensor2 = sensor2
                .iter()
                .map(|p| p.rotate(rot_idx))
                .collect::<Vec<_>>();

            let mut m: HashMap<Point<T>, usize> = HashMap::new();
            for d in iproduct!(sensor1.iter().cloned(), rotated_sensor2.iter())
                .map(|(p1, p2)| p1 - p2.clone())
            {
                *m.entry(d).or_default() += 1;
            }
            let (most_frequent_distance, num_ocurrences) =
                m.into_iter().max_by_key(|(_, v)| *v).unwrap();
            if num_ocurrences >= MIN_MATCHES {
                return Some((most_frequent_distance, rot_idx));
            }
        }
        None
    }

    fn read_input<T>(input: &str) -> Vec<Vec<Point<T>>>
    where
        T: Signed + Integer + Zero + Neg + Copy + FromStr + Hash,
    {
        let lines = input.lines().filter(|l| !l.is_empty()).map(|l| l.trim());
        let mut sensors = Vec::new();
        let mut current_sensor = Vec::new();
        for line in lines {
            if line.contains("---") {
                if current_sensor.is_empty() {
                    continue;
                }
                sensors.push(current_sensor);
                current_sensor = Vec::new();
            } else {
                let point = Point::<T>::from_str(line).unwrap();
                current_sensor.push(point);
            }
        }
        sensors
    }

    fn dfs(
        links: &[(usize, usize)],
        visited: &mut HashSet<usize>,
        currpath: &mut Vec<usize>,
        current: usize,
        destination: usize,
    ) -> Option<Vec<usize>> {
        if current == destination {
            return Some(currpath.clone());
        }
        visited.insert(current);
        for (i, j) in links
            .iter()
            .cloned()
            .filter(|(i, j)| *i == current || *j == current)
        {
            // i is the current node, j is the next node
            let (i, j) = if i == current { (i, j) } else { (j, i) };
            if visited.contains(&j) {
                continue;
            }
            currpath.push(j);
            if let Some(result) = Self::dfs(links, visited, currpath, j, destination) {
                return Some(result);
            }
            currpath.pop();
        }
        None
    }

    fn find_path(links: &[(usize, usize)], i: usize, j: usize) -> Vec<usize> {
        Self::dfs(links, &mut HashSet::new(), &mut vec![0], i, j).unwrap()
    }
}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {
        let sensors = Self::read_input::<i32>(input);
        assert_eq!(sensors.len(), 33);
        let mut rotation_offset = HashMap::new();
        let mut links = vec![];
        for i in 0..sensors.len() {
            for j in 0..sensors.len() {
                if (i == j) {
                    continue;
                }
                let distances1 = Self::calculate_distances(sensors[i].clone());
                let distances2 = Self::calculate_distances(sensors[j].clone());
                if Self::matches(distances1, distances2) {
                    let (offset, rotation) = Self::find_rotation(&sensors[i], &sensors[j]).unwrap();
                    rotation_offset.insert((i, j), (rotation, offset.clone()));
                    links.push((i, j));
                    assert!(
                        sensors[j]
                            .iter()
                            .map(|p| p.rotate(rotation) + offset.clone())
                            .collect::<Vec<_>>()
                            .intersect(sensors[i].iter().cloned().collect::<Vec<_>>())
                            .len()
                            >= MIN_MATCHES
                    );
                }
            }
        }
        let mut result = HashSet::new();
        sensors[0].iter().for_each(|p| {
            result.insert(p.clone());
        });
        for i in 1..sensors.len() {
            let path = Self::find_path(&links, 0, i);
            let mut new_sensors = sensors[i].clone();
            for (ii, jj) in path.iter().rev().tuple_windows() {
                //
                let (rotation, offset) = rotation_offset.get(&(*jj, *ii)).unwrap();
                new_sensors = new_sensors
                    .iter()
                    .map(|p| p.rotate(*rotation) + offset.clone())
                    .collect::<Vec<_>>();
            }
            new_sensors.iter().for_each(|p| {
                result.insert(p.clone());
            });
        }

        format!("{}", result.len())
    }
    fn part_two(&self, input: &str) -> String {
        let sensors = Self::read_input::<i32>(input);
        assert_eq!(sensors.len(), 33);
        let mut rotation_offset = HashMap::new();
        let mut links = vec![];
        for i in 0..sensors.len() {
            for j in 0..sensors.len() {
                if (i == j) {
                    continue;
                }
                let distances1 = Self::calculate_distances(sensors[i].clone());
                let distances2 = Self::calculate_distances(sensors[j].clone());
                if Self::matches(distances1, distances2) {
                    let (offset, rotation) = Self::find_rotation(&sensors[i], &sensors[j]).unwrap();
                    rotation_offset.insert((i, j), (rotation, offset.clone()));
                    links.push((i, j));
                    assert!(
                        sensors[j]
                            .iter()
                            .map(|p| p.rotate(rotation) + offset.clone())
                            .collect::<Vec<_>>()
                            .intersect(sensors[i].to_vec())
                            .len()
                            >= MIN_MATCHES
                    );
                }
            }
        }
        let mut ssensors = vec![];
        for i in 1..sensors.len() {
            let path = Self::find_path(&links, 0, i);
            let mut new_sensors = vec![Point { x: 0, y: 0, z: 0 }];
            for (ii, jj) in path.iter().rev().tuple_windows() {
                //
                let (rotation, offset) = rotation_offset.get(&(*jj, *ii)).unwrap();
                new_sensors = new_sensors
                    .iter()
                    .map(|p| p.rotate(*rotation) + offset.clone())
                    .collect::<Vec<_>>();
            }
            ssensors.push(new_sensors[0].clone());
        }
        ssensors.push(Point { x: 0, y: 0, z: 0 });
        ssensors
            .iter()
            .flat_map(|s1| ssensors.iter().map(|s2| s1.distance(s2)).collect_vec())
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = Point { x: 0, y: 0, z: 1 };
        let b = Point { x: 1, y: 1, z: 3 };
        assert_eq!(a.distance(&b), 4);
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

    #[test]
    fn sample() {
        let input = "
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        let sensors = DayNineteen::read_input::<i32>(input);
        for i in 0..sensors.len() {
            for j in 0..i {
                let distances1 = DayNineteen::calculate_distances(sensors[i].clone());
                let distances2 = DayNineteen::calculate_distances(sensors[j].clone());
                if DayNineteen::matches(distances1, distances2) {
                    let (offset, rotation) =
                        DayNineteen::find_rotation(&sensors[i], &sensors[j]).unwrap();
                    println!(
                        "{} and {} match. Rotation: {:?}, Offset: {:?}",
                        i, j, rotation, offset
                    );
                    println!(
                        "{}",
                        sensors[j]
                            .iter()
                            .map(|p| p.rotate(rotation) + offset.clone())
                            .collect::<Vec<_>>()
                            .intersect(sensors[i].iter().cloned().collect::<Vec<_>>())
                            .len()
                    );
                }
            }
        }
        assert!(false)
    }
}
