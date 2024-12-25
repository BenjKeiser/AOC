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

fn get_matches(locks: &Vec<[usize; 5]>, keys: &Vec<[usize; 5]>) -> usize {
    let mut matches = 0;

    locks.iter().for_each(|l| {
        keys.iter().for_each(|k| {
            let mut m = true;
            for i in 0..k.len() {
                if l[i] + k[i] > 5 {
                    m = false;
                    break;
                }
            }
            if m {
                matches += 1;
            }
        })
    });

    matches
}

fn get_locks_and_keys(locks_and_keys: &Vec<Vec<Vec<char>>>) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for el in locks_and_keys {
        if el[0] == ['#', '#', '#', '#', '#'] {
            //lock
            let mut lock: [usize; 5] = [0, 0, 0, 0, 0];
            for x in 0..el[0].len() {
                for y in 1..el.len() {
                    if el[y][x] == '#' {
                        lock[x] += 1;
                    } else {
                        break;
                    }
                }
            }
            locks.push(lock);
        } else {
            //key
            let mut key: [usize; 5] = [0, 0, 0, 0, 0];
            for x in 0..el[0].len() {
                for y in (0..el.len()-1).rev() {
                    if el[y][x] == '#' {
                        key[x] += 1;
                    } else {
                        break;
                    }
                }
            }
            keys.push(key);
        }
    }

    (locks, keys)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut locks_n_keys = Vec::new();
    let mut lock_or_key = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        if line.is_empty() {
            locks_n_keys.push(lock_or_key);
            lock_or_key = Vec::new();
        } else {
            let chars: Vec<_> = line.chars().collect();
            lock_or_key.push(chars);
        }
    }
    locks_n_keys.push(lock_or_key);

    let start = Instant::now();

    let (locks, keys) = get_locks_and_keys(&locks_n_keys);

    let matches = get_matches(&locks, &keys);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        matches,
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
