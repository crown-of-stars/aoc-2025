use std::path::Path;
use std::collections::HashSet;

use crate::{DayPart, Test};

fn parse(data: &str) -> Vec<(u64, u64)> {
    data.split(',').map(|s| {
        let mut nums = s.split('-');
        (nums.next().expect("Should exist").parse().expect("Should be a number"),
            nums.next().expect("Should exist").parse().expect("Should be a number"))
    }).collect()
}

pub struct PartOne;
pub struct PartTwo;

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        parse(data).into_iter()
            .fold(HashSet::new(), |hash_set, (start, stop)| count_range(start, stop, 1, hash_set))
            .iter().sum()
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/2t1.txt"),
            expected: 33
        }, Test {
            id: 2,
            path: Path::new("./data/2t2.txt"),
            expected: 1188511885
        }, Test {
            id: 3,
            path: Path::new("./data/2t3.txt"),
            expected: 1227775554
        }, Test {
            id: 4,
            path: Path::new("./data/2.txt"),
            expected: 18893502033
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/2.txt").into()
    }
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        parse(data).into_iter()
            .fold(HashSet::new(), |hash_set, (start, stop)| {
                (1..stop.checked_ilog10().unwrap() + 1)
                    .fold(hash_set, |hash_set_, n| count_range(start, stop, n.into(), hash_set_))
            }).iter().sum()
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/2t3.txt"),
            expected: 4174379265
        }, Test {
            id: 2,
            path: Path::new("./data/2t1.txt"),
            expected: 33
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/2.txt").into()
    }
}

fn count_range(start: u64, stop_inclusive: u64, seq_count: u64, mut hash_set: HashSet<u64>) -> HashSet<u64> {
    let mut offset = 1;
    let mut counter;

    while start > (10_u64.pow(offset) - 1) * (get_pat(offset, seq_count)) {
        offset += 1;
    }

    counter = start / (get_pat(offset, seq_count));
    counter = counter.max(10_u64.pow(offset - 1));
    
    // println!("Starting at {}", (get_pat(offset, seq_count)) * counter);

    while stop_inclusive >= (get_pat(offset, seq_count)) * counter {
        // println!("Checking at {}; {}, {}", (get_pat(offset, seq_count)) * counter, offset, counter);
        if (get_pat(offset, seq_count)) * counter >= start { hash_set.insert((get_pat(offset, seq_count)) * counter); }
        counter += 1;
        if counter >= 10_u64.pow(offset) {
            offset += 1;
            counter = counter.max(10_u64.pow(offset - 1));
        }
    }

    hash_set
}

fn get_pat(offset: u32, seq_count: u64) -> u64 {
    (0..seq_count).fold(1, |n, _| n * 10_u64.pow(offset) + 1)
}