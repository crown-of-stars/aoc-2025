use std::{iter::once, path::Path, collections::HashMap};

use crate::{DayPart, Test};

pub struct PartOne;
pub struct PartTwo;

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        data.lines().map(|s| {
            let mut nums = s.split(',');
            (
                nums.next().expect("should exist").parse::<u64>().expect("should be a number"),
                nums.next().expect("should exist").parse::<u64>().expect("should be a number")
            )
        }).fold((None, vec![]), |(largest, mut v), (x, y)| {
            (
                v.iter().fold(largest, |l, (px, py)| {
                    let dprod = (x.abs_diff(*px) + 1) * (y.abs_diff(*py) + 1);
                    l.map(|q: u64| dprod.max(q)).or(Some(dprod))
                }),
                {
                    v.push((x, y));
                    v
                }
            )
        }).0.expect("should not be empty")
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/9t1.txt"),
            expected: 50
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/9.txt").into()
    }
}

enum TileColor {
    RedV, // top entry
    RedH, // other
    Green,
    GreenH,
    Black,
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        let red_points: Vec<_> = data.lines().map(|s| {
            let mut nums = s.split(',');
            (
                nums.next().expect("should exist").parse::<usize>().expect("should be a number"),
                nums.next().expect("should exist").parse::<usize>().expect("should be a number")
            )
        }).collect();

        let mut max_x = red_points.iter().map(|(x, _)| *x).max().expect("should not be empty");
        let mut max_y = red_points.iter().map(|(_, y)| *y).max().expect("should not be empty");

        let mut grid: Vec<Vec<_>> = (0..=max_x).map(|_| (0..=max_y).map(|_| TileColor::Black).collect()).collect();
        let (mut last_x, mut last_y) = *red_points.iter().last().expect("should not be empty");

        // println!();
        for (x, y) in &red_points {
            grid[*x][*y] = if *y < last_y { TileColor::RedV } else { TileColor::RedH };
            if last_x == *x {
                for py in (last_y.min(*y) + 1)..last_y.max(*y) {
                    grid[*x][py] = TileColor::Green;
                }
            } else {
                for px in (last_x.min(*x) + 1)..last_x.max(*x) {
                    grid[px][*y] = TileColor::GreenH;
                }
            }
            (last_x, last_y) = (*x, *y);
        }

        println!("#");
        
        for y in 0..=max_y {
            let mut green = false;
            for x in 0..=max_x {
                match grid[x][y] {
                    TileColor::RedV | TileColor::Green => { green = !green },
                    TileColor::GreenH | TileColor::RedH => {}
                    TileColor::Black => if green { grid[x][y] = TileColor::Green },
                };
            }
            if y % 100 == 0 { println!("{}", y); }
        }

        println!("#");

        let mut i = 0;

        red_points.into_iter().fold((None, vec![]), |(largest, mut v), (x, y)| {
            println!("{}", i); i += 1;
            (
                v.iter().fold(largest, |l, (px, py): &(usize, usize)| {
                    let dprod = if (*px.min(&x)..=*px.max(&x))
                        .flat_map(|cx| (*py.min(&y)..=*py.max(&y)).map(move |cy| (cx, cy)))
                        .all(|(cx, cy)| match grid[cx][cy] {
                            TileColor::Black => false,
                            _ => true,
                        }
                    ) {
                        (x.abs_diff(*px) + 1) as u64 * (y.abs_diff(*py) + 1) as u64
                    } else { 0 };

                    l.map(|q| dprod.max(q)).or(Some(dprod))
                }),
                {
                    v.push((x, y));
                    v
                }
            )
        }).0.expect("should not be empty")
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/9t1.txt"),
            expected: 24
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/9.txt").into()
    }
}