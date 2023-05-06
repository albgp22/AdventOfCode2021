use crate::problem::problemdef::Problem;
use itertools::Itertools;
use priority_queue::DoublePriorityQueue;

pub struct DayFifteen {}

impl DayFifteen {
    fn read_input(input: &str) -> Vec<Vec<i8>> {
        let mut r = vec![];
        for line in input.split('\n').filter(|l| !l.is_empty()) {
            r.push(
                line.trim()
                    .chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect_vec(),
            );
        }
        r
    }

    fn neighbors(i: usize, j: usize, m: &Vec<Vec<i8>>) -> Vec<(usize, usize)> {
        let mut v = vec![(i + 1, j), (i, j + 1)];
        if i.checked_sub(1).is_some() {
            v.push((i.checked_sub(1).unwrap(), j));
        }
        if j.checked_sub(1).is_some() {
            v.push((i, j.checked_sub(1).unwrap()));
        }
        v.iter()
            .filter(|(i, j)| *i < m.len() && *j < m[0].len())
            .map(|(i, j)| (*i, *j))
            .collect_vec()
    }

    fn dijkstra(m: &Vec<Vec<i8>>) -> i128 {
        let mut q = DoublePriorityQueue::new();

        let mut dists = vec![vec![1000000000000i128; m[0].len()]; m.len()];
        let mut parents = vec![vec![(0, 0); m[0].len()]; m.len()];

        dists[0][0] = 0;

        for i in 0..m.len() {
            for j in 0..m[0].len() {
                q.push((i, j), dists[i][j]);
            }
        }

        while !q.is_empty() {
            let ((i, j), d) = q.pop_min().unwrap();
            for (ii, jj) in Self::neighbors(i, j, m) {
                let alt = d + (m[ii][jj] as i128);
                if alt < dists[ii][jj] {
                    dists[ii][jj] = alt;
                    parents[ii][jj] = (i, j);
                    q.change_priority(&(ii, jj), alt);
                }
            }
        }

        *dists.last().unwrap().last().unwrap()
    }
}

impl Problem for DayFifteen {
    fn part_one(&self, input: &str) -> String {
        let m = Self::read_input(input);
        format!("{}", Self::dijkstra(&m))
    }

    fn part_two(&self, input: &str) -> String {
        let m = Self::read_input(input);
        let mut m2: Vec<Vec<i8>> = vec![vec![0; m[0].len() * 5]; m.len() * 5];
        for i in 0..m.len() * 5 {
            for j in 0..m.len() * 5 {
                let mut candidate =
                    m[i % m.len()][j % m.len()] + (i / m.len()) as i8 + (j / m.len()) as i8;
                while candidate > 9 {
                    candidate -= 9;
                }
                m2[i][j] = candidate;
            }
        }
        format!("{}", Self::dijkstra(&m2))
    }
}

#[cfg(test)]
mod tests {
    
}
