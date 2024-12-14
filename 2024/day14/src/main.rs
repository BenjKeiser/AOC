use regex::Regex;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::ops::Add;
use std::time::Instant;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Velocity {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let input = fs::read_to_string(file_path)?;

    // Regex to capture integers after X or Y markers
    let re = Regex::new(r"p=([+-]?\d+),([+-]?\d+) v=([+-]?\d+),([+-]?\d+)").unwrap();

    // Collect all groups
    let robots: Vec<(Point, Velocity)> = re
        .captures_iter(&input) // Iterate over all matches
        .filter_map(|cap| {
            Some((
                Point {
                    x: cap[1].parse::<i32>().ok()?, // p.x
                    y: cap[2].parse::<i32>().ok()?,
                }, // p.y
                Velocity {
                    x: cap[3].parse::<i32>().ok()?, // v.x
                    y: cap[4].parse::<i32>().ok()?,
                }, // v.y
            ))
        })
        .collect();

    println!("{:?}", robots);

    //println!("{:?}", machines);

    let start = Instant::now();

    let duration = start.elapsed();
    println!("Part1: {} | {}s", 2, duration.as_secs_f32());

    let start = Instant::now();

    let duration = start.elapsed();
    println!("Part2: {} | {}s", 1, duration.as_secs_f32());

    Ok(())
}
