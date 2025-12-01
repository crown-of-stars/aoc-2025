use std::env;
use std::path::Path;
use std::fs;
use days::*;
use tests::*;
use generate_day::get_day;

mod days;
mod tests;
mod generate_day;

fn main() {
    if let Some(n) = env::args().nth(1) {
        let day = get_day( n.parse().expect("Please pass a number") ).expect("Please pass a valid day number");

        println!("Running tests for part 1.");
        day.part_one.evaluate_tests();
        print!("Result for part 1 ");
        day.part_one.evaluate_solution();
        println!("Running tests for part 2.");
        day.part_two.evaluate_tests();
        print!("Result for part 2 ");
        day.part_two.evaluate_solution();
    } else {
        panic!("Please pass a number");
    }
}

struct Day {
    part_one: Box<dyn DayPart>,
    part_two: Box<dyn DayPart>
}

trait DayPart {
    fn solve(&self, data: &str) -> u64;

    fn tests(&self) -> Vec<Test>;

    fn data_path(&self) -> Box<Path>;

    fn evaluate_solution(&self) {
        let data = fs::read_to_string(self.data_path()).expect("File error");
        println!("{}", self.solve(data.as_str()));
    }

    fn evaluate_tests(&self) {
        let result = self.tests().iter().fold(true, |b, test| {
            if !b { return b; }
            print!(": Tests ...");
            let data = fs::read_to_string(test.path).expect("File error");
            let solution = self.solve(data.as_str());
            if solution == test.expected {
                println!(" \x1b[92m✓\x1b[0m");
            } else {
                println!(" \x1b[91m✖ FAILED\x1b[0m");
                println!("Result: {}", solution);
            }
            solution == test.expected
        });
        if !result { panic!("Failed tests") }
    }
}