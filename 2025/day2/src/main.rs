use std::error::Error;
use std::ffi::OsString;
use std::{env, fs};
use std::collections::HashSet;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;

    while n > 0 {
        n /= 10;
        count += 1;
    }

    count
}

fn split_by_digits(n: u64, digits: u32, factor: u32) -> (u64, u64) {
    let base = 10u64.pow(digits - digits/factor);
    (n / base, n % base)
}

// returns the sum of the invalid IDs
fn check_range(start: &str, end: &str, factor: u32, results: &mut HashSet<u64>) -> Result<u64, Box<dyn Error>> {
    let mut sum = 0;

    let min = start.parse::<u64>()?;

    let max = end.parse::<u64>()?;

    //println!("Range: {} - {}: {}", min, max, factor);

    let mut current = min;
    let mut cur_digits;

    while current < max{
        cur_digits = count_digits(current);
        //digits must be even
        if (cur_digits % factor) != 0 {
            current = 10u64.pow(cur_digits as u32);
            continue;
        }

        let (pattern, _rest) = split_by_digits(current, cur_digits, factor);
        //println!("Current {}, Left {}, Right {}", current, pattern, rest);
        let base = 10u64.pow(cur_digits - cur_digits/factor);

        let mut invalid = 0;
        for i in 1..=factor {
            invalid += pattern * 10u64.pow((i - 1) * cur_digits/factor );
        }

        if (invalid >= min) && (invalid <= max) {
            sum += invalid as u64;
            results.insert(invalid);
            //println!("Invalid: {}", invalid);
        }

        current = (pattern) * base + base;        
    }

    return Ok(sum);
}

// returns the sum of the invalid IDs
fn check_all_factors(start: &str, end: &str, results: &mut HashSet<u64>) -> Result<(), Box<dyn Error>> {
    let max_cnt = end.chars().count();

    for i in 2..=max_cnt {
        let _ = check_range(start, end, i as u32, results)?;
    }

    return Ok(());
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let input = fs::read_to_string(file_path)?;
    let mut list = Vec::new();

    for range in input.split(',') {
        list.push(range);
    }

    let ranges: Vec<(&str, &str)> = list
        .iter()
        .filter_map(|s| {
            let mut parts = s.split('-');
            Some((parts.next()?, parts.next()?))
        })
        .collect();

    //println!("{:?}", ranges);

    let mut results: HashSet<u64> = HashSet::new();

    let sum_invalid: u64 = ranges
        .iter()
        .filter_map(|(start, end)| check_range(start, end, 2, &mut results).ok())
        .sum();

    println!("Part1: {}", sum_invalid);

    let mut results: HashSet<u64> = HashSet::new();
    let _ = ranges
        .iter()
        .try_for_each(|(start, end)| check_all_factors(start, end, &mut results).ok());

    let sum: u64 = results.iter().sum();
    println!("Part2: {}", sum);

    Ok(())
}
