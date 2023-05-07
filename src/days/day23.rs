use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};

use crate::problem::problemdef::Problem;

use hashbrown::HashMap;
use itertools::Itertools;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Amphipod {
    fn energy(&self) -> usize {
        10usize.pow(*self as u32)
    }

    fn target_room_index(&self) -> usize {
        *self as usize
    }

    fn from_room_index(room_index: usize) -> Self {
        assert!(room_index < 4);
        unsafe { std::mem::transmute(room_index as u8) }
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State<const R: usize> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; R]; 4],
}

impl<const R: usize> State<R> {
    fn encode(&self) -> u64 {
        self.rooms
            .iter()
            .flatten()
            .rev()
            .chain(self.hallway.iter().rev())
            .map(|s| match *s {
                None => 0,
                Some(amphipod) => amphipod.target_room_index() as u64 + 1,
            })
            .fold(0, |encoded, encoded_space| encoded * 5 + encoded_space)
    }

    fn decode(mut encoded: u64) -> Self {
        fn decode_space(encoded_space: u64) -> Option<Amphipod> {
            match encoded_space {
                0 => None,
                1 | 2 | 3 | 4 => Some(Amphipod::from_room_index((encoded_space - 1) as usize)),
                _ => unreachable!(),
            }
        }

        let mut it = std::iter::from_fn(move || {
            let encoded_space = encoded % 5;
            encoded /= 5;
            Some(decode_space(encoded_space))
        });

        Self {
            hallway: [(); 11].map(|_| it.next().unwrap()),
            rooms: [(); 4].map(|_| [(); R].map(|_| it.next().unwrap())),
        }
    }

    fn goal() -> Self {
        Self {
            hallway: [None; 11],
            rooms: [
                [Some(Amphipod::A); R],
                [Some(Amphipod::B); R],
                [Some(Amphipod::C); R],
                [Some(Amphipod::D); R],
            ],
        }
    }

    fn is_room_enterable(&self, room_index: usize) -> bool {
        self.rooms[room_index].iter().all(|space| match space {
            None => true,
            Some(amphipod) => amphipod.target_room_index() == room_index,
        })
    }

    fn is_room_exitable(&self, room_index: usize) -> bool {
        !self.is_room_enterable(room_index)
    }

    fn room_x(&self, room_index: usize) -> usize {
        2 + (room_index) * 2
    }

    fn is_above_room(&self, x: usize) -> bool {
        (x - 2) % 2 == 0 && (x - 2) / 2 < self.rooms.len()
    }

    fn is_hallway_clear(&self, start_x: usize, target_x: usize) -> bool {
        let slice = match start_x.cmp(&target_x) {
            Ordering::Equal => {
                return true;
            }
            Ordering::Less => &self.hallway[(start_x + 1)..=target_x],
            Ordering::Greater => &self.hallway[target_x..start_x],
        };

        slice.iter().all(|space| space.is_none())
    }

    fn iter_empty_spaces(&self, start_x: usize) -> impl Iterator<Item = usize> + '_ {
        let left_it = (0..start_x)
            .rev()
            .take_while(|x| self.hallway[*x].is_none());
        let right_it =
            ((start_x + 1)..self.hallway.len()).take_while(|x| self.hallway[*x].is_none());
        left_it.chain(right_it)
    }

    fn transitions(&self) -> Vec<(State<R>, usize)> {
        let mut transitions = self.room_to_hallway_transitions();
        transitions.extend(self.hallway_to_room_transitions().into_iter());
        transitions
    }

    fn room_to_hallway_transitions(&self) -> Vec<(State<R>, usize)> {
        self.rooms
            .iter()
            .enumerate()
            .filter(|(room_index, _)| self.is_room_exitable(*room_index))
            .flat_map(|(room_index, room)| {
                let (room_depth, amphipod) = room
                    .iter()
                    .enumerate()
                    .find_map(|(room_depth, space)| space.map(|amphipod| (room_depth, amphipod)))
                    .unwrap();

                let current_x = self.room_x(room_index);

                self.iter_empty_spaces(current_x)
                    .filter(|target_x| !self.is_above_room(*target_x))
                    .map(move |target_x| {
                        let steps = room_depth + 1 + abs_diff(current_x, target_x);
                        let energy = steps * amphipod.energy();

                        let mut state = *self;
                        std::mem::swap(
                            &mut state.rooms[room_index][room_depth],
                            &mut state.hallway[target_x],
                        );
                        (state, energy)
                    })
            })
            .collect()
    }

    fn hallway_to_room_transitions(&self) -> Vec<(State<R>, usize)> {
        self.hallway
            .iter()
            .enumerate()
            .filter_map(|(current_x, space)| space.map(|amphipod| (current_x, amphipod)))
            .filter_map(|(current_x, amphipod)| {
                let target_room_index = amphipod.target_room_index();

                if !self.is_room_enterable(target_room_index) {
                    return None;
                }

                let target_x = self.room_x(target_room_index);

                if !self.is_hallway_clear(current_x, target_x) {
                    return None;
                }

                let target_room_depth = self.rooms[target_room_index]
                    .iter()
                    .rposition(|space| space.is_none())
                    .unwrap();

                let steps = target_room_depth + 1 + abs_diff(current_x, target_x);
                let energy = steps * amphipod.energy();

                let mut state = *self;
                std::mem::swap(
                    &mut state.rooms[target_room_index][target_room_depth],
                    &mut state.hallway[current_x],
                );

                Some((state, energy))
            })
            .collect()
    }

    fn h_score(&self) -> usize {
        let exit_room = self
            .rooms
            .iter()
            .enumerate()
            .flat_map(|(room_index, room)| {
                let current_x = self.room_x(room_index);

                room.iter()
                    .enumerate()
                    .rev()
                    .filter_map(|(room_depth, space)| space.map(|amphipod| (room_depth, amphipod)))
                    .skip_while(move |(_, amphipod)| amphipod.target_room_index() == room_index)
                    .map(move |(room_depth, amphipod)| {
                        let target_room_index = amphipod.target_room_index();
                        let target_x = self.room_x(target_room_index);

                        let hallway_steps = abs_diff(current_x, target_x).max(2);
                        let steps = room_depth + 1 + hallway_steps;

                        steps * amphipod.energy()
                    })
            })
            .sum::<usize>();

        let move_hallway = self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(current_x, space)| space.map(|amphipod| (current_x, amphipod)))
            .map(|(current_x, amphipod)| {
                let target_room_index = amphipod.target_room_index();
                let target_x = self.room_x(target_room_index);
                let steps = abs_diff(current_x, target_x);

                steps * amphipod.energy()
            })
            .sum::<usize>();

        let enter_room = self
            .rooms
            .iter()
            .enumerate()
            .flat_map(|(room_index, room)| {
                room.iter()
                    .enumerate()
                    .rev()
                    .skip_while(move |(_, space)| {
                        if let Some(amphipod) = space {
                            amphipod.target_room_index() == room_index
                        } else {
                            false
                        }
                    })
                    .map(move |(room_depth, _)| {
                        let target_amphipod = Amphipod::from_room_index(room_index);
                        let steps = room_depth + 1;

                        steps * target_amphipod.energy()
                    })
            })
            .sum::<usize>();

        exit_room + move_hallway + enter_room
    }
}

