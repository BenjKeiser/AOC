use std::error::Error;
use std::ffi::OsString;
use std::{env, fs};

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

fn split_by_digits(n: u64, digits: u32) -> (u64, u64) {
    let base = 10u64.pow(digits);
    (n / base, n % base)
}

// returns the sum of the invalid IDs
fn check_range(start: &str, end: &str) -> Result<u64, Box<dyn Error>> {
    let mut sum = 0;

    let min = start.parse::<u64>()?;

    let max = end.parse::<u64>()?;

    println!("Range: {} - {}", min, max);

    let mut current = min;
    let mut cur_digits;

    loop {
        cur_digits = count_digits(current);
        //digits must be even
        if (cur_digits % 2) != 0 {
            current = 10u64.pow(cur_digits as u32);
            continue;
        }

        let (left, right) = split_by_digits(current, cur_digits / 2);

        let invalid = left * 10u64.pow(cur_digits / 2) + left;
        println!("Check: {}", invalid);

        if (invalid >= min) && (invalid <= max) {
            sum += invalid as u64;
            println!("Invalid: {}", invalid);
        }

        current = (left) * 10u64.pow(cur_digits / 2) + 10u64.pow(cur_digits / 2);
        println!("Next: {}", current);
        if current > max {
            break;
        }
    }

    return Ok(sum);
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

    let sum_invalid: u64 = ranges
        .iter()
        .filter_map(|(start, end)| check_range(start, end).ok())
        .sum();

    println!("Part1: {}", sum_invalid);

    Ok(())
}
