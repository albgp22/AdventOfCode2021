
use std::time::Instant;

pub trait Problem{
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

impl dyn Problem{
    pub fn run(&self, input: &str){
        println!["Part 1 solution:"];
        println!["{}",self.part_one(input)];
        println!["Part 2 solution:"];
        println!["{}",self.part_two(input)];
    }

    pub fn benchmark(&self, input: &str){
        let now = Instant::now();
        const N: u32 = 100;

        for _ in 0..N{
            self.part_one(input);
        }

        let elapsed = now.elapsed();
        println!("Part 1 took {:.5?} on average", elapsed/N);

        let now = Instant::now();

        for _ in 0..N{
            self.part_two(input);
        }

        let elapsed = now.elapsed();
        println!("Part 2 took {:.5?} on average", elapsed/N);
    }
}
