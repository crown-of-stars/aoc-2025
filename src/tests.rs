use std::path::*;
use std::fs;

use crate::DayPart;

pub struct Test<'a> {
    pub path: &'a Path,
    pub expected: u64,
}

impl<'a> Test<'a> {
    pub fn assert<D, Solver: Fn(&str) -> u64>(&self, part: &D) -> (bool, u64) where D: DayPart {
        let data = fs::read_to_string(part.data_path()).expect("File error");
        let result = part.solve(data.as_str());
        return (result == self.expected, result)
    }
}