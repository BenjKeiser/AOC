use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::ops::Add;
use std::ops::Mul;
use std::time::Instant;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,
}

impl Add for Point {
    type Output = Point; // Define the result type of the addition

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<usize> for Point {
    type Output = Point; // Define the result type of the addition

    fn mul(self, scalar: usize) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Point {}

impl Grid {
    fn print_grid(&self) {
        
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut moves: Vec<char> = Vec::new();
    let mut warehouse: Grid = Grid {grid: Vec::new()};

    for line in fs::read_to_string(file_path)?.lines() {
        if line.len() > 0 {
            let mut cs: Vec<_> = line.chars().collect();
            if cs[0] == '#' {
                warehouse.grid.push(cs);
            } else {
                moves.append(&mut cs);
            }
        }
    }

    println!("{:?}", warehouse);

    println!("{:?}", moves);

    let start = Instant::now();

    let duration = start.elapsed();
    println!("Part1: {} | {}s", 1, duration.as_secs_f32());

    let start = Instant::now();

    let duration = start.elapsed();
    println!("Part2: {} | {}s", 2, duration.as_secs_f32());

    Ok(())
}
