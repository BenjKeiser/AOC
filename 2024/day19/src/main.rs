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

fn match_pattern(pattern: &str, towel: &str) -> bool {
    if pattern.len() >= towel.len() {
        &pattern[..towel.len()] == towel
    } else {
        false
    }
}

fn match_towels(pattern: &str, towels: &[&str]) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    for t in towels {
        if match_pattern(pattern, *t) {
            if match_towels(&pattern[(*t).len()..], towels) {
                return true;
            }
        }
    }
    false
}

fn get_possible_patterns(towels: &[&str], patterns: &[&str]) -> usize {
    let mut possible = 0;
    for p in patterns {
        if match_towels(*p, towels) {
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
