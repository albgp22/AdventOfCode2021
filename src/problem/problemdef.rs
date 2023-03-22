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
}
