use std::{iter::once, path::Path};
use itertools::Itertools;

use crate::{DayPart, Test};

pub struct PartOne;
pub struct PartTwo;

fn dist(a: &u64, b: &u64) -> u64 {
    if a > b { a - b }
    else { b - a }
}

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        let points: Vec<(u64, u64, u64)> = data.lines().map(|line| {
            let mut line = line.split(',');
            (
                line.next().expect("Should exist").parse().expect("Should be a number"),
                line.next().expect("Should exist").parse().expect("Should be a number"),
                line.next().expect("Should exist").parse().expect("Should be a number")
            )
        }).collect();

        let connections = (0..points.len()).combinations(2).map(|v| (v[0], v[1])).k_smallest_by_key(10, |(a, b)| {
            let dx = dist(&points[*a].0, &points[*b].0);
            let dy = dist(&points[*a].1, &points[*b].1);
            let dz = dist(&points[*a].2, &points[*b].2);

            dx*dx + dy*dy + dz*dz
        });

        let mut mapping: Vec<_> = (0..points.len()).collect();

        for (a, b) in connections {
            let mut root_a = a;
            let mut root_b = b;
            while root_a != mapping[root_a] { root_a = mapping[root_a] }
            while root_b != mapping[root_b] { root_b = mapping[root_b] }
            if root_a <= root_b {
                mapping[root_b] = root_a;
            } else {
                mapping[root_a] = root_b;
            }
        }

        (0..points.len()).map(|p| (0..points.len()).map(|mut x| {
            while x != mapping[x] { x = mapping[x]; }
            x
        }).filter(|x| *x == p).count() as u64).k_largest(3).product()
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/8t1.txt"),
            expected: 40
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/8.txt").into()
    }
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        let points: Vec<(u64, u64, u64)> = data.lines().map(|line| {
            let mut line = line.split(',');
            (
                line.next().expect("Should exist").parse().expect("Should be a number"),
                line.next().expect("Should exist").parse().expect("Should be a number"),
                line.next().expect("Should exist").parse().expect("Should be a number")
            )
        }).collect();

        let connections = (0..points.len()).combinations(2).map(|v| (v[0], v[1])).sorted_by_key(|(a, b)| {
            let dx = dist(&points[*a].0, &points[*b].0);
            let dy = dist(&points[*a].1, &points[*b].1);
            let dz = dist(&points[*a].2, &points[*b].2);

            dx*dx + dy*dy + dz*dz
        });

        let mut mapping: Vec<_> = (0..points.len()).collect();

        let mut last = None;
        for (a, b) in connections {
            if mapping.iter().map(|x| *x).all(|mut x| x == 0 || mapping[x] != x ) { break; }
            last = Some(points[a].0 * points[b].0);
            let mut root_a = a;
            let mut root_b = b;
            while root_a != mapping[root_a] { root_a = mapping[root_a] }
            while root_b != mapping[root_b] { root_b = mapping[root_b] }
            if root_a <= root_b {
                mapping[root_b] = root_a;
            } else {
                mapping[root_a] = root_b;
            }
        }

        last.expect("should not be empty")
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/8t1.txt"),
            expected: 25272
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/8.txt").into()
    }
}