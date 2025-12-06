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
        for nb in numbers.iter().skip(1) {
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

fn do_ceph_math(numbers: &Vec<Vec<i64>>, operators: &Vec<char>) -> i64 {
    let mut sum = 0;
    for (idx, op) in operators.iter().enumerate() {
        let mut result = numbers[idx][0];
        for nb in numbers[idx].iter().skip(1) {
            match op {
                '+' => {
                    result += *nb;
                }
                '*' => {
                    result *= *nb;
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

    let mut input = Vec::new();
    let mut ceph_numbers = Vec::new();
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    
    let start = Instant::now();

    for line in fs::read_to_string(file_path)?.lines() {
        if starts_with_number(line) {
            let nb: Vec<i64> = line
                .split_whitespace()
                .map(|s| i64::from_str_radix(s, 10).unwrap())
                .collect();
            numbers.push(nb);
            input.push(line);
        } else {
            operators = line
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect();

            //parse input for part 2
            let mut last = 0;
            let mut ops: String = line.to_string();
            ops.push('e');
            for (p, _c) in ops.chars().skip(1).enumerate().filter(|(_p, c)| *c != ' ') {
                let mut cn = Vec::new();
                for idx in last..=p {
                    let mut val = String::new();
                    for row in 0..input.len() {
                        let c = input[row].chars().nth(idx).unwrap();
                        if c != ' ' {
                            val.push(c);
                        }
                    }
                    if starts_with_number(&val) {
                        cn.push(val.parse::<i64>()?);
                    }
                }
                ceph_numbers.push(cn);
                last = p
            }
        }
    }

    if numbers[0].len() != operators.len() {
        return Err("MISMATCH".into());
    }

    
    let duration = start.elapsed();

    println!(
        "Parse: {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

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

    let sum = do_ceph_math(&ceph_numbers, &operators);

    let duration = start.elapsed();
    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        sum,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
