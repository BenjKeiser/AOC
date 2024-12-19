use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::time::Instant;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn get_splits(pattern: &str, towels: &[&str]) -> Option<(bool, Vec<(String, String)>)> {
    let mut splits: Vec<(String, String)> = Vec::new();

    //println!("get_splits {pattern}");
    for t in towels {
        if let Some((lhs, rhs)) = pattern.split_once(*t) {
            //println!("{} => {} & {}", *t, lhs, rhs);
            if lhs.len() == 0 && rhs.len() == 0 {
                return Some((true, splits));
            } else {
                splits.push((lhs.to_string(), rhs.to_string()));
            }
        }
    }

    if splits.len() == 0 {
        return None;
    }

    Some((false, splits))
}

fn is_possible(pattern: &str, towels: &[&str]) -> bool {
    if let Some((possible, next)) = get_splits(pattern, towels) {
        if possible {
            return true;
        }
        else {
            for (lhs, rhs) in next {
                if (lhs.len() == 0 || is_possible(&lhs, towels)) && (rhs.len() == 0 || is_possible(&rhs, towels)) {
                    return true;
                }
            }            
        }
    }
    false
}

fn get_possible_patterns(towels: &[&str], patterns: &[&str]) -> usize {
    let mut possible = 0;
    for p in patterns {
        if is_possible(*p, towels) {
            possible += 1;
        }
    }

    possible
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut towels: Vec<&str> = Vec::new();
    let mut patterns: Vec<&str> = Vec::new();

    let file = fs::read_to_string(file_path)?;

    let mut lines = file.lines();

    for towel in lines.next().unwrap().split(',') {
        towels.push(towel.trim());
    }
    for line in lines.skip(1) {
        patterns.push(line);
    }

    let start = Instant::now();
    let patterns = get_possible_patterns(&towels, &patterns);
    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        patterns,
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
