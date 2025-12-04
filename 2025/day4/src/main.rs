use grid::{Grid, Point};
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

fn get_reachable(paper: &Grid, remove: bool) -> u64 {
    let mut sum = 0;

    let mut paper = paper.clone();

    loop {
        let mut reachable = 0;
        for row in 0..paper.len() {
            for column in 0..paper[0].len() {
                if paper[row][column] == '@' {
                    let neighbours = paper.get_neighbours(&Point { x: column, y: row }, true);
                    let mut rolls = 0;
                    for n in neighbours {
                        if paper[n.y][n.x] == '@' || paper[n.y][n.x] == 'x' {
                            rolls += 1;
                        }
                    }
                    if rolls < 4 {
                        reachable += 1;
                        if remove {
                            paper[row][column] = 'x'
                        }
                    }
                }
            }
        }

        sum += reachable;
        if !remove || (reachable == 0) {
            break;
        }

        if remove {
            paper
                .iter_mut()
                .flat_map(|row| row.iter_mut())
                .for_each(|c| {
                    if *c == 'x' {
                        *c = '.';
                    }
                })
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut paper: Grid = Grid::new();

    for line in fs::read_to_string(file_path)?.lines() {
        if line.len() > 0 {
            let cs: Vec<_> = line.chars().collect();
            paper.push(cs);
        }
    }

    let start = Instant::now();

    let rolls = get_reachable(&paper, false);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        rolls,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let rolls = get_reachable(&paper, true);

    let duration = start.elapsed();
    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        rolls,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
