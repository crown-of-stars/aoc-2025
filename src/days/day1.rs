use std::path::Path;

use crate::{Day, DayPart, Test};

fn parse(data: &str) -> Vec<LockRotation> {
    data.lines().map(|s| {
        let mut chars = s.chars();
        let direction = match chars.next().expect("Parse error") {
            'L' => LockDirection::L,
            'R' => LockDirection::R,
            _ => panic!("Parse error")
        };
        let travel = chars.as_str().parse().expect("Parse error");
        LockRotation {
            direction: direction,
            travel: travel
        }
    }).collect()
}

pub struct PartOne;
pub struct PartTwo;

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        let rotations = parse(data);

        let mut counter = 0;

        Lock {
            state: LockState { digit: 50 },
            checker: |lock_state, is_end| if is_end && lock_state.digit == 0 { counter += 1 }
        }.rotate(rotations);

        counter
    }
    
    fn tests(&self) -> Vec<Test> {
        todo!()
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/1a.txt").into()
    }
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        let rotations = parse(data);

        let mut counter = 0;

        Lock {
            state: LockState { digit: 50 },
            checker: |lock_state, _| if lock_state.digit == 0 { counter += 1 }
        }.rotate(rotations);

        counter
    }
    
    fn tests(&self) -> Vec<Test> {
        todo!()
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/1b.txt").into()
    }
}

struct Lock<S, C: FnMut(&S, bool)> {
    state: S,
    checker: C,
}

struct LockState {
    digit: isize,
}

enum LockDirection {
    L, // Down
    R, // Up
}

struct LockRotation {
    direction: LockDirection,
    travel: isize,
}

impl<C> Lock<LockState, C> where C: FnMut(&LockState, bool) {
    fn rotate(&mut self, rotations: Vec<LockRotation>) {
        for rotation in rotations {
            (0..rotation.travel).for_each(|n| {
                self.state.digit += match rotation.direction { LockDirection::L => -1, LockDirection::R => 1};
                self.state.digit = self.state.digit.rem_euclid(100);
                (self.checker)(&self.state, n == rotation.travel - 1);
            });
        }
    }
}