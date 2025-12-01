use std::env;
use std::path::Path;
use std::fs;
use days::*;

mod days;

fn main() {
    if let Some(n) = env::args().nth(1) {
        let day = match n.parse().expect("Please pass a number") {
            1 => Day {
                a_base_tests: vec![],
                a_file_tests: vec![FileTest { path: Path::new("./data/1a_t1.txt"), expected: 3 }],
                b_base_tests: vec![],
                b_file_tests: vec![FileTest { path: Path::new("./data/1a_t1.txt"), expected: 6 }],

                a_data_path: Path::new("./data/1a.txt"),
                b_data_path: Path::new("./data/1b.txt"),

                solver: Box::new(day1::Day1Solver {})
            },
            _ => panic!("Please pass a valid number")
        };

        let mut tests_succeeded = true;

        for base_test in day.a_base_tests {
            print!("# Base Test A ...");
            let solution = day.solver.solve_a(base_test.data);
            if solution == base_test.expected {
                println!(" \x1b[92m✓\x1b[0m");
            } else {
                println!(" \x1b[91m✖ FAILED\x1b[0m");
                println!("Result: {}", solution);
                tests_succeeded = false;
            }
        }
        for file_test in day.a_file_tests {
            let data = fs::read_to_string(file_test.path).expect("File error");
            print!("# File Test A ...");
            let solution = day.solver.solve_a(data.as_str());
            if solution == file_test.expected {
                println!(" \x1b[92m✓\x1b[0m");
            } else {
                println!(" \x1b[91m✖ FAILED\x1b[0m");
                println!("Result: {}", solution);
                tests_succeeded = false;
            }
        }

        if !tests_succeeded { panic!("Tests failed"); }

        let data_a = fs::read_to_string(day.a_data_path).expect("File error");
        println!("Result for part A: {}", day.solver.solve_a(data_a.as_str()));

        for base_test in day.b_base_tests {
            print!("# Base Test B ...");
            let solution = day.solver.solve_b(base_test.data);
            if solution == base_test.expected {
                println!(" \x1b[92m✓\x1b[0m");
            } else {
                println!(" \x1b[91m✖ FAILED\x1b[0m");
                println!("Result: {}", solution);
                tests_succeeded = false;
            }
        }
        for file_test in day.b_file_tests {
            let data = fs::read_to_string(file_test.path).expect("File error");
            print!("# File Test B ...");
            let solution = day.solver.solve_b(data.as_str());
            if solution == file_test.expected {
                println!(" \x1b[92m✓\x1b[0m");
            } else {
                println!(" \x1b[91m✖ FAILED\x1b[0m");
                println!("Result: {}", solution);
                tests_succeeded = false;
            }
        }

        if !tests_succeeded { panic!("Tests failed"); }

        let data_b = fs::read_to_string(day.b_data_path).expect("File error");
        println!("Result for part B: {}", day.solver.solve_b(data_b.as_str()));
    } else {
        panic!("Please pass a number");
    }
}

struct Day<'a> {
    a_base_tests: Vec<BaseTest<'a>>,
    a_file_tests: Vec<FileTest<'a>>,
    b_base_tests: Vec<BaseTest<'a>>,
    b_file_tests: Vec<FileTest<'a>>,

    a_data_path: &'a Path,
    b_data_path: &'a Path,

    solver: Box<dyn DaySolver>
}

trait DaySolver {
    fn solve_a(&self, data: &str) -> u64;
    fn solve_b(&self, data: &str) -> u64;
}

struct BaseTest<'a> {
    data: &'a str,
    expected: u64,
}

struct FileTest<'a> {
    path: &'a Path,
    expected: u64,
}