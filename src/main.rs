use std::path::Path;

fn main() {
    println!("Hello, world!");
}

struct Day<SolverA: DaySolver, SolverB: DaySolver> {
    a_base_tests: Vec<BaseTest>,
    a_file_tests: Vec<FileTest>,
    b_base_tests: Vec<BaseTest>,
    b_file_tests: Vec<FileTest>,

    a_data_path: Box<Path>,
    b_data_path: Box<Path>,
}

trait DaySolver {
    fn solve(data: &str) -> u64;
}

struct BaseTest {
    data: String,
    expected: u64,
}

struct FileTest {
    path: Box<Path>,
    expected: u64,
}

impl Day<SolverA: DaySolver, SolverB: DaySolver> {
    fn solve_a() {
        
    }
}