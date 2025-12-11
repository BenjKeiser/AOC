use std::collections::{HashMap, HashSet};
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

fn get_paths(target: &String, start: &String, devices: &HashMap<String, Vec<String>>) -> u64 {
    let visited: HashSet<String> = HashSet::new();

    let mut memo: HashMap<(String, String), u64> = HashMap::new();
    explore_paths(target, start, &visited, devices, &mut memo)
}

fn explore_paths(
    target: &String,
    node: &String,
    visited: &HashSet<String>,
    map: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, String), u64>,
) -> u64 {
    let mut paths = 0;
    if !visited.contains(node) {
        if *node == *target {
            return 1;
        }

        let mut v = visited.clone();
        v.insert(node.clone());

        // insert next
        if let Some(next) = map.get(node) {
            for n in next {
                if let Some(v) = memo.get(&(n.clone(), target.clone())) {
                    paths += *v;
                } else {
                    let p = explore_paths(target, n, &v, map, memo);
                    memo.insert((n.clone(), target.clone()), p);
                    paths += p;
                }
            }
        }
    }

    return paths;
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let start = Instant::now();

    let mut devices: HashMap<String, Vec<String>> = HashMap::new();

    for line in fs::read_to_string(file_path)?.lines() {
        let mut splits = line.split(':');
        let mut outs = Vec::new();
        if let (Some(dev), Some(outputs)) = (splits.next(), splits.next()) {
            for o in outputs.split(' ').skip(1) {
                outs.push(o.to_string());
            }
            devices.insert(dev.to_string(), outs);
        }
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

    let paths = get_paths(&"out".to_string(), &"you".to_string(), &devices);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        paths,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();
    
    let mut paths1 = get_paths(&"dac".to_string(), &"svr".to_string(), &devices);
    paths1 *= get_paths(&"fft".to_string(), &"dac".to_string(), &devices);
    paths1 *= get_paths(&"out".to_string(), &"fft".to_string(), &devices);

    
    let mut paths2 = get_paths(&"fft".to_string(), &"svr".to_string(), &devices);
    paths2 *= get_paths(&"dac".to_string(), &"fft".to_string(), &devices);
    paths2 *= get_paths(&"out".to_string(), &"dac".to_string(), &devices);

    let paths = paths1 + paths2;

    let duration = start.elapsed();
    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        paths,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
