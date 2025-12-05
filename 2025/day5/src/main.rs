use std::error::Error;
use std::ffi::OsString;
use std::ops::RangeInclusive;
use std::time::Instant;
use std::{env, fs};

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn get_spoiled(ranges: &Vec<RangeInclusive<u64>>, ids: &Vec<u64>) -> u64 {
    let spoiled: Vec<_> = ids.iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .collect();

    spoiled.len() as u64
}

fn combine_ranges_inclusive(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return Vec::new();
    }

    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    let mut current = ranges.remove(0);

    for r in ranges {
        if *r.start() <= *current.end() + 1 {
            let new_end = (*current.end()).max(*r.end());
            current = *current.start()..=new_end;
        } else {
            merged.push(current);
            current = r;
        }
    }

    merged.push(current);
    merged
}

fn get_fresh(ranges: &Vec<RangeInclusive<u64>>) -> u64{
    if ranges.is_empty() {
        return 0;
    }
    let mut ranges_combined: Vec<RangeInclusive<u64>> = ranges.clone();

    ranges_combined = combine_ranges_inclusive(ranges_combined);

    ranges_combined.iter().map(|r| r.end() - r.start() + 1).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut r: bool = true;
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines() {
        if line.len() == 0 {
            //we read all ranges
            r = false;
            continue;
        }

        if r {
            let mut range_str = line.split('-');
            let range = range_str.next().unwrap().parse::<u64>()?
                ..=range_str.next().unwrap().parse::<u64>()?;

            ranges.push(range);
        } else {
            ids.push(line.parse::<u64>()?);
        }
    }

    //println!("{:?}", ranges);
    //println!("{:?}", ids);

    let start = Instant::now();

    let spoiled = get_spoiled(&ranges, &ids);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        spoiled,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let fresh = get_fresh(&ranges);

    let duration = start.elapsed();
    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        fresh,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
