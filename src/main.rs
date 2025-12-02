use std::env;
use std::path::Path;
use std::fs;

include!(concat!(env!("OUT_DIR"), "\\generate_day.rs"));

fn main() {
    println!(concat!(env!("OUT_DIR"), "\\generate_day.rs"));
    if let Some(n) = env::args().nth(1) {
        let day = get_day( n.parse().expect("Please pass a number") ).expect("Please pass a valid day number.");

        println!("# Running tests for part 1.");
        day.part_one.evaluate_tests();
        print!("\nResult for part 1: ");
        day.part_one.evaluate_solution();
        bench(day.part_one);
        println!("\n# Running tests for part 2.");
        day.part_two.evaluate_tests();
        print!("\nResult for part 2: ");
        day.part_two.evaluate_solution();
        bench(day.part_two);
    } else {
        panic!("Please pass a number");
    }
}

pub struct Day {
    part_one: Box<dyn DayPart>,
    part_two: Box<dyn DayPart>
}

trait DayPart {
    fn solve(&self, data: &str) -> u64;

    fn tests(&self) -> Vec<Test>;

    fn data_path(&self) -> Box<Path>;

    fn evaluate_solution(&self) {
        let data = fs::read_to_string(self.data_path()).expect("File error");
        println!("\x1b[4m{}\x1b[0m", self.solve(data.as_str()));
    }

    fn evaluate_tests(&self) {
        let result = self.tests().iter().fold(true, |b, test| {
            print!(": Test {} ", test.id);
            let data = fs::read_to_string(test.path).expect("File error");
            let solution = self.solve(data.as_str());
            if solution == test.expected {
                println!("\x1b[92m✓\x1b[0m");
            } else {
                println!("\x1b[91m✕ FAILED\x1b[0m");
                println!("⌁ Result: {}", solution);
                println!("  Expected: {}", test.expected);
            }
            b && solution == test.expected
        });
        if !result { panic!("Failed tests") }
    }
}

pub struct Test<'a> {
    pub id: usize,
    pub path: &'a Path,
    pub expected: u64,
}

const BENCH_COUNT: usize = 1000;
const BACKSPACE: char = 8u8 as char;

fn bench(part: Box<dyn DayPart>) {    
    let data = fs::read_to_string(part.data_path()).expect("File error");

    use std::time::Instant;

    let mut steps = 0;
    let mut ticking_up = 0;

    let times: Vec<_> = (0..BENCH_COUNT).map(|_| {
        let now = Instant::now();
        let _ = part.solve(data.as_str());
        now.elapsed().as_micros() as f32
    }).collect();

    let mean = times.iter().sum::<f32>() / BENCH_COUNT as f32;
    let variance: f32 = times.iter().map(|t| (t - mean) * (t - mean)).sum::<f32>() / BENCH_COUNT as f32;
    
    println!("Finished in: avg {} us : stddev {} us", mean, variance.sqrt());
}