use std::path::Path;

use crate::{DayPart, Test};

pub struct PartOne;
pub struct PartTwo;

fn parse(data: &str) -> (usize, usize, Vec<Vec<bool>>) {
    let (x_size, y_size) = data.chars().fold((0, 1), |(x_size, y_size), c| {
        match c {
            '@' | '.' => (x_size + 1, y_size),
            '\n' => (0, y_size + 1),
            _ => (x_size, y_size),
        }
    });

    let mut grid: Vec<Vec<bool>> = vec![vec![]];

    for c in data.chars() {
        let grid_height = grid.len() - 1;
        match c {
            '@' => grid[grid_height].push(true),
            '.' => grid[grid_height].push(false),
            '\n' => grid.push(vec![]),
            '\r' => {},
            character => {
                println!("Encountered character {}", character.escape_default());
                panic!("Unexpected character")
            }
        }
    }

    (x_size, y_size, grid)
}

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        let (x_size, y_size, mut grid) = parse(data);

        clear_obstacles(&mut grid, x_size, y_size, false)
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/4t1.txt"),
            expected: 13
        }, Test {
            id: 2,
            path: Path::new("./data/4.txt"),
            expected: 1486
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/4.txt").into()
    }
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        let (x_size, y_size, mut grid) = parse(data);

        clear_obstacle_loop(&mut grid, x_size, y_size, true)
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/4t1.txt"),
            expected: 43
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/4.txt").into()
    }
}

fn clear_obstacle_loop(grid: &mut Vec<Vec<bool>>, x_size: usize, y_size: usize, remove: bool) -> u64 {
    let removed = clear_obstacles(grid, x_size, y_size, remove);
    if removed == 0 { return removed; }
    else { return removed + clear_obstacle_loop(grid, x_size, y_size, remove); }
}

fn clear_obstacles(grid: &mut Vec<Vec<bool>>, x_size: usize, y_size: usize, remove: bool) -> u64 {
    let mut removed = 0;
    for x in 0..x_size as i64 {
        for y in 0..y_size as i64 {
            if !grid[y as usize][x as usize] { continue; }
            if [-1, 0, 1].into_iter().flat_map(|x_add| [-1, 0, 1].into_iter()
                    .filter(move |y_add| (x_add, *y_add) != (0, 0))
                    .map(move |y_add| (x + x_add, y + y_add))
                ).filter(|(x, y)| {
                    0 <= *x && *x < x_size as i64 && 0 <= *y && *y < y_size as i64
                }).filter(|(x, y)| {
                    grid[*y as usize][*x as usize]
                }).count() < 4 {
                if remove {
                    grid[y as usize][x as usize] = false;
                }
                removed += 1;
            }
        }
    }
    removed
}