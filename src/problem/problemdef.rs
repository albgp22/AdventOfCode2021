use std::time::Instant;

pub trait Problem {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkResult {
    problem: usize,
    //durations: Vec<Duration>,
    mean_millis: f64,
    var_millis_squared: f64,
}

impl BenchmarkResult {
    pub fn set_problem_num(&mut self, i: usize) {
        self.problem = i;
    }
}

impl dyn Problem {
    pub fn run(&self, input: &str) {
        println!["Part 1 solution:"];
        println!["{}", self.part_one(input)];
        println!["Part 2 solution:"];
        println!["{}", self.part_two(input)];
    }

    pub fn benchmark(&self, input: &str) -> BenchmarkResult {
        let now = Instant::now();
        const N: u32 = 10;

        let mut durations = vec![];

        for _ in 0..N {
            let now = Instant::now();
            self.part_one(input);
            let elapsed = now.elapsed();
            durations.push(elapsed);
        }

        let elapsed = now.elapsed();
        println!("Part 1 took {:.5?} on average", elapsed / N);

        let now = Instant::now();

        for _ in 0..N {
            self.part_two(input);
        }

        let elapsed = now.elapsed();
        println!("Part 2 took {:.5?} on average", elapsed / N);

        let mean = durations
            .iter()
            .map(|f| f.as_secs_f64() * (1000 as f64))
            .fold(0f64, |x, y| x + y)
            / (durations.len() as f64);
        let var = durations
            .iter()
            .map(|f| f.as_secs_f64() * (1000 as f64))
            .fold(0f64, |x, y| x + (mean - y).powi(2))
            / (durations.len() as f64);

        BenchmarkResult {
            problem: 0,
            //durations: durations,
            mean_millis: mean,
            var_millis_squared: var,
        }
    }
}
