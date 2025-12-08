use std::path::Path;
use std::iter;

use crate::{DayPart, Test};

pub struct PartOne;
pub struct PartTwo;

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        enum Problem {
            Addition(u64),
            Multiplication(u64),
            Unknown(u64, u64) // add, mult
        }

        let mut problems = vec![];
        for line in data.lines() {
            for (n, item) in line.split_whitespace().enumerate() {
                match item {
                    "+" => {
                        problems[n] = match problems[n] {
                            Problem::Addition(x) => Problem::Addition(x),
                            Problem::Multiplication(x) => Problem::Multiplication(x),
                            Problem::Unknown(x, _) => Problem::Addition(x),
                        }
                    },
                    "*" => {
                        problems[n] = match problems[n] {
                            Problem::Addition(x) => Problem::Addition(x),
                            Problem::Multiplication(x) => Problem::Multiplication(x),
                            Problem::Unknown(_, y) => Problem::Multiplication(y),
                        }
                    }
                    item => {
                        let number = item.parse().expect("Should be a number");
                        if problems.len() <= n {
                            problems.push(Problem::Unknown(number, number));
                        } else {
                            problems[n] = match problems[n] {
                                Problem::Addition(x) => Problem::Addition(x + number),
                                Problem::Multiplication(x) => Problem::Multiplication(x * number),
                                Problem::Unknown(x, y) => Problem::Unknown(x + number, y * number),
                            }
                        }
                    }
                }
            }
        }

        problems.into_iter().filter_map(|p| match p {
            Problem::Addition(x) => Some(x),
            Problem::Multiplication(x) => Some(x),
            Problem::Unknown(_, _) => None,
        }).sum()
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/6t1.txt"),
            expected: 4277556
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/6.txt").into()
    }
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        todo!()
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/6t1.txt"),
            expected: 3263827
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/6.txt").into()
    }
}

fn quersumme(x: u64) -> u64 {
    if x == 0 { return 0; }
    x % 10 + quersumme(x / 10)
}

