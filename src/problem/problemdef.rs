use std::time::Instant;

pub trait Problem {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkResult {
    problem: usize,
    //durations: Vec<Duration>,
    mean_millis_pt1: f64,
    var_millis_squared_pt1: f64,
    mean_millis_pt2: f64,
    var_millis_squared_pt2: f64,
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
        const N: u32 = 3;

        let mut durations1 = vec![];
        let mut durations2 = vec![];

        for _ in 0..N {
            let now = Instant::now();
            self.part_one(input);
            let elapsed = now.elapsed();
            durations1.push(elapsed);
        }

        let elapsed = now.elapsed();
        println!("Part 1 took {:.5?} on average", elapsed / N);

        let now = Instant::now();

        for _ in 0..N {
            let now = Instant::now();
            self.part_two(input);
            let elapsed = now.elapsed();
            durations2.push(elapsed);
        }

        let elapsed = now.elapsed();
        println!("Part 2 took {:.5?} on average", elapsed / N);

        let mean1 = durations1
            .iter()
            .map(|f| f.as_secs_f64() * 1000_f64)
            .fold(0f64, |x, y| x + y)
            / (durations1.len() as f64);
        let var1 = durations1
            .iter()
            .map(|f| f.as_secs_f64() * 1000_f64)
            .fold(0f64, |x, y| x + (mean1 - y).powi(2))
            / (durations1.len() as f64);
        let mean2 = durations2
            .iter()
            .map(|f| f.as_secs_f64() * 1000_f64)
            .fold(0f64, |x, y| x + y)
            / (durations2.len() as f64);
        let var2 = durations2
            .iter()
            .map(|f| f.as_secs_f64() * 1000_f64)
            .fold(0f64, |x, y| x + (mean2 - y).powi(2))
            / (durations2.len() as f64);

        BenchmarkResult {
            problem: 0,
            mean_millis_pt1: mean1,
            var_millis_squared_pt1: var1,
            mean_millis_pt2: mean2,
            var_millis_squared_pt2: var2,
        }
    }
}