impl<const R: usize> Display for State<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let space_to_str = |space: Option<Amphipod>| -> &str {
            match space {
                None => ".",
                Some(Amphipod::A) => "A",
                Some(Amphipod::B) => "B",
                Some(Amphipod::C) => "C",
                Some(Amphipod::D) => "D",
            }
        };

        writeln!(f, "{}", "#".repeat(self.hallway.len() + 2))?;
        writeln!(f, "#{}#", self.hallway.map(space_to_str).join(""))?;
        writeln!(
            f,
            "###{}###",
            self.rooms.map(|r| space_to_str(r[0])).join("#")
        )?;
        for room_depth in 1..R {
            writeln!(
                f,
                "  #{}#  ",
                self.rooms.map(|r| space_to_str(r[room_depth])).join("#")
            )?;
        }
        write!(f, "  {}  ", "#".repeat(self.rooms.len() * 2 + 1))?;

        Ok(())
    }
}

pub fn parse_input(input: &str) -> Vec<Option<Amphipod>> {
    let input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.replace(['#', '.'], "").trim().to_owned())
        .collect_vec();
    let input = input.join("");
    let amphipods = input
        .chars()
        .filter_map(|c| match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        })
        .map(Some)
        .collect::<Vec<_>>();

    assert_eq!(amphipods.len(), 8);

    amphipods
}

#[derive(PartialEq, Eq)]
struct Entry {
    encoded_state: u64,
    f_score: usize,
}

impl PartialOrd<Self> for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

fn solve<const R: usize>(initial_state: State<R>) -> usize {
    let mut q = BinaryHeap::new();
    q.push(Entry {
        encoded_state: initial_state.encode(),
        f_score: 0,
    });

    let mut g_score: HashMap<u64, usize> = HashMap::new();
    g_score.insert(initial_state.encode(), 0);

    let encoded_goal_state = State::<R>::goal().encode();

    while let Some(Entry {
        encoded_state,
        f_score,
    }) = q.pop()
    {
        if encoded_state == encoded_goal_state {
            return f_score;
        }

        let current_state = State::<R>::decode(encoded_state);
        let current_g_score = g_score[&encoded_state];

        for (next_state, transition_cost) in current_state.transitions() {
            let encoded_next_state = next_state.encode();
            let tentative_g_score = current_g_score + transition_cost;
            if tentative_g_score < *g_score.get(&encoded_next_state).unwrap_or(&usize::MAX) {
                g_score.insert(encoded_next_state, tentative_g_score);
                q.push(Entry {
                    encoded_state: encoded_next_state,
                    f_score: tentative_g_score + next_state.h_score(),
                });
            }
        }
    }

    unreachable!();
}

pub struct DayTwentyThree;
impl Problem for DayTwentyThree {
    fn part_one(&self, input: &str) -> String {
        let parsed_input = parse_input(input);
        let initial_state = State {
            hallway: [None; 11],
            rooms: [
                [parsed_input[0], parsed_input[4]],
                [parsed_input[1], parsed_input[5]],
                [parsed_input[2], parsed_input[6]],
                [parsed_input[3], parsed_input[7]],
            ],
        };

        solve(initial_state).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let parsed_input = parse_input(input);
        let initial_state = State {
            hallway: [None; 11],
            rooms: [
                [parsed_input[0], Some(Amphipod::D), Some(Amphipod::D), parsed_input[4]],
                [parsed_input[1], Some(Amphipod::C), Some(Amphipod::B), parsed_input[5]],
                [parsed_input[2], Some(Amphipod::B), Some(Amphipod::A), parsed_input[6]],
                [parsed_input[3], Some(Amphipod::A), Some(Amphipod::C), parsed_input[7]],
            ],
        };

        solve(initial_state).to_string()
    }
}
