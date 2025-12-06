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

fn starts_with_number(line: &str) -> bool {
    matches!(line.trim_start().as_bytes().first(), Some(b'0'..=b'9'))
}

fn do_math(numbers: &Vec<Vec<i64>>, operators: &Vec<char>) -> i64 {
    let mut sum = 0;
    for (col, op) in operators.iter().enumerate() {
        let mut result = numbers[0][col];
        for (_, nb) in numbers.iter().enumerate().skip(1) {
            match op {
                '+' => {
                    result += nb[col];
                }
                '*' => {
                    result *= nb[col];
                }
                _ => {
                    result = 0;
                }
            }
        }
        sum += result;
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines() {
        if starts_with_number(line) {
            let nb: Vec<i64> = line
                .split_whitespace()
                .map(|s| i64::from_str_radix(s, 10).unwrap())
                .collect();
            numbers.push(nb);
        } else {
            operators = line
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect();
        }
    }

    //println!("{:?}", numbers);
    //println!("{:?}", operators);

    if numbers[0].len() != operators.len() {
        return Err("MISMATCH".into());
    }

    let start = Instant::now();

    let sum = do_math(&numbers, &operators);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        sum,
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
