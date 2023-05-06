use crate::problem::problemdef::Problem;

pub struct DayThirteen {}

#[derive(Debug, Copy, Clone)]
enum Folding {
    Y(usize),
    X(usize),
}

impl DayThirteen {
    fn read_input(input: &str) -> (Vec<(usize, usize)>, Vec<Folding>) {
        let mut dots = vec![];
        let mut foldings = vec![];

        for line in input.split('\n').filter(|l| !l.is_empty()) {
            if line.contains("fold") {
                let op = line.split(' ').nth(2).unwrap();
                let rowcol = op
                    .split('=').nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                foldings.push(if op.contains("x=") {
                    Folding::X(rowcol)
                } else {
                    Folding::Y(rowcol)
                });
            } else {
                let mut coords = line.split(',');
                dots.push((
                    coords.next().unwrap().parse().unwrap(),
                    coords.next().unwrap().parse().unwrap(),
                ));
            }
        }

        (dots, foldings)
    }

    fn fold(dots: &mut Vec<(usize, usize)>, fold: Folding) {
        match fold {
            Folding::X(col) => {
                for dot in dots.iter_mut() {
                    if dot.0 == col {
                        panic!()
                    }
                    if dot.0 > col {
                        dot.0 = (2 * col).checked_sub(dot.0).unwrap()
                    }
                }
            }
            Folding::Y(row) => {
                for dot in dots.iter_mut() {
                    if dot.1 == row {
                        panic!()
                    }
                    if dot.1 > row {
                        dot.1 = (2 * row).checked_sub(dot.1).unwrap()
                    }
                }
            }
        }
        dots.sort();
        dots.dedup();
    }

    fn show_dots(dots: &Vec<(usize, usize)>) -> String {
        let mut r = "".to_string();
        for j in 0..=*dots.iter().map(|(_i, j)| j).max().unwrap() {
            for i in 0..=*dots.iter().map(|(i, _j)| i).max().unwrap() {
                r = format!("{}{}", r, if dots.contains(&(i, j)) { "#" } else { "." });
            }
            r = format!("{}\n", r);
        }
        r
    }
}
impl Problem for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        let (mut dots, foldings) = Self::read_input(input);
        Self::fold(&mut dots, foldings[0]);
        format!["{}", dots.len()]
    }

    fn part_two(&self, input: &str) -> String {
        let (mut dots, foldings) = Self::read_input(input);
        for fold in foldings {
            Self::fold(&mut dots, fold)
        }
        Self::show_dots(&dots)
    }
}
