use crate::problem::problemdef::Problem;
use gag::Gag;
use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};

pub struct DayTwentyFour {}

impl DayTwentyFour {
    fn solve(maximize: bool) -> String {
        let print_gag = Gag::stdout().unwrap();
        let mut vars = variables!();
        let a1 = vars.add(variable().integer().min(1).max(9));
        let a2 = vars.add(variable().integer().min(1).max(9));
        let a3 = vars.add(variable().integer().min(1).max(9));
        let a4 = vars.add(variable().integer().min(1).max(9));
        let a5 = vars.add(variable().integer().min(1).max(9));
        let a6 = vars.add(variable().integer().min(1).max(9));
        let a7 = vars.add(variable().integer().min(1).max(9));
        let a8 = vars.add(variable().integer().min(1).max(9));
        let a9 = vars.add(variable().integer().min(1).max(9));
        let a10 = vars.add(variable().integer().min(1).max(9));
        let a11 = vars.add(variable().integer().min(1).max(9));
        let a12 = vars.add(variable().integer().min(1).max(9));
        let a13 = vars.add(variable().integer().min(1).max(9));
        let a14 = vars.add(variable().integer().min(1).max(9));
        let expr = 1000000 * a1 + 100000 * a2 + 10000 * a3 + 1000 * a5 + 100 * a7 + 10 * a8 + a9;
        let problem = if maximize {
            vars.maximise(expr)
        } else {
            vars.minimise(expr)
        };
        let solution = problem
            .using(default_solver)
            .with(constraint![a4 == a3 - 8])
            .with(constraint![a6 == a5 + 5])
            .with(constraint![a10 == a9 - 2])
            .with(constraint![a11 == a8 - 4])
            .with(constraint![a12 == a7 - 6])
            .with(constraint![a13 == a2 - 3])
            .with(constraint![a14 == a1 + 3])
            .solve()
            .unwrap();
        let r = vec![a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14]
            .iter()
            .map(|a| format!("{}", solution.value(*a).round()))
            .collect::<Vec<String>>()
            .join("");
        drop(print_gag);
        r
    }
}

impl Problem for DayTwentyFour {
    fn part_one(&self, _input: &str) -> String {
        Self::solve(true)
    }

    fn part_two(&self, _input: &str) -> String {
        Self::solve(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
