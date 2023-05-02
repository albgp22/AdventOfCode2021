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

fn room_to_index(x: usize) -> i32 {
    match x {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => unreachable!(),
    }
}

impl Position {
    fn distance_to(&self, other: &Self, amphipod_type: usize) -> i32 {
        let steps = match (self, other) {
            (Hall(x), Hall(y)) => (*x as i32 - *y as i32).abs(),
            (Hall(x), Room(x2, y2)) | (Room(x2, y2), Hall(x)) => {
                let room_col = room_to_index(*x2);
                (room_col - *x as i32).abs() + 1 + *y2 as i32
            }
            (Room(x, y), Room(x2, y2)) => {
                let room_col = room_to_index(*x);
                let room2_col = room_to_index(*x2);
                let (y, y2) = (*y as i32, *y2 as i32);
                (room_col - room2_col).abs()
                    + if room_col != room2_col {
                        2 + y + y2
                    } else {
                        (y - y2).abs()
                    }
            }
        };
        let mult_factor = 10i32.pow(amphipod_type as u32 / 2);
        steps.checked_mul(mult_factor).unwrap()
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
    fn is_valid_state(&self, room_depth: usize) -> bool {
        self.positions.iter().enumerate().all(|(i, p)| {
            if let Room(x, _y) = p {
                *x == i / room_depth
            } else {
                false
            }
        })
    }
    fn is_position_reachable(&self, pos: &Position, amphipod_idx: usize) -> bool {
        match (&self.positions[amphipod_idx], pos) {
            (Hall(x), Hall(y)) => (x.min(y) + 1..*x.max(y))
                .map(Hall)
                .all(|h| !self.positions.contains(&h)),
            (Hall(x), Room(x2, y2)) | (Room(x2, y2), Hall(x)) => {
                let room_col = room_to_index(*x2) as usize;
                let (x, x2) = (*x, *x2);
                let y2 = *y2;
                let hall_clear = if room_col < x {
                    (room_col..x)
                        .map(Hall)
                        .all(|h| !self.positions.contains(&h))
                } else {
                    (x + 1..=room_col)
                        .map(Hall)
                        .all(|h| !self.positions.contains(&h))
                };
                let col_clear = (0..y2)
                    .map(|r| Room(x2, r))
                    .all(|r| !self.positions.contains(&r));
                hall_clear && col_clear
            }
            (Room(x, y), Room(x2, y2)) => {
                let room_col1 = room_to_index(*x) as usize;
                let room_col2 = room_to_index(*x2) as usize;
                let (x, x2) = (*x, *x2);
                let (y, y2) = (*y, *y2);
                if x == x2 {
                    (y.min(y2) + 1..y.max(y2))
                        .map(|r| Room(x, r))
                        .all(|r| !self.positions.contains(&r))
                } else {
                    let hall_clear = (room_col1.min(room_col2)..=room_col1.max(room_col2))
                        .map(Hall)
                        .all(|h| !self.positions.contains(&h));
                    let col1_clear = (0..y)
                        .map(|r| Room(x, r))
                        .all(|r| !self.positions.contains(&r));
                    let col2_clear = (0..y2)
                        .map(|r| Room(x2, r))
                        .all(|r| !self.positions.contains(&r));
                    col1_clear && col2_clear && hall_clear
                }
            }
        }
    }

    // TODO: Refactor to new depth
    fn get_neighbors(&self) -> Vec<State> {
        let room_depth = 2;
        let mut neighbors = Vec::new();
        for i in 0..self.positions.len() {
            if self.already_moved.contains(&i) {
                continue;
            }
            // Room to Hall, Room
            match self.positions[i] {
                Room(x, y) => {
                    // TODO: Skip if last moved was the same amphipod
                    /* Get available Hall positions */
                    let available_hall_positions =
                        (0..11usize).filter(|i| !self.positions.contains(&Hall(*i)));
                    /* For each of them, move the current amphipod to this location*/
                    for pos in available_hall_positions {
                        // Todo: Skip if destination is unreachable
                        if !self.is_position_reachable(&Hall(pos), i) {
                            continue;
                        }
                        let mut new_positions = self.positions.clone();
                        new_positions[i] = Hall(pos);
                        neighbors.push(State {
                            cost: self.cost
                                + self.positions[i].distance_to(&Hall(pos), i),
                            last_moved: Type::from_position_index(i),
                            positions: new_positions,
                            already_moved: self.already_moved.clone(),
                        });
                    }
                    // Get available room positions
                    let amphipod_type_index = i / room_depth;
                    // Only move to the bottom-most room
                    let available_positions = (0..room_depth)
                        .filter(|j| !self.positions.contains(&Room(amphipod_type_index, *j)))
                        .max();
                    if let Some(j) = available_positions {
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
                // Hall to Room
                Hall(_) => {
                    // Amphipods can only move to a room of it's type.
                    let amphipod_type_index = i / room_depth;
                    // Only move to the bottom-most room
                    let available_positions = (0..room_depth)
                        .filter(|j| !self.positions.contains(&Room(amphipod_type_index, *j)))
                        .max();
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
    fn a_star(start_state: State, room_depth: usize) -> State {
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();
        pq.push(start_state);
        while !pq.is_empty() {
            let current_state = pq.pop().unwrap();
            if visited.contains(&current_state.positions) {
                continue;
            }
            visited.insert(current_state.positions.clone());
            if current_state.is_valid_state(room_depth) {
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
            Self::a_star(
                State {
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
                },
                2
            )
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
        assert_eq!(Hall(2).distance_to(&Room(0, 1), 0), 2);
        assert_eq!(Hall(2).distance_to(&Room(0, 0), 0), 1);
        assert_eq!(Room(0, 0).distance_to(&Hall(2), 0), 1);
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
        println!("{:?}", origin.get_neighbors());
        assert!(false);
    }
}
