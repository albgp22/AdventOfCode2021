use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    hash::{Hash, Hasher},
};

use crate::problem::problemdef::Problem;

pub struct DayTwentyThree {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
enum Position {
    Hall(usize),
    Room(usize, usize),
}
use Position::*;

impl Position {
    fn distance_to(&self, other: &Self, amphipod_type: usize) -> i32 {
        (match self {
            Hall(x) => match other {
                Hall(y) => {
                    let x = *x as i32;
                    let y = *y as i32;
                    (x - y).abs()
                }
                Room(rx, ry) => {
                    let x = *x as i32;
                    let ry = *ry as i32;
                    let room_hall_index = match rx {
                        0 => 2,
                        1 => 4,
                        2 => 6,
                        3 => 8,
                        _ => unreachable!(),
                    };
                    (room_hall_index - x).abs() + 2 - ry
                }
            },
            Room(x, y) => match other {
                Hall(x2) => {
                    let x2 = *x2 as i32;
                    let y = *y as i32;
                    let room_hall_index = match x {
                        0 => 2,
                        1 => 4,
                        2 => 6,
                        3 => 8,
                        _ => unreachable!(),
                    };
                    (room_hall_index - x2).abs() + 2 - y
                }
                Room(x2, y2) => {
                    let y = *y as i32;
                    let y2 = *y2 as i32;
                    let room1_hall_index = match x {
                        0 => 2i32,
                        1 => 4,
                        2 => 6,
                        3 => 8,
                        _ => unreachable!(),
                    };
                    let room2_hall_index = match x2 {
                        0 => 2i32,
                        1 => 4,
                        2 => 6,
                        3 => 8,
                        _ => unreachable!(),
                    };
                    (room1_hall_index - room2_hall_index).abs()
                        + if room1_hall_index != room2_hall_index {
                            4 - y - y2
                        } else {
                            (y - y2).abs()
                        }
                }
            },
        }) * match amphipod_type {
            0 | 1 => 1,
            2 | 3 => 10,
            4 | 5 => 100,
            6 | 7 => 1000,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
enum Type {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
    D1,
    D2,
    None,
}
use Type::*;

impl Type {
    fn get_position_index(&self) -> usize {
        match self {
            A1 => 0,
            A2 => 1,
            B1 => 2,
            B2 => 3,
            C1 => 4,
            C2 => 5,
            D1 => 6,
            D2 => 7,
            _ => 100,
        }
    }
    fn from_position_index(idx: usize) -> Self {
        match idx {
            0 => A1,
            1 => A2,
            2 => B1,
            3 => B2,
            4 => C1,
            5 => C2,
            6 => D1,
            7 => D2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct State {
    cost: i32,
    last_moved: Type,
    // A1,A2,B1,B2,C1,C2,D1,D2
    positions: Vec<Position>,
    already_moved: HashSet<usize>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cost.hash(state);
        self.positions.hash(state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn is_valid_state(&self) -> bool {
        (Room(0, 0) == self.positions[0] || Room(0, 1) == self.positions[0])
            && (Room(0, 0) == self.positions[1] || Room(0, 1) == self.positions[1])
            && (Room(1, 0) == self.positions[2] || Room(1, 1) == self.positions[2])
            && (Room(1, 0) == self.positions[3] || Room(1, 1) == self.positions[3])
            && (Room(2, 0) == self.positions[4] || Room(2, 1) == self.positions[4])
            && (Room(2, 0) == self.positions[5] || Room(2, 1) == self.positions[5])
            && (Room(3, 0) == self.positions[6] || Room(3, 1) == self.positions[6])
            && (Room(3, 0) == self.positions[7] || Room(3, 1) == self.positions[7])
    }
    fn is_position_reachable(&self, pos: &Position, amphipod_idx: usize) -> bool {
        match self.positions[amphipod_idx] {
            Room(x, y) => match pos {
                Room(x2, y2) => {
                    if x == *x2 {
                        return true;
                    }
                    let room1_hall_index = match x {
                        0 => 2i32,
                        1 => 4,
                        2 => 6,
                        3 => 8,
                        _ => unreachable!(),
                    };
                    let room2_hall_index = match x2 {
                        0 => 2i32,
                        1 => 4,
                        2 => 6,
                        3 => 8,
                        _ => unreachable!(),
                    };
                    let (a, b) = (
                        room1_hall_index.min(room2_hall_index) as usize,
                        room1_hall_index.max(room2_hall_index) as usize,
                    );
                    (a..=b).map(Hall).all(|h| !self.positions.contains(&h))
                        && (y == 1 || !self.positions.contains(&Room(x, 1)))
                        && (*y2 == 1 || !self.positions.contains(&Room(*x2, 1)))
                }
                Hall(x2) => {
                    let room1_hall_index = match x {
                        0 => 2i32,
                        1 => 4,
                        2 => 6,
                        3 => 8,
                        _ => unreachable!(),
                    };
                    let (a, b) = (
                        room1_hall_index.min(*x2 as i32) as usize,
                        room1_hall_index.max(*x2 as i32) as usize,
                    );
                    (if a == *x2 { ((a + 1)..(b + 1)) } else { (a..b) })
                        .map(Hall)
                        .all(|h| !self.positions.contains(&h))
                        && (y == 1 || !self.positions.contains(&Room(x, 1)))
                }
            },
            Hall(x) => match pos {
                Hall(x2) => {
                    let (a, b) = (
                        (x as i32).min(*x2 as i32) as usize + 1,
                        (x as i32).max(*x2 as i32) as usize,
                    );
                    (a..b).map(Hall).all(|h| !self.positions.contains(&h))
                }
                Room(x2, y2) => {
                    let room1_hall_index = match x2 {
                        0 => 2i32,
                        1 => 4,
                        2 => 6,
                        3 => 8,
                        _ => unreachable!(),
                    };
                    let (a, b) = (
                        room1_hall_index.min(x as i32) as usize,
                        room1_hall_index.max(x as i32) as usize,
                    );
                    (if a==x{a+1..b+1}else{a..b}).map(Hall).all(|h| !self.positions.contains(&h))
                        && (*y2 == 1 || !self.positions.contains(&Room(*x2, 1)))
                }
            },
        }
    }
    fn get_neighbors(&self) -> Vec<State> {
        let mut neighbors = Vec::new();
        for i in 0..self.positions.len() {
            if self.already_moved.contains(&i) {
                continue;
            }
            // Room to Hall, Room
            match self.positions[i] {
                Room(x, y) => {
                    let amphipod_type = i;
                    // TODO: Skip if last moved was the same amphipod
                    /* Get available Hall positions */
                    let available_positions =
                        (0..11usize).filter(|i| !self.positions.contains(&Hall(*i)));
                    /* For each of them, move the current amphipod to this location*/
                    for pos in available_positions {
                        // Todo: Skip if destination is unreachable
                        if !self.is_position_reachable(&Hall(pos), amphipod_type) {
                            continue;
                        }
                        let mut new_positions = self.positions.clone();
                        new_positions[i] = Hall(pos);
                        neighbors.push(State {
                            cost: self.cost
                                + self.positions[i].distance_to(&Hall(pos), amphipod_type),
                            last_moved: Type::from_position_index(amphipod_type),
                            positions: new_positions,
                            already_moved: self.already_moved.clone(),
                        });
                    }
                    // Get available room positions
                    let amphipod_type_index = i / 2;
                    // Only move to the bottom-most room
                    let available_positions = (0..=1)
                        .filter(|j| !self.positions.contains(&Room(amphipod_type_index, *j)))
                        .min();
                    if let Some(j) = available_positions {
                        if self.is_position_reachable(&Room(amphipod_type_index, j), i) {
                            let mut new_already_moved = self.already_moved.clone();
                            new_already_moved.insert(amphipod_type);
                            let mut new_positions = self.positions.clone();
                            new_positions[i] = Room(amphipod_type_index, j);
                            let amphipod_type = i;
                            neighbors.push(State {
                                cost: self.cost
                                    + self.positions[i]
                                        .distance_to(&Room(amphipod_type_index, j), amphipod_type),
                                last_moved: Type::from_position_index(amphipod_type),
                                positions: new_positions,
                                already_moved: new_already_moved,
                            });
                        }
                    }
                }
                // Hall to Room
                Hall(_) => {
                    // Amphipods can only move to a room of it's type.
                    let amphipod_type_index = i / 2;
                    // Only move to the bottom-most room
                    let available_positions = (0..2)
                        .filter(|j| !self.positions.contains(&Room(amphipod_type_index, *j)))
                        .min();
                    if let Some(j) = available_positions {
                        // Todo: Skip if destination is unreachable
                        if self.is_position_reachable(&Room(amphipod_type_index, j), i) {
                            let mut new_already_moved = self.already_moved.clone();
                            new_already_moved.insert(i);
                            let mut new_positions = self.positions.clone();
                            new_positions[i] = Room(amphipod_type_index, j);
                            let amphipod_type = i;
                            neighbors.push(State {
                                cost: self.cost
                                    + self.positions[i]
                                        .distance_to(&Room(amphipod_type_index, j), amphipod_type),
                                last_moved: Type::from_position_index(amphipod_type),
                                positions: new_positions,
                                already_moved: new_already_moved,
                            });
                        }
                    }
                }
            }
        }
        if neighbors.contains(self) {
            neighbors.remove(neighbors.iter().position(|n| n == self).unwrap());
        }
        neighbors
    }
}

impl DayTwentyThree {
    fn a_star(start_state: State) -> State {
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();
        pq.push(start_state);
        while !pq.is_empty() {
            let current_state = pq.pop().unwrap();
            if visited.contains(&current_state.positions) {
                continue;
            }
            visited.insert(current_state.positions.clone());
            if current_state.is_valid_state() {
                return current_state;
            }
            for neighbor in current_state.get_neighbors() {
                if !visited.contains(&neighbor.positions) {
                    pq.push(neighbor);
                }
            }
        }
        unreachable!("No valid state found")
    }
}

impl Problem for DayTwentyThree {
    fn part_one(&self, input: &str) -> String {
        format!(
            "{:?}",
            Self::a_star(State {
                cost: 0,
                last_moved: Type::None,
                positions: vec![
                    Room(2, 0),
                    Room(3, 1),
                    Room(0, 1),
                    Room(1, 1),
                    Room(1, 0),
                    Room(3, 0),
                    Room(0, 0),
                    Room(2, 1),
                ],
                already_moved: HashSet::new(),
            })
        )
    }

    fn part_two(&self, input: &str) -> String {
        format!("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(Room(0, 0).distance_to(&Room(0, 1), 0), 1);
        assert_eq!(Room(0, 0).distance_to(&Room(1, 1), 0), 5);
        assert_eq!(Room(0, 0).distance_to(&Room(1, 1), 2), 50);
        assert_eq!(Room(0, 0).distance_to(&Room(1, 1), 5), 500);
        assert_eq!(Room(0, 0).distance_to(&Room(1, 1), 6), 5000);
        assert_eq!(Room(0, 0).distance_to(&Room(3, 1), 6), 9000);
        assert_eq!(Hall(2).distance_to(&Room(0, 1), 0), 1);
        assert_eq!(Hall(2).distance_to(&Room(0, 0), 0), 2);
        assert_eq!(Room(0, 0).distance_to(&Hall(2), 0), 2);
        assert_eq!(Hall(3).distance_to(&Hall(0), 0), 3);
        assert_eq!(Hall(0).distance_to(&Hall(3), 0), 3);
        assert_eq!(Hall(0).distance_to(&Hall(3), 2), 30);
    }

    #[test]
    fn test_is_position_reachable() {
        let origin = State {
            cost: 0,
            last_moved: Type::A1,
            positions: vec![
                Room(0, 0),
                Room(1, 0),
                Room(2, 0),
                Room(3, 0),
                Hall(0),
                Hall(1),
                Hall(2),
                Hall(3),
            ],

            already_moved: HashSet::new(),
        };
        assert!(origin.is_position_reachable(&Room(0, 1), 0));
        assert!(!origin.is_position_reachable(&Room(2, 1), 0));
        assert!(!origin.is_position_reachable(&Hall(1), 0));
        assert!(origin.is_position_reachable(&Hall(2), 0));
        assert!(!origin.is_position_reachable(&Hall(2), 4));
        assert!(origin.is_position_reachable(&Hall(5), 7));
        assert!(origin.is_position_reachable(&Hall(4), 7));
        assert!(origin.is_position_reachable(&Hall(3), 7));
        assert!(origin.is_position_reachable(&Hall(2), 7));
        assert!(!origin.is_position_reachable(&Hall(1), 7));
        assert!(!origin.is_position_reachable(&Room(0, 1), 1));
    }

    #[test]
    fn test_get_neighbors() {
        let origin = State {
            cost: 0,
            last_moved: Type::A1,
            positions: vec![
                Room(0, 0),
                Room(1, 0),
                Room(2, 0),
                Room(3, 0),
                Hall(0),
                Hall(1),
                Hall(2),
                Hall(3),
            ],
            already_moved: HashSet::new(),
        };
        let expected = vec![
            State {
                cost: 1,
                last_moved: A1,
                positions: vec![
                    Room(0, 1),
                    Room(1, 0),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::from([0]),
            },
            State {
                cost: 2,
                last_moved: A2,
                positions: vec![
                    Room(0, 0),
                    Hall(4),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 3,
                last_moved: A2,
                positions: vec![
                    Room(0, 0),
                    Hall(5),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 4,
                last_moved: A2,
                positions: vec![
                    Room(0, 0),
                    Hall(6),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 5,
                last_moved: A2,
                positions: vec![
                    Room(0, 0),
                    Hall(7),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 6,
                last_moved: A2,
                positions: vec![
                    Room(0, 0),
                    Hall(8),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 7,
                last_moved: A2,
                positions: vec![
                    Room(0, 0),
                    Hall(9),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 8,
                last_moved: A2,
                positions: vec![
                    Room(0, 0),
                    Hall(10),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 40,
                last_moved: B1,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Hall(4),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 30,
                last_moved: B1,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Hall(5),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 20,
                last_moved: B1,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Hall(6),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 30,
                last_moved: B1,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Hall(7),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 40,
                last_moved: B1,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Hall(8),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 50,
                last_moved: B1,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Hall(9),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 60,
                last_moved: B1,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Hall(10),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 50,
                last_moved: B1,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(1, 1),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::from([2]),
            },
            State {
                cost: 60,
                last_moved: B2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Hall(4),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 50,
                last_moved: B2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Hall(5),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 40,
                last_moved: B2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Hall(6),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 30,
                last_moved: B2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Hall(7),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 20,
                last_moved: B2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Hall(8),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 30,
                last_moved: B2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Hall(9),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 40,
                last_moved: B2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Hall(10),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::new(),
            },
            State {
                cost: 70,
                last_moved: B2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Room(1, 1),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Hall(3),
                ],
                already_moved: HashSet::from([3]),
            },
            State {
                cost: 6000,
                last_moved: D2,
                positions: vec![
                    Room(0, 0),
                    Room(1, 0),
                    Room(2, 0),
                    Room(3, 0),
                    Hall(0),
                    Hall(1),
                    Hall(2),
                    Room(3, 1),
                ],
                already_moved: HashSet::from([7]),
            },
        ];

        println!("{:?}", origin.get_neighbors());
        assert_eq!(origin.get_neighbors(), expected);
    }
}
