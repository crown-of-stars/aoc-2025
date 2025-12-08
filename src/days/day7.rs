use std::{iter::once, path::Path};
use itertools::Itertools;

use crate::{DayPart, Test};

pub struct PartOne;
pub struct PartTwo;

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        data.lines().fold((0, None), |(n, state): (_, Option<String>), line| {
            let line: String = line.to_string();
            if let Some(state) = state {
                let mut splits = 0;
                let register_splits = once('.').chain(state.chars().zip(line.chars()).map(|(x, y)| {
                    match (x, y) {
                        ('.', _) => '.',
                        ('S', '.') => 'S',
                        ('S', '^') => {
                            splits += 1;
                            's'
                        },
                        (ca, cb) => panic!("Character not recognized: {} {}", ca.escape_default(), cb.escape_default())
                    }
                })).chain(once('.'));
                let new_state = register_splits.tuple_windows().map(|x| match x {
                    (_, 'S', _) => 'S',
                    ('s', _, _) | (_, _, 's') => 'S',
                    _ => '.'
                }).collect();
                (n + splits, Some(new_state))
            } else {
                (n, Some(line))
            }
        }).0
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/7t1.txt"),
            expected: 21
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/7.txt").into()
    }
}

#[derive(Clone)]
enum TachyonState {
    Tachyons(u64),
    Split(u64),
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        data.lines().fold(None, |state: Option<Vec<_>>, line| {
            let line: String = line.to_string();
            if let Some(state) = state {
                let register_splits = once(TachyonState::Tachyons(0)).chain(state.into_iter().zip(line.chars()).map(|(x, y)| {
                    match (x, y) {
                        (TachyonState::Tachyons(n), '.') => TachyonState::Tachyons(n),
                        (TachyonState::Tachyons(n), '^') => TachyonState::Split(n),
                        (TachyonState::Split(_), _) => panic!("Inappropriate split"),
                        (_, c) => panic!("Character not recognized: {}", c.escape_default())
                    }
                })).chain(once(TachyonState::Tachyons(0)));
                let new_state = register_splits.tuple_windows().map(|(l, c, r)| {
                    let down = match c {
                        TachyonState::Tachyons(n) => n,
                        TachyonState::Split(_) => 0,
                    };
                    let left = match l {
                        TachyonState::Tachyons(_) => 0,
                        TachyonState::Split(n) => n,
                    };
                    let right = match r {
                        TachyonState::Tachyons(_) => 0,
                        TachyonState::Split(n) => n,
                    };
                    TachyonState::Tachyons(down + left + right)
                }).collect();
                Some(new_state)
            } else {
                Some(line.chars().map(|x| match x {
                    'S' => TachyonState::Tachyons(1),
                    '.' => TachyonState::Tachyons(0),
                    c => panic!("Character not recognized: {}", c.escape_default())
                }).collect())
            }
        }).expect("Should have an output").into_iter().map(|x| match x {
            TachyonState::Tachyons(n) => n,
            TachyonState::Split(_) => 0,
        }).sum()
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/7t1.txt"),
            expected: 40
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/7.txt").into()
    }
}