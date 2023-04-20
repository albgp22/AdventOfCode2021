use crate::problem::problemdef::Problem;
use regex::Regex;

pub struct DaySeventeen {}

#[allow(dead_code)]
impl DaySeventeen {
    fn x(i: i128, v0: i128) -> i128 {
        let m = (i - 1).min(v0);
        (m + 1) * v0 - m * (m + 1) / 2
    }

    fn y(i: i128, v0: i128) -> i128 {
        i * v0 - (i - 1) * i / 2
    }
}

impl Problem for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let y_re = Regex::new(r"y=(-?\d+)..(-?\d+)").unwrap();
        let y_cap = y_re.captures_iter(&input).next().unwrap();
        let ydownlim: i32= y_cap[1].parse().unwrap();
        let h_max = ((ydownlim.abs() as f64 - 1f64) + 0.5).powi(2) / (2 as f64);
        format!("{}", h_max.round())
    }

    fn part_two(&self, input: &str) -> String {
        let x_re = Regex::new(r"x=(-?\d+)..(-?\d+)").unwrap();
        let y_re = Regex::new(r"y=(-?\d+)..(-?\d+)").unwrap();
        let x_cap = x_re.captures_iter(&input).next().unwrap();
        let xdownlim: i128= x_cap[1].parse().unwrap();
        let xuplim: i128 = x_cap[2].parse().unwrap();
        let y_cap = y_re.captures_iter(&input).next().unwrap();
        let ydownlim: i128= y_cap[1].parse().unwrap();
        let yuplim: i128 = y_cap[2].parse().unwrap();

        let hits = |v0x: i128, v0y: i128| {
            let mut x = 0;
            let mut y = 0;
            let mut vx = v0x;
            let mut vy = v0y;

            loop {
                if x > xuplim {
                    return false;
                }
                if vx == 0 && (x < xdownlim || x > xuplim) {
                    return false;
                }
                if vx == 0 && y < ydownlim {
                    return false;
                }
                if x >= xdownlim && x <= xuplim && y >= ydownlim && y <= yuplim {
                    return true;
                }
                x += vx;
                y += vy;
                if vx > 0 {
                    vx -= 1;
                }
                vy -= 1;
            }
        };

        let ymax = ydownlim.abs().max(yuplim.abs());
        let mut num_velocities = 0;

        for v0x in 0..=xuplim {
            for v0y in -ymax..=ymax {
                if hits(v0x, v0y) {
                    num_velocities += 1
                }
            }
        }
        format!("{}", num_velocities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
