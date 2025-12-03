use std::path::Path;

use crate::{DayPart, Test};

pub struct PartOne;
pub struct PartTwo;

enum Batteries {
    NoBatteries,
    OneBattery(char),
    TwoBatteries(char, char, char) // a, b, previous
}

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        data.lines().map(|line| {
            match line.chars().fold(Batteries::NoBatteries, |acc, c| {
                match acc {
                    Batteries::NoBatteries => Batteries::OneBattery(c),
                    Batteries::OneBattery(b) => Batteries::TwoBatteries(b, c, c),
                    Batteries::TwoBatteries(a, b, previous) => {
                        if a < previous {
                            Batteries::TwoBatteries(previous, c, c)
                        } else {
                            if b < c {
                                Batteries::TwoBatteries(a, c, c)
                            } else {
                                Batteries::TwoBatteries(a, b, c)
                            }
                        }
                    }
                }
            }) {
                Batteries::NoBatteries => None,
                Batteries::OneBattery(_) => None,
                Batteries::TwoBatteries(a, b, _) => {
                    let a = a.to_digit(10).expect("Should be a digit") as u64;
                    let b = b.to_digit(10).expect("Should be a digit") as u64;
                    Some(10 * a + b)
                }
            }.expect("Should have found enough batteries")
        }).sum()
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/3t1.txt"),
            expected: 357
        }, Test {
            id: 2,
            path: Path::new("./data/3t2.txt"),
            expected: 98
        }, Test {
            id: 3,
            path: Path::new("./data/3t3.txt"),
            expected: 89
        }, Test {
            id: 4,
            path: Path::new("./data/3t4.txt"),
            expected: 78
        }, Test {
            id: 5,
            path: Path::new("./data/3t5.txt"),
            expected: 92
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/3.txt").into()
    }
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        data.lines().map(|line| recursive_joltage(line, 12)).sum()
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/3t1.txt"),
            expected: 3121910778619
        }, Test {
            id: 2,
            path: Path::new("./data/3t2.txt"),
            expected: 987654321111
        }, Test {
            id: 3,
            path: Path::new("./data/3t3.txt"),
            expected: 811111111119
        }, Test {
            id: 4,
            path: Path::new("./data/3t4.txt"),
            expected: 434234234278
        }, Test {
            id: 5,
            path: Path::new("./data/3t5.txt"),
            expected: 888911112111
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/3.txt").into()
    }
}

fn recursive_joltage(string: &str, take_batteries: u32) -> u64 {
    if take_batteries == 0 { return 0; }

    let (joltage_location, joltage) = string[..string.len() - take_batteries as usize + 1].chars().enumerate().max_by(|(ia, xa), (ib, xb)| {
        if xa != xb { xa.cmp(xb) }
        else { ib.cmp(ia) }
    }).expect("Battery should exist");

    joltage.to_digit(10).expect("Should be a digit") as u64 * 10_u64.pow(take_batteries - 1)
        + recursive_joltage(&string[(joltage_location + 1)..], take_batteries - 1)
}