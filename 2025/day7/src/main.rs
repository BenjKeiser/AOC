use std::error::Error;
use std::ffi::OsString;
use std::time::Instant;
use std::{env, fs};
use grid::{Grid, Point};

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    
    let start = Instant::now();

    let mut manifold: Grid = Grid::new();

    let mut row = 0;

    let mut tach = Point{x: 0, y: 0};

    for line in fs::read_to_string(file_path)?.lines() {
        manifold.push(line.chars().collect());

        if let Some(col) = line.find('S') {
            tach = Point{x: col, y: row};
        }

        row += 1;
    }

    println!("{}", tach);
    println!("{}", manifold);

    
    let duration = start.elapsed();

    println!(
        "Parse: {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();


    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        1,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();


    let duration = start.elapsed();
    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        2,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
