
use crate::problem::problemdef::Problem;
use itertools::Itertools;

pub struct DayTwentyTwo {}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq)]
enum ActionType{
    ON, OFF
}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq)]
struct Interval{
    start: i32,
    end: i32,
}

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
struct Cube{
    at: ActionType,
    coords: Vec<Interval>,
}

impl DayTwentyTwo{
    fn parse_input(input: &str) -> Vec<Cube>{
        let mut ret = vec![];
        for line in input.split('\n').filter(|l| !l.is_empty()){
            let mut lineparts = line.split(' ');
            let at = match lineparts.next().unwrap() {
                "on" => ActionType::ON,
                "off" => ActionType::OFF,
                _ => unreachable!(),
            };
            let mut coords = vec![];
            for coord in lineparts.next().unwrap().split(',').map(|s| s.trim().replace(&['x','y','z','='], "")){
                let (x,y) = coord.split("..").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
                coords.push(Interval{start:x,end:y});
            }
            ret.push(Cube{at: at, coords: coords});
        }
        ret
    }
}

impl Problem for DayTwentyTwo {
    fn part_one(&self, input: &str) -> String {
        format!("{:?}", Self::parse_input(input))
    }

    fn part_two(&self, input: &str) -> String {
        format!("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
