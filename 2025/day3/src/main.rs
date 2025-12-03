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

fn get_joltage(bank: &str) -> Result<u32, Box<dyn Error>> {
    if let Some((pos, first)) = bank
        .chars()
        .enumerate()
        .take(bank.chars().count() - 1)
        .max_by(|(p1, c1), (p2, c2)| c1.cmp(c2).then(p2.cmp(p1)))
    {
        if let Some(rest) = bank.get(pos + 1..) {
            if let Some(second) = rest.chars().max_by_key(|&c| c) {
                let s = format!("{}{}", first, second);
                let joltage = s.parse::<u32>()?;
                //println!("Bank: {}, joltage {}, rest {}", bank, joltage, rest);
                return Ok(joltage);
            }
        }
    }
    Err("No valid battery found".into())
}

fn get_joltage_recursive(bank: &str, depth: usize) -> Option<u64> {
    if bank.chars().count() <= depth {
        return None;
    }

    if let Some(max) = bank
        .chars()
        .take(bank.chars().count() - depth)
        .max_by_key(|&c| c)
    {
        if depth == 0 {
            if let Some(val) = max.to_digit(10) {
                return Some(val as u64);
            }
        }

        let mut joltage = 0;
        for (p, _) in bank.chars().enumerate().filter(|&(_, c)| c == max) {
            if (bank.chars().count() - p) >= (depth - 1) {
                if let Some(rest) = bank.get(p + 1..) {
                    if let Some(j) = get_joltage_recursive(rest, depth - 1) {
                        if j > joltage {
                            joltage = j;
                        }
                    }
                }
            }
        }
        if joltage > 0 {
            let s = format!("{}{}", max, joltage);
            joltage = s.parse::<u64>().unwrap();
            return Some(joltage);
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let input = fs::read_to_string(file_path)?;

    let mut banks = Vec::new();
    for l in input.lines() {
        banks.push(l);
    }

    let start = Instant::now();

    let total_joltage: u32 = banks.iter().filter_map(|b| get_joltage(b).ok()).sum();

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        total_joltage,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let total_joltage: u64 = banks
        .iter()
        .filter_map(|b| get_joltage_recursive(b, 12 - 1))
        .sum();

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        total_joltage,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
