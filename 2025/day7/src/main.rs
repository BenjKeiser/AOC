use grid::{Direction, Grid, Point};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::time::Instant;
use std::{env, fs};

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn propagate_vertical(manifold: &Grid, start: Point) -> u32 {
    let mut splits = 0;

    let mut visited: Vec<Vec<bool>> = vec![vec![false; manifold[0].len()]; manifold.len()];

    let mut queue: Vec<Point> = Vec::new();

    let mut man = manifold.clone();

    let dir = Direction { x: 0, y: 1 };
    let dir_left = Direction { x: -1, y: 0 };
    let dir_right = Direction { x: 1, y: 0 };

    queue.push(start);

    while !queue.is_empty() {
        if let Some(p) = queue.pop() {
            if visited[p.y][p.x] {
                continue;
            }

            visited[p.y][p.x] = true;

            match man[p.y][p.x] {
                'S' => {
                    if man.is_move_valid(&p, &dir) {
                        queue.push((p + dir).unwrap())
                    }
                }
                '|' => {
                    println!("not yet visited? {}", p)
                }
                '^' => {
                    splits += 1;
                    if man.is_move_valid(&p, &dir_left) {
                        queue.push((p + dir_left).unwrap())
                    }
                    if man.is_move_valid(&p, &dir_right) {
                        queue.push((p + dir_right).unwrap())
                    }
                }
                '.' => {
                    man[p.y][p.x] = '|';
                    if man.is_move_valid(&p, &dir) {
                        queue.push((p + dir).unwrap())
                    }
                }
                _ => {
                    println!("unexpected");
                }
            }
        }
    }

    //println!("{}", man);

    splits
}

fn propagate_quantum(manifold: &Grid, start: Point, memory: &mut HashMap<Point, u64>) -> u64 {
    let mut timelines = 0;

    let dir = Direction { x: 0, y: 1 };
    let dir_left = Direction { x: -1, y: 0 };
    let dir_right = Direction { x: 1, y: 0 };

    let mut p = start;

    loop {
        if p.y == manifold.len() - 1 {
            return 1;
        }

        match manifold[p.y][p.x] {
            'S' => {
                if manifold.is_move_valid(&p, &dir) {
                    p = (p + dir).unwrap();
                }
            }
            '^' => {
                if manifold.is_move_valid(&p, &dir_left) {
                    let left = (p + dir_left).unwrap();
                    if let Some(val) = memory.get(&left) {
                        timelines += *val;
                    } else {
                        let val = propagate_quantum(manifold, left, memory);
                        memory.insert(left, val);
                        timelines += val;
                    }
                }
                if manifold.is_move_valid(&p, &dir_right) {
                    let right = (p + dir_right).unwrap();
                    if let Some(val) = memory.get(&right) {
                        timelines += *val;
                    } else {
                        let val = propagate_quantum(manifold, right, memory);
                        memory.insert(right, val);
                        timelines += val;
                    }
                }
                break;
            }
            '.' => {
                if manifold.is_move_valid(&p, &dir) {
                    p = (p + dir).unwrap();
                }
            }
            _ => {
                println!("unexpected");
            }
        }
    }

    timelines
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let start = Instant::now();

    let mut manifold: Grid = Grid::new();

    let mut row = 0;

    let mut tach = Point { x: 0, y: 0 };

    for line in fs::read_to_string(file_path)?.lines() {
        manifold.push(line.chars().collect());

        if let Some(col) = line.find('S') {
            tach = Point { x: col, y: row };
        }

        row += 1;
    }

    //println!("{}", tach);
    //println!("{}", manifold);

    let duration = start.elapsed();

    println!(
        "Parse: {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let splits = propagate_vertical(&manifold, tach);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        splits,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let mut memory: HashMap<Point, u64> = HashMap::new();
    let timelines = propagate_quantum(&manifold, tach, &mut memory);

    let duration = start.elapsed();
    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        timelines,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
