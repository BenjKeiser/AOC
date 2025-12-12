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

fn get_area(presents: &Vec<Vec<Vec<char>>>, amount: &Vec<u32>) -> u32 {
    let mut area = 0;
    for (idx, nb) in amount.iter().enumerate() {
        let a = presents[idx]
            .iter()
            .flatten()
            .filter(|&&c| c == '#')
            .count() as u32;
        area += *nb * a;
    }
    area
}

fn get_fits(presents: &Vec<Vec<Vec<char>>>, areas: &Vec<((u32, u32), Vec<u32>)>) -> u32 {
    //get all elements where the bounding box fits within the area
    let n: Vec<&((u32, u32), Vec<u32>)> = areas
        .iter()
        .filter(|((x, y), n)| (*x * (*y)) > get_area(presents, n))
        .collect();

    n.len() as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let start = Instant::now();

    let mut presents: Vec<Vec<Vec<char>>> = Vec::new();
    let mut present: Vec<Vec<char>> = Vec::new();

    let mut areas: Vec<((u32, u32), Vec<u32>)> = Vec::new();

    for line in fs::read_to_string(file_path)?.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        if line.chars().nth(0).unwrap() == '.' || line.chars().nth(0).unwrap() == '#' {
            present.push(line.chars().collect());
        } else if let Some(x) = line.split(':').next() {
            if present.len() > 0 {
                presents.push(present);
                present = Vec::new();
            }
            if !x.parse::<u32>().is_ok() {
                let mut s = line.split(':');
                if let Some(dim) = s.next() {
                    let mut d = dim.split('x');
                    if let (Some(x), Some(y)) = (d.next(), d.next()) {
                        let mut v = Vec::new();
                        if let Some(p) = s.next() {
                            for n in p.split(' ').skip(1) {
                                v.push(n.parse::<u32>()?);
                            }
                        }
                        areas.push(((x.parse::<u32>()?, y.parse::<u32>()?), v));
                    }
                }
            }
        }
    }

    //println!("{:?}", presents);
    //println!("{:?}", areas);

    let duration = start.elapsed();

    println!(
        "Parse: {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let fit = get_fits(&presents, &areas);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        fit,
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
