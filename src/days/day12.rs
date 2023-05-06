use std::collections::HashMap;

use crate::problem::problemdef::Problem;

pub struct DayTwelve {}
#[derive(Debug)]
struct Graph {
    edges: Vec<(usize, usize)>,
    labels: Vec<String>,
}

impl Graph {
    fn dfs(&self, origin: usize, visited: &mut Vec<usize>, counter: &mut usize) {
        if origin == self.labels.iter().position(|s| s == "end").unwrap() {
            *counter += 1;
            return;
        }
        for &e in self.edges.iter() {
            if e.1 == origin {
                if visited.contains(&e.0) && self.labels[e.0] == self.labels[e.0].to_lowercase() {
                    continue;
                }
                visited.push(origin);
                self.dfs(e.0, visited, counter);
                visited.pop();
            } else if e.0 == origin {
                if visited.contains(&e.1) && self.labels[e.1] == self.labels[e.1].to_lowercase() {
                    continue;
                }
                visited.push(origin);
                self.dfs(e.1, visited, counter);
                visited.pop();
            }
        }
    }
    fn dfs2(
        &self,
        origin: usize,
        visited_amount: &mut HashMap<usize, usize>,
        counter: &mut usize,
        path: String,
    ) {
        if visited_amount
            .iter()
            .filter(|(&i, _j)| self.labels[i] == self.labels[i].to_lowercase())
            .filter(|(_i, &j)| j > 1)
            .count()
            > 1
        {
            return;
        }
        if origin == self.labels.iter().position(|s| s == "end").unwrap() {
            {
                *counter += 1;
                return;
            }
        }
        for &e in self.edges.iter() {
            if e.1 == origin {
                if *visited_amount.get(&e.0).unwrap() > 1usize
                    && self.labels[e.0] == self.labels[e.0].to_lowercase()
                {
                    continue;
                }
                if e.0 == self.labels.iter().position(|s| s == "start").unwrap() {
                    continue;
                }
                *visited_amount.get_mut(&origin).unwrap() += 1;
                self.dfs2(
                    e.0,
                    visited_amount,
                    counter,
                    format!("{},{}", path, self.labels[e.0]),
                );
                *visited_amount.get_mut(&origin).unwrap() -= 1;
            } else if e.0 == origin {
                if *visited_amount.get(&e.1).unwrap() > 1usize
                    && self.labels[e.1] == self.labels[e.1].to_lowercase()
                {
                    continue;
                }
                if e.1 == self.labels.iter().position(|s| s == "start").unwrap() {
                    continue;
                }
                *visited_amount.get_mut(&origin).unwrap() += 1;
                self.dfs2(
                    e.1,
                    visited_amount,
                    counter,
                    format!("{},{}", path, self.labels[e.1]),
                );
                *visited_amount.get_mut(&origin).unwrap() -= 1;
            }
        }
    }
}

impl DayTwelve {
    fn read_tunnels(input: &str) -> Graph {
        let mut edges = vec![];
        let mut labels = vec![];
        for line in input.split('\n').filter(|l| !l.is_empty()) {
            let mut link = line.split('-');
            let origin = link.next().unwrap();
            let dest = link.next().unwrap();

            let origin_idx = match labels.iter().position(|r| r == origin) {
                Some(idx) => idx,
                None => {
                    labels.push(origin.to_string());
                    labels.len().checked_sub(1).unwrap()
                }
            };
            let dest_idx = match labels.iter().position(|r| r == dest) {
                Some(idx) => idx,
                None => {
                    labels.push(dest.to_string());
                    labels.len().checked_sub(1).unwrap()
                }
            };
            edges.push((origin_idx, dest_idx));
        }
        Graph {
            edges,
            labels,
        }
    }
}

impl Problem for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let g = Self::read_tunnels(input);
        let mut counter = 0;
        g.dfs(
            g.labels.iter().position(|s| s == "start").unwrap(),
            &mut vec![],
            &mut counter,
        );
        format!("{}", counter)
    }

    fn part_two(&self, input: &str) -> String {
        let g = Self::read_tunnels(input);
        let mut counter = 0;
        let mut visited = HashMap::new();
        for n in 0..g.labels.len() {
            visited.insert(n, 0);
        }
        g.dfs2(
            g.labels.iter().position(|s| s == "start").unwrap(),
            &mut visited,
            &mut counter,
            "start".to_string(),
        );
        format!("{}", counter)
    }
}
