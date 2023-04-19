use std::collections::HashMap;
extern crate itertools;

use itertools::Itertools;

use crate::problem::problemdef::Problem;

pub struct DayFourteen {}

impl DayFourteen {
    fn read_input(input: &str) -> (String, HashMap<String, String>) {
        let mut lines = input.split('\n').filter(|l| !l.is_empty());
        let initial = lines.next().unwrap().to_string();
        let mut rules = HashMap::new();
        for line in lines {
            let mut ruleparts = line.split("->");
            rules.insert(
                ruleparts.next().unwrap().trim().to_string(),
                ruleparts.next().unwrap().trim().to_string(),
            );
        }
        (initial, rules)
    }

    fn step(input: String, rules: &HashMap<String, String>) -> String {
        let mut r = "".to_string();
        let mut first = true;
        for (a, b) in input.chars().tuple_windows() {
            let curr = format!("{}{}", a, b);
            if first {
                first = false;
                r.push_str(
                    &(if rules.contains_key(&curr) {
                        format!("{}{}{}", a, rules.get(&curr).unwrap(), b)
                    } else {
                        curr
                    }),
                );
            } else {
                r.push_str(
                    &(if rules.contains_key(&curr) {
                        format!("{}{}", rules.get(&curr).unwrap(), b)
                    } else {
                        b.to_string()
                    }),
                );
            }
        }
        r
    }

    fn count_chars(s: String) -> HashMap<char, i32> {
        s.to_lowercase().chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
    }
}

impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let (initial, rules) = Self::read_input(input);
        let mut curr = initial;
        for _i in 0..10 {
            curr = Self::step(curr, &rules);
        }
        let cc = Self::count_chars(curr);
        format!(
            "{}",
            cc.iter().map(|(_k, &v)| v).max().unwrap() - cc.iter().map(|(_k, &v)| v).min().unwrap()
        )
    }

    fn part_two(&self, input: &str) -> String {
        let (initial, rules) = Self::read_input(input);

        let pairs = initial
            .chars()
            .tuple_windows()
            .map(|(a, b)| format!("{}{}", a, b));

        let mut pairs_occurrences = HashMap::new();

        for p in pairs {
            *pairs_occurrences.entry(p).or_insert(0) += 1;
        }

        for _ in 0..40 {
            let mut pairs_occurrences_inc = HashMap::<String, i128>::new();

            for (k, v) in pairs_occurrences.iter() {
                if rules.contains_key(k) {
                    let newpair1 =
                        format!("{}{}", k.chars().next().unwrap(), rules.get(k).unwrap());
                    let newpair2 = format!(
                        "{}{}",
                        rules.get(k).unwrap(),
                        k.chars().skip(1).next().unwrap(),
                    );
                    *pairs_occurrences_inc.entry(k.to_string()).or_insert(0) -= *v;
                    *pairs_occurrences_inc.entry(newpair1).or_insert(0) += *v;
                    *pairs_occurrences_inc.entry(newpair2).or_insert(0) += *v;
                }
            }

            for (k, v) in pairs_occurrences_inc.iter() {
                *pairs_occurrences.entry(k.to_string()).or_insert(0) += *v;
            }
        }

        let mut char_occurrences = HashMap::new();

        for (k, v) in pairs_occurrences.iter() {
            for c in k.chars() {
                *char_occurrences.entry(c).or_insert(0) += *v;
            }
        }

        format!(
            "{}",
            (char_occurrences.iter().map(|(_k, v)| v).max().unwrap()
                - char_occurrences.iter().map(|(_k, v)| v).min().unwrap())
                / 2
        )
    }
}
